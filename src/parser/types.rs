use crate::parser::extract_digits;

#[derive(Debug, PartialEq)]
pub struct Int64 {
    pub(crate) value: i64,
}

impl Int64 {
    pub fn new(token: &str) -> (&str, Self) {
        let (s, number) = extract_digits(token);
        (s, Self  {
            value: number.parse().unwrap(),  // TODO: remove unwrap
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Float64 {
    value: f64,
}
