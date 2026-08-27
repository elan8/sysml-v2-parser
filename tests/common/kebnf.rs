//! Small `.kebnf` expression parser used to derive nullable-aware FIRST sets in conformance tests.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Empty,
    Terminal(String),
    Reference(String),
    Sequence(Vec<Expr>),
    Choice(Vec<Expr>),
    Optional(Box<Expr>),
    Repeat(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Identifier(String),
    Terminal(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Choice,
    Optional,
    Star,
    Plus,
    Assign,
    AddAssign,
    QueryAssign,
    Dot,
    Comma,
}

pub fn parse_grammar(source: &str) -> BTreeMap<String, Expr> {
    let mut definitions = BTreeMap::new();
    let mut current_name: Option<String> = None;
    let mut current_rhs = String::new();

    let finish =
        |name: Option<String>, rhs: &mut String, definitions: &mut BTreeMap<String, Expr>| {
            if let Some(name) = name {
                let tokens = tokenize(rhs);
                let mut parser = Parser {
                    tokens: &tokens,
                    at: 0,
                };
                definitions.insert(name, parser.choice(None));
                assert_eq!(parser.at, tokens.len(), "unparsed .kebnf tokens in {rhs:?}");
            }
            rhs.clear();
        };

    for raw_line in source.lines() {
        let line = raw_line.split_once("//").map_or(raw_line, |(head, _)| head);
        let starts_definition = raw_line
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphabetic);
        if starts_definition {
            if let Some(eq) = definition_equals(line) {
                finish(current_name.take(), &mut current_rhs, &mut definitions);
                let lhs = &line[..eq];
                let name = lhs
                    .split(|ch: char| ch == ':' || ch.is_whitespace())
                    .find(|part| !part.is_empty())
                    .expect("production name");
                current_name = Some(name.to_owned());
                current_rhs.push_str(&line[eq + 1..]);
                current_rhs.push('\n');
                continue;
            }
        }
        if current_name.is_some() {
            current_rhs.push_str(line);
            current_rhs.push('\n');
        }
    }
    finish(current_name, &mut current_rhs, &mut definitions);
    definitions
}

fn definition_equals(line: &str) -> Option<usize> {
    let mut quoted = false;
    for (index, ch) in line.char_indices() {
        match ch {
            '\'' => quoted = !quoted,
            '=' if !quoted => return Some(index),
            _ => {}
        }
    }
    None
}

pub fn first_terminals(grammar: &BTreeMap<String, Expr>, root: &str) -> BTreeSet<String> {
    let mut nullable = BTreeMap::<String, bool>::new();
    let mut first = BTreeMap::<String, BTreeSet<String>>::new();
    loop {
        let mut changed = false;
        for (name, expr) in grammar {
            let is_nullable = expr_nullable(expr, &nullable);
            if nullable.insert(name.clone(), is_nullable) != Some(is_nullable) {
                changed = true;
            }
            let terminals = expr_first(expr, &nullable, &first);
            if first.insert(name.clone(), terminals.clone()).as_ref() != Some(&terminals) {
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    first.get(root).cloned().unwrap_or_default()
}

fn expr_nullable(expr: &Expr, nullable: &BTreeMap<String, bool>) -> bool {
    match expr {
        Expr::Empty | Expr::Optional(_) | Expr::Repeat(_) => true,
        Expr::Terminal(_) => false,
        Expr::Reference(name) => nullable.get(name).copied().unwrap_or(false),
        Expr::Sequence(items) => items.iter().all(|item| expr_nullable(item, nullable)),
        Expr::Choice(items) => items.iter().any(|item| expr_nullable(item, nullable)),
    }
}

fn expr_first(
    expr: &Expr,
    nullable: &BTreeMap<String, bool>,
    first: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    match expr {
        Expr::Empty => BTreeSet::new(),
        Expr::Terminal(value) => [value.clone()].into_iter().collect(),
        Expr::Reference(name) => first.get(name).cloned().unwrap_or_default(),
        Expr::Optional(inner) | Expr::Repeat(inner) => expr_first(inner, nullable, first),
        Expr::Choice(items) => items
            .iter()
            .flat_map(|item| expr_first(item, nullable, first))
            .collect(),
        Expr::Sequence(items) => {
            let mut result = BTreeSet::new();
            for item in items {
                result.extend(expr_first(item, nullable, first));
                if !expr_nullable(item, nullable) {
                    break;
                }
            }
            result
        }
    }
}

fn tokenize(source: &str) -> Vec<Token> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        let byte = bytes[at];
        if byte.is_ascii_whitespace() {
            at += 1;
            continue;
        }
        if byte == b'\'' {
            let start = at + 1;
            at += 1;
            while at < bytes.len() && bytes[at] != b'\'' {
                if bytes[at] == b'\\' && at + 1 < bytes.len() {
                    at += 2;
                } else {
                    at += 1;
                }
            }
            assert!(at < bytes.len(), "unterminated terminal in .kebnf");
            tokens.push(Token::Terminal(source[start..at].to_owned()));
            at += 1;
            continue;
        }
        if byte.is_ascii_alphabetic() || byte == b'_' {
            let start = at;
            at += 1;
            while at < bytes.len() && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_') {
                at += 1;
            }
            tokens.push(Token::Identifier(source[start..at].to_owned()));
            continue;
        }
        let (token, width) = if bytes[at..].starts_with(b"+=") {
            (Token::AddAssign, 2)
        } else if bytes[at..].starts_with(b"?=") {
            (Token::QueryAssign, 2)
        } else {
            let token = match byte {
                b'(' => Token::LParen,
                b')' => Token::RParen,
                b'{' => Token::LBrace,
                b'}' => Token::RBrace,
                b'|' => Token::Choice,
                b'?' => Token::Optional,
                b'*' => Token::Star,
                b'+' => Token::Plus,
                b'=' => Token::Assign,
                b'.' => Token::Dot,
                b',' => Token::Comma,
                _ => {
                    at += 1;
                    continue;
                }
            };
            (token, 1)
        };
        tokens.push(token);
        at += width;
    }
    tokens
}

struct Parser<'a> {
    tokens: &'a [Token],
    at: usize,
}

impl Parser<'_> {
    fn choice(&mut self, closing: Option<Token>) -> Expr {
        let mut choices = vec![self.sequence(closing.as_ref())];
        while self.peek() == Some(&Token::Choice) {
            self.at += 1;
            choices.push(self.sequence(closing.as_ref()));
        }
        if choices.len() == 1 {
            choices.pop().unwrap_or(Expr::Empty)
        } else {
            Expr::Choice(choices)
        }
    }

    fn sequence(&mut self, closing: Option<&Token>) -> Expr {
        let mut items = Vec::new();
        while self.at < self.tokens.len()
            && self.peek() != Some(&Token::Choice)
            && closing.is_none_or(|token| self.peek() != Some(token))
        {
            if self.peek() == Some(&Token::Comma) {
                self.at += 1;
                continue;
            }
            items.push(self.postfix());
        }
        match items.len() {
            0 => Expr::Empty,
            1 => items.pop().unwrap_or(Expr::Empty),
            _ => Expr::Sequence(items),
        }
    }

    fn postfix(&mut self) -> Expr {
        let mut expr = self.primary();
        while let Some(token) = self.peek() {
            expr = match token {
                Token::Optional => Expr::Optional(Box::new(expr)),
                Token::Star => Expr::Repeat(Box::new(expr)),
                Token::Plus => {
                    self.at += 1;
                    continue;
                }
                _ => break,
            };
            self.at += 1;
        }
        expr
    }

    fn primary(&mut self) -> Expr {
        match self.tokens.get(self.at).cloned() {
            Some(Token::LParen) => self.group(Token::RParen),
            Some(Token::LBrace) => self.group(Token::RBrace),
            Some(Token::Terminal(value)) => {
                self.at += 1;
                Expr::Terminal(value)
            }
            Some(Token::Identifier(name)) => {
                self.at += 1;
                while self.peek() == Some(&Token::Dot) {
                    self.at += 1;
                    if matches!(self.peek(), Some(Token::Identifier(_))) {
                        self.at += 1;
                    }
                }
                if matches!(
                    self.peek(),
                    Some(Token::Assign | Token::AddAssign | Token::QueryAssign)
                ) {
                    self.at += 1;
                    self.primary()
                } else {
                    Expr::Reference(name)
                }
            }
            Some(Token::Assign | Token::AddAssign | Token::QueryAssign) => {
                self.at += 1;
                self.primary()
            }
            Some(unexpected) => panic!("unexpected .kebnf token {unexpected:?}"),
            None => Expr::Empty,
        }
    }

    fn group(&mut self, closing: Token) -> Expr {
        self.at += 1;
        let expression = self.choice(Some(closing.clone()));
        assert_eq!(
            self.tokens.get(self.at),
            Some(&closing),
            "unclosed .kebnf group"
        );
        self.at += 1;
        expression
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_follows_nullable_prefixes_and_assignments() {
        let grammar = parse_grammar(
            "Root = Prefix? ( owned += A | owned += B )\nPrefix = 'public'\nA = 'binding' 'bind'\nB = 'part'\n",
        );
        assert_eq!(
            first_terminals(&grammar, "Root"),
            ["binding", "part", "public"]
                .into_iter()
                .map(str::to_owned)
                .collect()
        );
    }
}
