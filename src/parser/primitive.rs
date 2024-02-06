use crate::parser::extract_digits;

//- 64 BIT INTEGER TYPE ------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct Int64 {
    pub(crate) value: i64,
}

impl Int64 {
    pub(crate) fn new(token: &str) -> (&str, Self) {
        let (s, numerical) = extract_digits(token);
        (
            s,
            Self {
                value: numerical.parse().unwrap(), // TODO: avoid having this unwrap a priori
            },
        )
    }
}

//- 64 BIT FLOATING POINT TYPE -----------------------------------------------

#[derive(Debug, PartialEq)]
pub struct Float64 {
    value: f64,
}
