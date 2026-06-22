//! A lightweight s-expression reader for shard source.
//!
//! This is *not* the kernel elaborator — it reads the paren tree only, far
//! enough to recover a project's structure (imports, fns, types, claims) and
//! to scan fn bodies for the symbols they reference. Shard's surface syntax is
//! plain s-exprs: `;`-to-EOL comments, `"..."` string literals, the `'X` quote
//! reader macro, and atoms (symbols / ints).

/// A parsed s-expression. Ints and symbols are kept distinct so a call-scan can
/// ignore numeric literals, but everything else is just symbols and lists.
#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Int(i64),
    Sym(String),
    Str(String),
    List(Vec<Sexpr>),
}

impl Sexpr {
    /// The head symbol of a list, e.g. `fn` in `(fn name ...)`. `None` for
    /// atoms or lists whose first element isn't a symbol.
    pub fn head(&self) -> Option<&str> {
        match self {
            Sexpr::List(items) => match items.first() {
                Some(Sexpr::Sym(s)) => Some(s.as_str()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn as_sym(&self) -> Option<&str> {
        match self {
            Sexpr::Sym(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[Sexpr]> {
        match self {
            Sexpr::List(items) => Some(items),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

/// Parse a whole source file into its sequence of top-level forms.
pub fn parse_top(src: &str) -> Result<Vec<Sexpr>, ParseError> {
    Ok(parse_top_spanned(src)?
        .into_iter()
        .map(|(e, _)| e)
        .collect())
}

/// Like [`parse_top`], but pairs each top-level form with the exact source
/// text it was parsed from (verbatim, comments between forms excluded). Used to
/// show a fn's real source in the viewer.
pub fn parse_top_spanned(src: &str) -> Result<Vec<(Sexpr, String)>, ParseError> {
    let mut p = Parser {
        chars: src.chars().collect(),
        pos: 0,
        line: 1,
    };
    let mut forms = Vec::new();
    loop {
        p.skip_trivia();
        if p.pos >= p.chars.len() {
            break;
        }
        let start = p.pos;
        let expr = p.read_expr()?;
        let text: String = p.chars[start..p.pos].iter().collect();
        forms.push((expr, text));
    }
    Ok(forms)
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
    line: usize,
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if let Some(c) = c {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
            }
        }
        c
    }

    /// Skip whitespace and `;`-to-end-of-line comments.
    fn skip_trivia(&mut self) {
        while let Some(c) = self.peek() {
            if c == ';' {
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.bump();
                }
            } else if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn read_expr(&mut self) -> Result<Sexpr, ParseError> {
        self.skip_trivia();
        match self.peek() {
            None => Err(self.err("unexpected end of input")),
            Some('(') | Some('[') => self.read_list(),
            Some(')') | Some(']') => Err(self.err("unexpected close paren")),
            Some('"') => self.read_string(),
            Some('\'') => {
                self.bump();
                let quoted = self.read_expr()?;
                Ok(Sexpr::List(vec![Sexpr::Sym("quote".to_string()), quoted]))
            }
            Some(_) => self.read_atom(),
        }
    }

    fn read_list(&mut self) -> Result<Sexpr, ParseError> {
        let open = self.bump().unwrap(); // ( or [
        let close = if open == '(' { ')' } else { ']' };
        let mut items = Vec::new();
        loop {
            self.skip_trivia();
            match self.peek() {
                None => return Err(self.err("unterminated list")),
                Some(c) if c == close => {
                    self.bump();
                    break;
                }
                // Tolerate mixed bracket kinds rather than erroring on them.
                Some(c) if c == ')' || c == ']' => {
                    self.bump();
                    break;
                }
                Some(_) => items.push(self.read_expr()?),
            }
        }
        Ok(Sexpr::List(items))
    }

    fn read_string(&mut self) -> Result<Sexpr, ParseError> {
        self.bump(); // opening quote
        let mut s = String::new();
        loop {
            match self.bump() {
                None => return Err(self.err("unterminated string")),
                Some('"') => break,
                Some('\\') => match self.bump() {
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('r') => s.push('\r'),
                    Some('\\') => s.push('\\'),
                    Some('"') => s.push('"'),
                    Some(other) => s.push(other),
                    None => return Err(self.err("unterminated string escape")),
                },
                Some(c) => s.push(c),
            }
        }
        Ok(Sexpr::Str(s))
    }

    fn read_atom(&mut self) -> Result<Sexpr, ParseError> {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ';' | '"') {
                break;
            }
            s.push(c);
            self.bump();
        }
        if s.is_empty() {
            return Err(self.err("empty atom"));
        }
        // Integer literal (incl. a leading `-`), else a symbol.
        if let Ok(n) = s.parse::<i64>() {
            Ok(Sexpr::Int(n))
        } else {
            Ok(Sexpr::Sym(s))
        }
    }

    fn err(&self, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            line: self.line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_fn() {
        let forms = parse_top("(fn bump ((x Int)) Int (+ x 3))").unwrap();
        assert_eq!(forms.len(), 1);
        assert_eq!(forms[0].head(), Some("fn"));
    }

    #[test]
    fn skips_comments() {
        let src = ";; a comment\n(import \"foo.shard\") ; trailing\n";
        let forms = parse_top(src).unwrap();
        assert_eq!(forms.len(), 1);
        assert_eq!(forms[0].head(), Some("import"));
    }

    #[test]
    fn quote_macro() {
        let forms = parse_top("'None").unwrap();
        assert_eq!(forms[0].head(), Some("quote"));
    }
}
