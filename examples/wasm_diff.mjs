// wasm_diff.mjs — engine side of the ISA slice-4 differential (see
// wasm_diff_run.shard for the plan format). Instantiates each MOD's bytes
// under the real engine (V8) and replays each CASE, comparing the engine's
// result against the model's. Any validation failure or mismatch is a FAIL;
// exit code is the number of failing lines (0 = full agreement).
import { readFileSync } from "node:fs";

const plan = readFileSync(process.argv[2], "utf8").split("\n");
const mods = new Map();
const modbytes = new Map();
let ok = 0, fail = 0;

const report = (verdict, line, detail) => {
  if (verdict) { ok++; } else { fail++; console.log(`FAIL ${line}${detail ? `  [${detail}]` : ""}`); }
};

for (const line of plan) {
  if (line.startsWith("MOD ")) {
    const [, name, hex] = line.split(" ");
    try {
      const bytes = Uint8Array.from(hex.match(/../g).map((b) => parseInt(b, 16)));
      modbytes.set(name, bytes);
      mods.set(name, new WebAssembly.Instance(new WebAssembly.Module(bytes)).exports);
    } catch (e) {
      mods.set(name, null);
      report(false, `MOD ${name}`, `engine rejects: ${e.message}`);
    }
  } else if (line.startsWith("CASE ")) {
    const m = line.match(/^CASE (\S+) (\S+)((?: \d+)*) -> (\S+)$/);
    if (!m) { report(false, line, "unparseable"); continue; }
    const [, mod, fn, argstr, expect] = m;
    const exports = mods.get(mod);
    if (!exports) { report(false, line, "module unavailable"); continue; }
    const args = argstr.trim() === "" ? [] : argstr.trim().split(" ").map(Number);
    let got;
    try {
      got = (exports[fn](...args) >>> 0).toString(); // engine i32 -> u32 view
    } catch (e) {
      // model None = trap: the engine trapping IS agreement (the trap leg
      // of the differential — division by zero etc. must trap BOTH sides).
      report(expect === "None", line, `engine traps: ${e.message}`);
      continue;
    }
    report(got === expect, line, `engine says ${got}`);
  } else if (line.startsWith("MEMCASE ")) {
    // MEMCASE <mod> <fn> <args...> MEM <hex-in> -> <result> <hex-out>
    // Fresh instance per case (memory is stateful); write hex-in at 0, call,
    // read back hex-in.length bytes and compare result + memory.
    const m = line.match(/^MEMCASE (\S+) (\S+)((?: \d+)*) MEM (\S+) -> (\S+) (\S+)$/);
    if (!m) { report(false, line, "unparseable"); continue; }
    const [, mod, fn, argstr, memin, expectv, memout] = m;
    const bytes = modbytes.get(mod);
    if (!bytes) { report(false, line, "module unavailable"); continue; }
    let inst;
    try {
      inst = new WebAssembly.Instance(new WebAssembly.Module(bytes)).exports;
    } catch (e) { report(false, line, `engine rejects: ${e.message}`); continue; }
    const inb = memin.match(/../g).map((h) => parseInt(h, 16));
    new Uint8Array(inst.mem.buffer).set(inb, 0);
    const args = argstr.trim() === "" ? [] : argstr.trim().split(" ").map(Number);
    let got;
    try {
      got = (inst[fn](...args) >>> 0).toString();
    } catch (e) {
      report(false, line, `engine traps: ${e.message}`);
      continue;
    }
    const after = Array.from(new Uint8Array(inst.mem.buffer).slice(0, inb.length))
      .map((b) => b.toString(16).padStart(2, "0")).join("");
    report(got === expectv && after === memout, line, `engine says ${got} ${after}`);
  }
}

console.log(`wasm differential: ${ok} agree, ${fail} disagree`);
process.exit(fail);
