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
        let (s, op) = Operator::new(s);
        let (s, rhs) = Int64::new(s);

        (s, Self { lhs, rhs, op })
    }
}

pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    let mut digits_end = 0;

    for (idx, c) in s.char_indices() {
        if c.is_ascii_digit() {
            digits_end = idx + 1;
        } else {
            break;
        }
    }

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];
    (remainder, digits)
}

pub(crate) fn extract_operator(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}