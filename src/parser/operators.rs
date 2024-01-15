use crate::parser::extract_operator;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn new(token: &str) -> (&str, Self) {
        let (s, operator) = extract_operator(token);
        let operator = match operator {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        };
        (s, operator)
    }
}
