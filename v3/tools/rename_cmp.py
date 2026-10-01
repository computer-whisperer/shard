#!/usr/bin/env python3
"""v3/tools/rename_cmp.py — E's rename of the comparison primitives
(v3/LANGUAGE.md §8.4, slice 3.17 rule 5): `(lt a b)` `(le a b)` `(int_eq a b)`
become `(< a b)` `(<= a b)` `(= a b)`.

    rename_cmp.py [--check] FILE-or-DIR…

Rewrites a symbol only where it is the head of a list — the token straight
after an opening parenthesis — and never inside a `"…"` string, a `;`
comment or a `(quote …)` form: a binder, a field or a quoted symbol named
`lt` is data and stays. `--check` rewrites nothing and exits 1 when a file
would change (the gate for a tree that has migrated).

Slice 3.17 ran it over v3/std, v3/examples and v3/pins (the pins
rename_refused and rename_refused_e hold the old spellings on purpose, so
`--check` reports those two sites). v3/kernel migrates with it when route
1's chain is V3's own.
"""
import os
import sys

NEW = {"lt": "<", "le": "<=", "int_eq": "="}
DELIMS = set(" \t\r\n()\";")


def rewrite(src):
    out = []
    i, n = 0, len(src)
    after_open = False      # the previous token was "("
    quote_depth = []        # paren depths at which a (quote …) form opened
    depth = 0
    changed = 0
    while i < n:
        c = src[i]
        if c == ";":
            j = src.find("\n", i)
            j = n if j < 0 else j
            out.append(src[i:j]); i = j
            continue
        if c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            out.append(src[i:j + 1]); i = j + 1
            after_open = False
            continue
        if c == "(":
            depth += 1
            out.append(c); i += 1
            after_open = True
            continue
        if c == ")":
            if quote_depth and quote_depth[-1] == depth:
                quote_depth.pop()
            depth -= 1
            out.append(c); i += 1
            after_open = False
            continue
        if c in " \t\r\n":
            out.append(c); i += 1
            continue
        j = i
        while j < n and src[j] not in DELIMS:
            j += 1
        tok = src[i:j]
        if after_open and tok == "quote":
            quote_depth.append(depth)
        elif after_open and not quote_depth and tok in NEW:
            tok = NEW[tok]
            changed += 1
        out.append(tok); i = j
        after_open = False
    return "".join(out), changed


def files(paths):
    for p in paths:
        if os.path.isdir(p):
            for root, _, names in os.walk(p):
                for name in sorted(names):
                    if name.endswith(".shard"):
                        yield os.path.join(root, name)
        else:
            yield p


def main(argv):
    check = "--check" in argv
    paths = [a for a in argv if a != "--check"]
    if not paths:
        sys.exit(__doc__)
    total = touched = 0
    for path in files(paths):
        with open(path, encoding="utf-8") as f:
            src = f.read()
        new, k = rewrite(src)
        if k:
            total += k; touched += 1
            print(f"{path}: {k}")
            if not check:
                with open(path, "w", encoding="utf-8") as f:
                    f.write(new)
    print(f"rename_cmp: {total} sites in {touched} files" + (" would change" if check else " rewritten"))
    return 1 if check and total else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
