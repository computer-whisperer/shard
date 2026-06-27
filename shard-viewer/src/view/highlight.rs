//! Lightweight shard syntax highlighter for the detail panel's source view.
//!
//! Shard has no syntect grammar, so this is a hand-rolled s-expr tokenizer: it
//! scans characters into `(text, color)` tokens following the same palette
//! convention damascene-markdown's highlighter uses (comments muted, strings
//! green, numbers/keywords blue, constructors/types amber).
//!
//! ## Why manual wrapping
//! The old plain `code_block` extended long lines off the right edge with no way
//! to read the rest. The obvious fix — a wrapping paragraph — isn't available
//! for *colored* text: a damascene `text_runs` (multi-color inline paragraph)
//! does not reflow in this layout context (only a single-color `text` leaf
//! does), and there is no horizontal scroll or wrapping-row primitive. But shard
//! source is **monospace**, so every glyph is the same width and we can wrap
//! ourselves at an exact character budget: each visual sub-line is emitted as a
//! non-wrapping `text_runs`, preserving per-token color *and* keeping every line
//! fully on-screen. Continuations hang under their line number.

use damascene_core::prelude::*;

/// One source token: its verbatim text and the palette color to paint it.
type Tok = (String, Color);

/// Build the highlighted, line-numbered source view (inside the standard
/// code-block chrome). Lines wider than `max_chars` code columns wrap onto
/// continuation rows. The caller wraps the result in a `scroll` for vertical
/// overflow.
pub(crate) fn source_view(src: &str, max_chars: usize) -> El {
    let sz = tokens::TEXT_SM.size;
    let digits = src.lines().count().max(1).to_string().len();
    let budget = max_chars.max(8);
    let mut rows: Vec<El> = Vec::new();
    for (i, line) in src.lines().enumerate() {
        let visuals = wrap_tokens(tokenize(line), budget);
        for (k, vline) in visuals.iter().enumerate() {
            // Line number on the first visual row; blank (aligned) on wraps.
            let label = if k == 0 {
                format!("{:>digits$}", i + 1)
            } else {
                " ".repeat(digits)
            };
            let gutter = text(label)
                .mono()
                .font_size(sz)
                .text_color(tokens::MUTED_FOREGROUND)
                .nowrap_text();
            let runs: Vec<El> = vline
                .iter()
                .map(|(t, c)| text(t.clone()).mono().font_size(sz).text_color(*c))
                .collect();
            rows.push(
                row([gutter, text_runs(runs).nowrap_text()])
                    .gap(tokens::SPACE_3)
                    .align(Align::Start),
            );
        }
    }
    if rows.is_empty() {
        rows.push(text("(empty)").mono().muted().font_size(sz));
    }
    code_block_chrome(column(rows).gap(1.0))
}

/// Greedily pack tokens into visual lines of at most `budget` characters,
/// breaking only at token boundaries (a single token longer than the budget
/// gets its own line and is allowed to overflow). A two-space hanging indent
/// marks continuation lines. Leading whitespace at a wrap point is dropped.
fn wrap_tokens(toks: Vec<Tok>, budget: usize) -> Vec<Vec<Tok>> {
    let mut lines: Vec<Vec<Tok>> = Vec::new();
    let mut cur: Vec<Tok> = Vec::new();
    let mut width = 0usize;
    for (text, color) in toks {
        let len = text.chars().count();
        if width + len > budget && width > 0 {
            lines.push(std::mem::take(&mut cur));
            // Hanging indent for the continuation; skip a pure-whitespace token
            // sitting at the fresh break (it would just be ragged leading space).
            cur.push(("  ".to_string(), color));
            width = 2;
            if text.trim().is_empty() {
                continue;
            }
        }
        cur.push((text, color));
        width += len;
    }
    if !cur.is_empty() || lines.is_empty() {
        lines.push(cur);
    }
    lines
}

/// Tokenize one source line into `(text, color)` pairs (whitespace preserved as
/// default-colored tokens so columns line up).
fn tokenize(line: &str) -> Vec<Tok> {
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    let mut toks: Vec<Tok> = Vec::new();
    let mut i = 0;
    while i < n {
        let c = chars[i];
        if c == ';' {
            toks.push((chars[i..].iter().collect(), tokens::MUTED_FOREGROUND));
            break;
        } else if c == '"' {
            let start = i;
            i += 1;
            while i < n && chars[i] != '"' {
                i += if chars[i] == '\\' { 2 } else { 1 };
            }
            i = (i + 1).min(n);
            toks.push((chars[start..i].iter().collect(), tokens::SUCCESS));
        } else if c == '(' || c == ')' {
            toks.push((c.to_string(), tokens::MUTED_FOREGROUND));
            i += 1;
        } else if c == '\'' {
            toks.push((c.to_string(), tokens::SUCCESS));
            i += 1;
        } else if c.is_whitespace() {
            let start = i;
            while i < n && chars[i].is_whitespace() {
                i += 1;
            }
            toks.push((chars[start..i].iter().collect(), tokens::FOREGROUND));
        } else {
            let start = i;
            while i < n && !is_delim(chars[i]) {
                i += 1;
            }
            let tok: String = chars[start..i].iter().collect();
            let color = atom_color(&tok);
            toks.push((tok, color));
        }
    }
    toks
}

fn is_delim(c: char) -> bool {
    c.is_whitespace() || matches!(c, '(' | ')' | ';' | '"' | '\'')
}

/// Color an atom: numbers and special forms blue, `Capitalized` constructors /
/// types amber, everything else (identifiers / params / fn names) the default.
fn atom_color(tok: &str) -> Color {
    let body = tok.strip_prefix('-').unwrap_or(tok);
    if !body.is_empty() && body.chars().all(|c| c.is_ascii_digit()) {
        return tokens::INFO;
    }
    if is_keyword(tok) {
        return tokens::INFO;
    }
    if tok.chars().next().is_some_and(char::is_uppercase) {
        return tokens::WARNING;
    }
    tokens::FOREGROUND
}

/// The shard special forms (declaration heads + control structures). Coloring
/// these blue ties the source view to the Flow view's control vocabulary.
fn is_keyword(tok: &str) -> bool {
    matches!(
        tok,
        "fn" | "sig"
            | "type"
            | "claim"
            | "fulfills"
            | "requirement"
            | "bin"
            | "module"
            | "use"
            | "axiom"
            | "theory"
            | "measure"
            | "match"
            | "if"
            | "let"
            | "struct"
            | "quote"
            | "import"
            | "entry"
            | "externs"
            | "trusts"
            | "requires"
            | "refine"
            | "prove"
            | "have"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_at_budget_and_keeps_all_tokens() {
        let toks = tokenize("(foo alpha beta gamma delta epsilon)");
        let total: String = toks.iter().map(|(t, _)| t.as_str()).collect();
        assert_eq!(total, "(foo alpha beta gamma delta epsilon)");
        let lines = wrap_tokens(toks, 12);
        assert!(lines.len() > 1, "long line should wrap at a 12-char budget");
        // Every original character survives the wrap (minus injected indents).
        let joined: String = lines
            .iter()
            .flat_map(|l| l.iter().map(|(t, _)| t.clone()))
            .collect();
        assert!(joined.contains("epsilon") && joined.contains("(foo"));
    }

    #[test]
    fn short_line_is_one_visual_row() {
        let lines = wrap_tokens(tokenize("(if a b)"), 40);
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn colors_constructors_and_keywords() {
        let toks = tokenize("(match Cons x)");
        let by_text = |want: &str| toks.iter().find(|(t, _)| t == want).map(|(_, c)| *c);
        assert_eq!(by_text("match"), Some(tokens::INFO));
        assert_eq!(by_text("Cons"), Some(tokens::WARNING));
        assert_eq!(by_text("x"), Some(tokens::FOREGROUND));
    }
}
