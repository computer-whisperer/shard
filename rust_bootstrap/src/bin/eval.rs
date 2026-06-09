//! `eval` — the clean narrow-interpreter passthrough.
//!
//! The Rust side is *only* a host: it bootstraps the kernel + shard toolchain
//! + the kernel's executable entrypoint (`kernel/eval.shard`) into the VM,
//! installs the World file-I/O extern handlers, and runs `(main world0)`.
//! Everything else — reading the referenced `.shard` files (and, in time,
//! their imports / stdlib), parsing, reducing, rendering — happens IN SHARD,
//! inside `eval.shard`, via the externs. No eval logic lives here.
//!
//!   eval <module.shard> <expr>
//!
//! This replaced the overloaded `check` binary's bespoke orchestration; the
//! proof-check entrypoint is now itself a shard app (`kernel/check.shard`)
//! run on top of this executor, and check.rs is deleted.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use lexpr::Value;
use lexpr::parse::Parser;
use num_traits::ToPrimitive;
use proving_bootstrap_v2::{ast, default_kernel_dir, eval, load};

/// Resolve the entrypoint's import closure: each `(import "X")` is followed
/// relative to the importing file's directory, post-order (deps before
/// dependents), deduped by canonical path, with the entrypoint itself last.
/// The kernel files declare no imports today, so this is one level deep; it
/// stays correct as they gain their own. This is host-side bootstrap (the VM
/// doesn't exist yet) — the analogue of a linker resolving a binary's deps.
fn resolve_closure(entry: &Path) -> Result<Vec<PathBuf>, String> {
    fn import_of(form: &Value) -> Option<String> {
        let items: Vec<&Value> = form.list_iter()?.collect();
        if items.len() != 2 {
            return None;
        }
        match items[0].as_symbol()? {
            "import" | "use-module" => items[1].as_str().map(|s| s.to_string()),
            _ => None,
        }
    }
    fn visit(p: &Path, order: &mut Vec<PathBuf>, seen: &mut HashSet<PathBuf>) -> Result<(), String> {
        let canon = p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
        if !seen.insert(canon) {
            return Ok(());
        }
        let src = std::fs::read_to_string(p).map_err(|e| format!("reading {}: {}", p.display(), e))?;
        let mut parser = Parser::from_str(&src);
        let dir = p.parent();
        loop {
            match parser.next_value() {
                Ok(Some(form)) => {
                    if let Some(dep) = import_of(&form) {
                        let rp = match dir {
                            Some(d) => d.join(&dep),
                            None => PathBuf::from(&dep),
                        };
                        visit(&rp, order, seen)?;
                    }
                }
                Ok(None) => break,
                Err(e) => return Err(format!("parsing {}: {}", p.display(), e)),
            }
        }
        order.push(p.to_path_buf());
        Ok(())
    }
    let mut order = Vec::new();
    let mut seen = HashSet::new();
    visit(entry, &mut order, &mut seen)?;
    Ok(order)
}

fn main() -> ExitCode {
    // The bootstrap interpreter (eval.rs) and the shard reader it hosts are
    // recursive tree-walkers; folding a large closure (e.g. an app whose
    // imports pull in the whole kernel, like check.shard) recurses deep enough
    // to blow the default 8 MiB main-thread stack. Run on a thread with a big
    // stack. This is purely a host concern — once compiled, the depth is bounded
    // by the program, not the interpreter. (The reserve is virtual memory; only
    // touched pages commit. The parse path's once-O(input-bytes) recursion is
    // fixed — the kernel's list-builders are accumulator-style now — so depth
    // scales with STRUCTURE (nesting, single-literal size), not file size.
    // SHARD_STACK_MIB overrides the reserve — used to probe depth ceilings.)
    let stack = std::env::var("SHARD_STACK_MIB")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .map(|mib| mib << 20)
        .unwrap_or(4 << 30); // default 4 GiB
    std::thread::Builder::new()
        .stack_size(stack)
        .spawn(run)
        .expect("spawn eval thread")
        .join()
        .unwrap_or(ExitCode::from(2))
}

fn run() -> ExitCode {
    let mut prog_args: Vec<String> = std::env::args().skip(1).collect();

    // --- bootstrap the VM: resolve the entrypoint's import closure ----------
    // `kernel/eval.shard` declares the files it needs; we follow its imports
    // (deps first, entrypoint last) and load exactly that closure — no proof
    // checker, no proof toolchain, no std/. The list lives in the entrypoint,
    // not here.
    //
    // `eval direct <app.shard> [args…]` instead loads the APP's closure and
    // runs its `main` on this bootstrap evaluator — one interpretation layer
    // instead of two (the tower `eval run` pays ~100× to interpret the app
    // through eval.shard's `ev`). Host-side verb only: eval.shard never sees
    // it. The app must be narrow (load.rs is the parser) with a FLAT import
    // closure (this host resolver does not understand directory modules —
    // that logic lives in loader.shard and stays there; check.shard
    // qualifies, shardfmt does not). Semantics are identical — the tower
    // remains the self-hosting cross-check.
    let entry = if prog_args.first().map(String::as_str) == Some("direct") {
        if prog_args.len() < 2 {
            eprintln!("usage: eval direct <app.shard> [args…]");
            return ExitCode::from(2);
        }
        let app = PathBuf::from(prog_args[1].clone());
        prog_args.drain(..2);
        app
    } else {
        default_kernel_dir().join("eval.shard")
    };
    let paths = match resolve_closure(&entry) {
        Ok(ps) => ps,
        Err(e) => {
            eprintln!("error resolving entrypoint closure: {}", e);
            return ExitCode::from(2);
        }
    };
    let vm = match load::module_from_paths_with_base(&paths, None) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading entrypoint closure: {}", e);
            return ExitCode::from(2);
        }
    };

    // --- run `(main (World 0))` under the file-I/O handler ------------------
    eval::set_effect_handler(Some(Box::new(make_handler(prog_args))));
    let call = ast::Expr::Call("main".into(), vec![world(0.into())]);
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
                ast::Expr::IntLit(k) => k.clone(),
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
            // persist bytes at a path — the dual of read_file. (Pair True World)
            // on success, (Pair False World) on any I/O error. Backs tools that
            // emit artifacts (tools/prove writes proof sidecars).
            "write_file" => {
                let ok =
                    std::fs::write(decode_str(&args[0]), decode_str(&args[1])).is_ok();
                Ok(ctor(
                    "Pair",
                    vec![ctor(if ok { "True" } else { "False" }, vec![]), w1],
                ))
            }
            // Blocking line-read from stdin: first byte as (Some b). A bare 'q',
            // an empty line typed as "q", or EOF ends the session as (None).
            // Returns (Pair (Option Int) World). Line-buffered (REPL-style): the
            // player types a key and presses Enter; any non-w/a/s/d byte is an
            // inert "advance in the current heading" tick.
            "read_key" => {
                let _ = std::io::stdout().flush();
                let mut line = String::new();
                let opt = match std::io::stdin().read_line(&mut line) {
                    Ok(0) => ctor("None", vec![]), // EOF
                    Ok(_) => match line.bytes().next() {
                        Some(b'q') => ctor("None", vec![]), // quit key
                        Some(b) => ctor("Some", vec![ast::Expr::IntLit(b.into())]),
                        None => ctor("None", vec![]),
                    },
                    Err(_) => ctor("None", vec![]),
                };
                Ok(ctor("Pair", vec![opt, w1]))
            }
            "write_line" | "emit" => {
                println!("{}", decode_str(&args[0]));
                Ok(w1)
            }
            "exit" => {
                let _ = std::io::stdout().flush();
                eval::prof_dump(); // no-op unless SHARD_PROF is set
                let code = match &args[0] {
                    ast::Expr::IntLit(n) => n.to_i32().unwrap_or(2),
                    _ => 0,
                };
                std::process::exit(code);
            }
            other => Err(format!(
                "unknown extern `{}` (eval handler: get_args/read_file/write/write_file/write_line/emit/read_key/exit)",
                other
            )),
        }
    }
}

// --- small value helpers (build/decode narrow Exprs) -----------------------

fn world(clk: ast::IntLit) -> ast::Expr {
    ctor("World", vec![ast::Expr::IntLit(clk)])
}

fn ctor(name: &str, args: Vec<ast::Expr>) -> ast::Expr {
    ast::Expr::Ctor(name.into(), args)
}

/// A Rust string as the narrow `(List Int)` of its char codepoints.
fn str_bytes(s: &str) -> ast::Expr {
    let mut acc = ctor("Nil", vec![]);
    for ch in s.chars().rev() {
        acc = ctor("Cons", vec![ast::Expr::IntLit((ch as u32).into()), acc]);
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
                if let Some(ch) = c.to_u32().and_then(char::from_u32) {
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
