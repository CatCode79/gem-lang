use crate::parser::{extract_whitespace, operator::Operator, primitive::Int64};

//- MATH EXPRESSION ----------------------------------------------------------

#[derive(Debug, PartialEq)]
pub(crate) struct Expression {
    pub lhs: Int64,
    pub rhs: Int64,
    pub op: Operator,
}

impl Expression {
    pub(crate) fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Int64::new(s);
        let (s, _) = extract_whitespace(s);

        let (s, op) = Operator::new(s);
        let (s, _) = extract_whitespace(s);

        let (s, rhs) = Int64::new(s);

        (s, Self { lhs, rhs, op })
    }
}
