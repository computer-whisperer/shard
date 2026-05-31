//! `eval` — the clean narrow-interpreter passthrough.
//!
//! The Rust side is *only* a host: it bootstraps the kernel + shard toolchain
//! + the kernel's executable entrypoint (`kernel/kernel.shard`) into the VM,
//! installs the World file-I/O extern handlers, and runs `(main world0)`.
//! Everything else — reading the referenced `.shard` files (and, in time,
//! their imports / stdlib), parsing, reducing, rendering — happens IN SHARD,
//! inside `kernel.shard`, via the externs. No eval logic lives here.
//!
//!   eval <module.shard> <expr>
//!
//! This replaces the overloaded `check` binary's bespoke orchestration for the
//! eval path; `check` remains (deprecated) until the proof-check entrypoint is
//! likewise a shard app run on top of this executor.

use std::path::PathBuf;
use std::process::ExitCode;

use proving_bootstrap_v2::{ast, default_kernel_dir, eval, load, load_kernel_from};

// The shard toolchain + entrypoint, loaded after the kernel. `kernel.shard`
// (the `main` we run) comes last so it sees the reader/reducer it calls.
const TOOLCHAIN: &[&str] = &[
    "reader.shard",
    "unreflect.shard",
    "desugar.shard",
    "trace.shard",
    "driver.shard",
    "kernel.shard",
];

fn main() -> ExitCode {
    let prog_args: Vec<String> = std::env::args().skip(1).collect();

    // --- bootstrap the VM: kernel + toolchain + entrypoint ------------------
    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel: {}", e);
            return ExitCode::from(2);
        }
    };
    let kdir = default_kernel_dir();
    let paths: Vec<PathBuf> = TOOLCHAIN.iter().map(|f| kdir.join(f)).collect();
    // Resolve the toolchain + entrypoint against the kernel as base.
    let mut vm = match load::module_from_paths_with_base(&paths, Some(&kernel)) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading toolchain/entrypoint: {}", e);
            return ExitCode::from(2);
        }
    };
    // Append the kernel's defs as a FALLBACK (toolchain/entrypoint fns shadow
    // same-named kernel fns; a call the app doesn't define — e.g. compute_expr
    // — still resolves). Same discipline as the `check run` path.
    vm.types.extend(kernel.types.iter().cloned());
    vm.fns.extend(kernel.fns.iter().cloned());
    vm.externs.extend(kernel.externs.iter().cloned());

    // --- run `(main (World 0))` under the file-I/O handler ------------------
    eval::set_effect_handler(Some(Box::new(make_handler(prog_args))));
    let call = ast::Expr::Call("main".into(), vec![world(0)]);
    let result = eval::eval(&vm, &call);
    eval::set_effect_handler(None);
    match result {
        // The entrypoint normally terminates via the `exit` extern (which calls
        // process::exit). Reaching here means main returned a World directly.
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error running entrypoint: {:?}", e);
            ExitCode::from(2)
        }
    }
}

/// The World effect handler: performs the real I/O for each stuck extern call.
/// World = (World clock); it is each extern's LAST argument, returned bumped by
/// one (matching the `*_ticks` clock axioms). Covers the eval app's surface —
/// get_args / read_file / write / exit.
fn make_handler(prog_args: Vec<String>) -> impl FnMut(&str, &[ast::Expr]) -> Result<ast::Expr, String> {
    use std::io::Write as _;
    move |name: &str, args: &[ast::Expr]| -> Result<ast::Expr, String> {
        let clk = match args.last() {
            Some(ast::Expr::Ctor(n, a)) if n == "World" && a.len() == 1 => match &a[0] {
                ast::Expr::IntLit(k) => *k,
                other => return Err(format!("World clock is not an Int: {:?}", other)),
            },
            _ => return Err(format!("{}: expected a World as the last argument", name)),
        };
        let w1 = world(clk + 1);
        match name {
            // the CLI arguments (after the binary name), as (List (List Int)).
            "get_args" => {
                let items: Vec<ast::Expr> = prog_args.iter().map(|a| str_bytes(a)).collect();
                Ok(ctor("Pair", vec![list_of(items), w1]))
            }
            // file contents as (Some bytes), or None on any read error.
            "read_file" => match std::fs::read_to_string(decode_str(&args[0])) {
                Ok(contents) => Ok(ctor("Pair", vec![ctor("Some", vec![str_bytes(&contents)]), w1])),
                Err(_) => Ok(ctor("Pair", vec![ctor("None", vec![]), w1])),
            },
            "write" => {
                print!("{}", decode_str(&args[0]));
                let _ = std::io::stdout().flush();
                Ok(w1)
            }
            "write_line" | "emit" => {
                println!("{}", decode_str(&args[0]));
                Ok(w1)
            }
            "exit" => {
                let _ = std::io::stdout().flush();
                let code = match &args[0] {
                    ast::Expr::IntLit(n) => *n,
                    _ => 0,
                };
                std::process::exit(code as i32);
            }
            other => Err(format!(
                "unknown extern `{}` (eval handler: get_args/read_file/write/write_line/emit/exit)",
                other
            )),
        }
    }
}

// --- small value helpers (build/decode narrow Exprs) -----------------------

fn world(clk: i64) -> ast::Expr {
    ctor("World", vec![ast::Expr::IntLit(clk)])
}

fn ctor(name: &str, args: Vec<ast::Expr>) -> ast::Expr {
    ast::Expr::Ctor(name.into(), args)
}

/// A Rust string as the narrow `(List Int)` of its char codepoints.
fn str_bytes(s: &str) -> ast::Expr {
    let mut acc = ctor("Nil", vec![]);
    for ch in s.chars().rev() {
        acc = ctor("Cons", vec![ast::Expr::IntLit(ch as i64), acc]);
    }
    acc
}

/// A narrow `(List Int)` of codepoints back to a Rust string.
fn decode_str(e: &ast::Expr) -> String {
    let mut out = String::new();
    let mut cur = e;
    while let ast::Expr::Ctor(n, a) = cur {
        if n == "Cons" && a.len() == 2 {
            if let ast::Expr::IntLit(c) = &a[0] {
                if let Some(ch) = char::from_u32(*c as u32) {
                    out.push(ch);
                }
            }
            cur = &a[1];
        } else {
            break;
        }
    }
    out
}

/// Build the narrow list `(Cons x0 (Cons x1 … Nil))` from items.
fn list_of(items: Vec<ast::Expr>) -> ast::Expr {
    let mut acc = ctor("Nil", vec![]);
    for it in items.into_iter().rev() {
        acc = ctor("Cons", vec![it, acc]);
    }
    acc
}
