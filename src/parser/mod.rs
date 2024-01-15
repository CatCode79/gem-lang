pub(crate) mod types;
pub(crate) mod operators;

use crate::parser::operators::Operator;
use crate::parser::types::Int64;

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Int64,
    pub rhs: Int64,
    pub op: Operator,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Int64::new(s);
        let (s, _) = extract_whitespace(s);

        let (s, op) = Operator::new(s);
        let (s, _) = extract_whitespace(s);

        let (s, rhs) = Int64::new(s);

        (s, Self { lhs, rhs, op })
    }
}

pub(crate) fn extract_operator(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}