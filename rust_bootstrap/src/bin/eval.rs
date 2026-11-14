//! `eval` — the bootstrap's executable.
//!
//! The Rust side is *only* a host: it loads a program's import closure
//! (load.rs), installs the handler of the host calls, and runs
//! `(main (World 0))` on the evaluator (eval.rs).
//!
//!   eval direct <app.shard> [args…]   load the app's closure, run its `main`
//!   eval dump <file.shard>            the closure as `direct` loads it, printed
//!                                     as the canonical text (dump.rs)

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use lexpr::Value;
use lexpr::parse::Parser;
use num_traits::ToPrimitive;
use shard_bootstrap::{ast, eval, load};

/// Resolve the entrypoint's import closure: each `(import "X")` is followed
/// relative to the importing file's directory, post-order (deps before
/// dependents), deduped by canonical path, with the entrypoint itself last.
/// A directory module is its view — `DIR/mod.req.shard` or
/// `DIR/mod.req/mod.req.shard`, whose `type`s are the module's own — and
/// then its implementation `DIR/BASE.shard`, each with its own closure; the
/// reader skips the view's `sig` forms, so a consumer's call resolves flat
/// to the implementation's `fn` of the same name (the V3 loader makes that
/// substitution by identity; docs/LANGUAGE.md "Views"). `(import Init
/// NAME)` names no file and is not followed.
fn resolve_closure(entry: &Path) -> Result<Vec<PathBuf>, String> {
    fn import_of(form: &Value) -> Option<String> {
        let items: Vec<&Value> = form.list_iter()?.collect();
        if items.len() != 2 {
            return None;
        }
        match items[0].as_symbol()? {
            "import" => items[1].as_str().map(|s| s.to_string()),
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
                        if rp.is_dir() {
                            let base = rp
                                .file_name()
                                .and_then(|b| b.to_str())
                                .ok_or_else(|| format!("directory import {}: no base name", rp.display()))?
                                .to_string();
                            let view = rp.join("mod.req.shard");
                            let view = if view.is_file() { view } else { rp.join("mod.req").join("mod.req.shard") };
                            if view.is_file() {
                                visit(&view, order, seen)?;
                            }
                            visit(&rp.join(format!("{base}.shard")), order, seen)?;
                        } else {
                            visit(&rp, order, seen)?;
                        }
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
    // The evaluator (eval.rs) recurses on every sub-evaluation that is not in
    // tail position, and the programs it runs — the reader, the checker — are
    // recursive tree walkers: a large closure goes deeper than the default
    // 8 MiB main-thread stack. Run on a thread with a big stack. The reserve is
    // virtual memory; only touched pages commit. SHARD_STACK_MIB overrides it
    // (to probe how deep a run goes).
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

    // `eval dump <file.shard>`: the file's closure loaded exactly as `direct`
    // loads it and printed as the canonical text (dump.rs) — nothing is run.
    if prog_args.first().map(String::as_str) == Some("dump") {
        if prog_args.len() != 2 {
            eprintln!("usage: eval dump <file.shard>");
            return ExitCode::from(2);
        }
        let paths = match resolve_closure(Path::new(&prog_args[1])) {
            Ok(ps) => ps,
            Err(e) => {
                eprintln!("error resolving entrypoint closure: {}", e);
                return ExitCode::from(2);
            }
        };
        let vm = match load::module_from_paths(&paths) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("error loading entrypoint closure: {}", e);
                return ExitCode::from(2);
            }
        };
        print!("{}", shard_bootstrap::dump::dump(&vm));
        return ExitCode::SUCCESS;
    }
    // `eval direct <app.shard> [args…]`: the app's closure (`resolve_closure`
    // above) loaded and its `main` run on this evaluator.
    let entry = if prog_args.first().map(String::as_str) == Some("direct") {
        if prog_args.len() < 2 {
            eprintln!("usage: eval direct <app.shard> [args…]");
            return ExitCode::from(2);
        }
        let app = PathBuf::from(prog_args[1].clone());
        prog_args.drain(..2);
        app
    } else {
        eprintln!("usage: eval direct <app.shard> [args…] | eval dump <file.shard>");
        return ExitCode::from(2);
    };
    let paths = match resolve_closure(&entry) {
        Ok(ps) => ps,
        Err(e) => {
            eprintln!("error resolving entrypoint closure: {}", e);
            return ExitCode::from(2);
        }
    };
    let vm = match load::module_from_paths(&paths) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading entrypoint closure: {}", e);
            return ExitCode::from(2);
        }
    };

    // --- run `(main (World 0))` under the handler of the host calls ---------
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

/// The handler of the host calls: it performs the I/O of each call to one of
/// the seven externs v3/kernel/host.shard declares — get_args, read_file,
/// read_range, write, write_line, write_file, exit. World = (World clock); it
/// is each extern's LAST argument and is returned with the clock advanced by
/// one.
///
/// The extern boundary speaks RAW BYTES: every `(List Int)` payload is a byte
/// sequence (each element 0..256), not Unicode codepoints. The host performs
/// no encoding or decoding — reads are binary-safe, writes emit the list
/// verbatim (elements masked mod 256).
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
            // the CLI arguments (after the binary name), as (List (List Int))
            // of their UTF-8 bytes.
            "get_args" => {
                let items: Vec<ast::Expr> = prog_args.iter().map(|a| byte_list(a.as_bytes())).collect();
                Ok(ctor("Pair", vec![list_of(items), w1]))
            }
            // file contents as (Some bytes), or None on any read error.
            // Binary-safe: the raw bytes, whatever they are — no UTF-8 gate.
            "read_file" => match std::fs::read(path_of_bytes(&args[0])) {
                Ok(contents) => Ok(ctor("Pair", vec![ctor("Some", vec![byte_list(&contents)]), w1])),
                Err(_) => Ok(ctor("Pair", vec![ctor("None", vec![]), w1])),
            },
            // a byte range of a file: at most n bytes from offset off, as
            // (Some bytes) — fewer at the file's end, an empty list past it —
            // or None on any read error or a negative offset or count
            // (v3/kernel/host.shard `read_range`).
            "read_range" => {
                let off = match &args[1] { ast::Expr::IntLit(n) => n.to_u64(), _ => None };
                let n = match &args[2] { ast::Expr::IntLit(n) => n.to_usize(), _ => None };
                let got = match (off, n) {
                    (Some(off), Some(n)) => {
                        use std::io::{Read as _, Seek as _};
                        std::fs::File::open(path_of_bytes(&args[0])).and_then(|mut f| {
                            f.seek(std::io::SeekFrom::Start(off))?;
                            let mut buf = Vec::new();
                            f.take(n as u64).read_to_end(&mut buf)?;
                            Ok(buf)
                        }).ok()
                    }
                    _ => None,
                };
                Ok(match got {
                    Some(bytes) => ctor("Pair", vec![ctor("Some", vec![byte_list(&bytes)]), w1]),
                    None => ctor("Pair", vec![ctor("None", vec![]), w1]),
                })
            }
            "write" => {
                let out = list_bytes(&args[0]);
                let mut stdout = std::io::stdout();
                let _ = stdout.write_all(&out);
                let _ = stdout.flush();
                Ok(w1)
            }
            // persist bytes at a path, verbatim — the dual of read_file.
            // (Pair True World) on success, (Pair False World) on any I/O
            // error. The toolchain writes its artifacts through it (the Init
            // index, a proof sidecar).
            "write_file" => {
                let ok = std::fs::write(path_of_bytes(&args[0]), list_bytes(&args[1])).is_ok();
                Ok(ctor(
                    "Pair",
                    vec![ctor(if ok { "True" } else { "False" }, vec![]), w1],
                ))
            }
            "write_line" => {
                let mut out = list_bytes(&args[0]);
                out.push(b'\n');
                let mut stdout = std::io::stdout();
                let _ = stdout.write_all(&out);
                let _ = stdout.flush();
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
                "unknown extern `{}` (eval handler: get_args/read_file/read_range/write/write_file/write_line/exit)",
                other
            )),
        }
    }
}

// --- small value helpers (build and decode `Expr` values) ------------------

fn world(clk: ast::IntLit) -> ast::Expr {
    ctor("World", vec![ast::Expr::IntLit(clk)])
}

fn ctor(name: &str, args: Vec<ast::Expr>) -> ast::Expr {
    ast::Expr::Ctor(name.into(), args)
}

/// A byte slice as the `(List Int)` of its bytes.
fn byte_list(bytes: &[u8]) -> ast::Expr {
    let mut acc = ctor("Nil", vec![]);
    for b in bytes.iter().rev() {
        acc = ctor("Cons", vec![ast::Expr::IntLit((*b).into()), acc]);
    }
    acc
}

/// A `(List Int)` back to raw bytes; elements are masked mod 256 (the
/// boundary never widens past a byte).
fn list_bytes(e: &ast::Expr) -> Vec<u8> {
    let mut out = Vec::new();
    let mut cur = e;
    while let ast::Expr::Ctor(n, a) = cur {
        if n == "Cons" && a.len() == 2 {
            if let ast::Expr::IntLit(c) = &a[0] {
                // two's-complement BitAnd == mod 256 for negatives as well
                out.push((c & &ast::IntLit::from(255)).to_u8().unwrap_or(0));
            }
            cur = &a[1];
        } else {
            break;
        }
    }
    out
}

/// A byte-list path as an OS path (raw bytes on unix — binary-safe).
fn path_of_bytes(e: &ast::Expr) -> std::path::PathBuf {
    use std::os::unix::ffi::OsStrExt as _;
    std::ffi::OsStr::from_bytes(&list_bytes(e)).into()
}

/// Build the list `(Cons x0 (Cons x1 … Nil))` from items.
fn list_of(items: Vec<ast::Expr>) -> ast::Expr {
    let mut acc = ctor("Nil", vec![]);
    for it in items.into_iter().rev() {
        acc = ctor("Cons", vec![it, acc]);
    }
    acc
}
