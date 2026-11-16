//! `eval` — the bootstrap's executable.
//!
//! The Rust side is *only* a host: it reads a program's closure
//! (plain/mod.rs), installs the handler of the host calls, and runs the root
//! module's `main` on the evaluator (eval.rs).
//!
//!   eval direct [--root DIR] [--facts FILE] <app.shard> [args…]
//!       read the app's closure, run its `main`
//!   eval dump [--root DIR] [--facts FILE] <file.shard>…
//!       the closure as `direct` reads it, printed as the canonical text
//!       (plain/out.rs); nothing is run
//!
//! DIR is the package root the files are modules under (docs/LANGUAGE.md "A
//! file is a module"); without `--root` it is `v3`, the root of the system's
//! own sources, as `v3/build.sh`'s is. FILE is the table of facts about
//! Init's constants the loader printed for the closure (`load.shard --facts
//! FILE`): a closure that sees Init is read by it. Where none is named the
//! root's own is read, `DIR/kernel/init.facts` — the toolchain's, kept in the
//! tree. A directory named among the files is its view; every view the
//! closure loads is linked to its implementation.

use std::path::PathBuf;
use std::process::ExitCode;

use num_traits::ToPrimitive;
use shard_bootstrap::{ast, eval, plain};

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

const USAGE: &str = "usage: eval direct [--root DIR] [--facts FILE] <app.shard> [args…] | eval dump [--root DIR] [--facts FILE] <file.shard>…";

/// The options before the first file: the root (`v3` where none is named) and the table
/// of Init's facts (the root's own, `kernel/init.facts`, where none is named and the root
/// has one); then what follows them.
fn options(args: &[String]) -> Option<(PathBuf, Option<PathBuf>, &[String])> {
    let (mut root, mut facts, mut i) = (PathBuf::from("v3"), None, 0);
    while i < args.len() && (args[i] == "--root" || args[i] == "--facts") {
        let v = PathBuf::from(args.get(i + 1)?);
        if args[i] == "--root" {
            root = v;
        } else {
            facts = Some(v);
        }
        i += 2;
    }
    if facts.is_none() {
        let own = root.join("kernel").join("init.facts");
        if own.is_file() {
            facts = Some(own);
        }
    }
    Some((root, facts, &args[i..]))
}

fn run() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str);
    let Some((root, facts, rest)) = (if matches!(mode, Some("direct" | "dump")) { options(&args[1..]) } else { None }) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    if rest.is_empty() {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
    // `dump`: every file after the options is a root; `direct`: the first is the app, the
    // rest are its arguments
    let (files, prog_args): (Vec<PathBuf>, Vec<String>) = if mode == Some("dump") {
        (rest.iter().map(PathBuf::from).collect(), Vec::new())
    } else {
        (vec![PathBuf::from(&rest[0])], rest[1..].to_vec())
    };
    let l = match plain::load(&root, facts.as_deref(), &files) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("error loading entrypoint closure: {}", e);
            return ExitCode::from(2);
        }
    };
    if mode == Some("dump") {
        // what has no reading is named on standard error, and the exit code says there
        // was some
        print!("{}", plain::out::dump(&l.t));
        for (n, u) in &l.t.unread {
            eprintln!("UNREAD {} {}: {}", n, u.0, u.1);
        }
        return if l.t.unread.is_empty() { ExitCode::SUCCESS } else { ExitCode::from(3) };
    }

    // --- run `(main (World 0))` under the handler of the host calls ---------
    let (entry, world_ctor) = match plain::out::entry(&l) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("error loading entrypoint closure: {}", e);
            return ExitCode::from(2);
        }
    };
    let vm = plain::out::module(&l.t);
    drop(l);
    eval::set_effect_handler(Some(Box::new(make_handler(prog_args))));
    let call = ast::Expr::Call(entry, vec![ctor(&world_ctor, vec![ast::Expr::IntLit(0.into())])]);
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

// The host speaks the runtime's cells — `Pair`, `Cons`, `Nil`, `Some`, `None`, `True`,
// `False` — which are Init's `Prod`, `List`, `Option` and `Bool` constructors and a file's
// own so named (plain/out.rs).

/// A byte slice as the list of its bytes.
fn byte_list(bytes: &[u8]) -> ast::Expr {
    let mut acc = ctor("Nil", vec![]);
    for b in bytes.iter().rev() {
        acc = ctor("Cons", vec![ast::Expr::IntLit((*b).into()), acc]);
    }
    acc
}

/// A list of bytes back to raw bytes; elements are masked mod 256 (the boundary never
/// widens past a byte).
fn list_bytes(e: &ast::Expr) -> Vec<u8> {
    let mut out = Vec::new();
    let mut cur = e;
    while let ast::Expr::Ctor(n, a) = cur {
        if n == "Cons" && a.len() == 2 {
            if let ast::Expr::IntLit(b) = &a[0] {
                // two's-complement BitAnd == mod 256 for negatives as well
                out.push((b & &ast::IntLit::from(255)).to_u8().unwrap_or(0));
            }
            cur = &a[1];
        } else {
            break;
        }
    }
    out
}

/// A byte-list path as an OS path (raw bytes on unix — binary-safe).
fn path(e: &ast::Expr) -> std::path::PathBuf {
    use std::os::unix::ffi::OsStrExt as _;
    std::ffi::OsStr::from_bytes(&list_bytes(e)).into()
}

fn list(items: Vec<ast::Expr>) -> ast::Expr {
    let mut acc = ctor("Nil", vec![]);
    for it in items.into_iter().rev() {
        acc = ctor("Cons", vec![it, acc]);
    }
    acc
}

fn pair(a: ast::Expr, b: ast::Expr) -> ast::Expr {
    ctor("Pair", vec![a, b])
}

fn option(v: Option<ast::Expr>) -> ast::Expr {
    match v {
        Some(x) => ctor("Some", vec![x]),
        None => ctor("None", vec![]),
    }
}

/// The handler of the host calls: it performs the I/O of each call to one of
/// the seven externs the host file declares (v3/kernel/host.shard for the toolchain,
/// v3/std/host.shard for an ordinary closure) — get_args, read_file, read_range, write,
/// write_line, write_file, exit — by the last component of the extern's identity, and
/// answers in the runtime's cells. The World is a one-constructor cell
/// over its clock; it is each extern's LAST argument and is returned with the clock
/// advanced by one.
///
/// The extern boundary speaks RAW BYTES: every byte-list payload is a byte
/// sequence (each element 0..256), not Unicode codepoints. The host performs
/// no encoding or decoding — reads are binary-safe, writes emit the list
/// verbatim (elements masked mod 256).
fn make_handler(prog_args: Vec<String>) -> impl FnMut(&str, &[ast::Expr]) -> Result<ast::Expr, String> {
    use std::io::Write as _;
    move |name: &str, args: &[ast::Expr]| -> Result<ast::Expr, String> {
        let (wname, clk) = match args.last() {
            Some(ast::Expr::Ctor(n, a)) if n.rsplit('.').next() == Some("World") && a.len() == 1 => match &a[0] {
                ast::Expr::IntLit(k) => (n.as_str(), k.clone()),
                other => return Err(format!("World clock is not an Int: {:?}", other)),
            },
            _ => return Err(format!("{}: expected a World as the last argument", name)),
        };
        let w1 = ctor(wname, vec![ast::Expr::IntLit(clk + 1)]);
        match name.rsplit('.').next().unwrap_or(name) {
            // the CLI arguments (after the binary name), each the list of its UTF-8 bytes
            "get_args" => {
                let items: Vec<ast::Expr> = prog_args.iter().map(|a| byte_list(a.as_bytes())).collect();
                Ok(pair(list(items), w1))
            }
            // file contents as (Some bytes), or None on any read error.
            // Binary-safe: the raw bytes, whatever they are — no UTF-8 gate.
            "read_file" => {
                let got = std::fs::read(path(&args[0])).ok().map(|c| byte_list(&c));
                Ok(pair(option(got), w1))
            }
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
                        std::fs::File::open(path(&args[0])).and_then(|mut f| {
                            f.seek(std::io::SeekFrom::Start(off))?;
                            let mut buf = Vec::new();
                            f.take(n as u64).read_to_end(&mut buf)?;
                            Ok(buf)
                        }).ok()
                    }
                    _ => None,
                };
                Ok(pair(option(got.map(|b| byte_list(&b))), w1))
            }
            "write" => {
                let out = list_bytes(&args[0]);
                let mut stdout = std::io::stdout();
                let _ = stdout.write_all(&out);
                let _ = stdout.flush();
                Ok(w1)
            }
            // persist bytes at a path, verbatim — the dual of read_file: the pair of
            // whether it was written and the World. The toolchain writes its artifacts
            // through it (the Init index, a proof sidecar).
            "write_file" => {
                let ok = std::fs::write(path(&args[0]), list_bytes(&args[1])).is_ok();
                Ok(pair(ctor(if ok { "True" } else { "False" }, vec![]), w1))
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

fn ctor(name: &str, args: Vec<ast::Expr>) -> ast::Expr {
    ast::Expr::Ctor(name.into(), args)
}
