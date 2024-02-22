use crate::parser::expression::Expression;

//- LET BINDING DEFINITION ---------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct Binding {
    name: String,
    val: Expression,
}

impl Binding {
    pub fn new(s: &str) -> (&str, Self) {
        let s = if s.starts_with("let") {
            &s[3..]
        } else {
            panic!("expected let")
        };
        let (s, _) = crate::parser::extract_whitespace(s);

        let (s, name) = crate::parser::extract_ident(s);
        let (s, _) = crate::parser::extract_whitespace(s);

        let s = if s.starts_with('=') {
            &s[1..]
        } else {
            panic!("expected equals sign")
        };
        let (s, _) = crate::parser::extract_whitespace(s);

        let (s, val) = Expression::new(s);

        (
            s,
            Self {
                name: name.to_string(),
                val,
            },
        )
    }
}

//- TEST ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::parser::binding::Binding;
    use crate::parser::expression::Expression;
    use crate::parser::operator::Operator;
    use crate::parser::primitive::Int64;

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Binding::new("let a = 10 / 2"),
            (
                "",
                Binding {
                    name: "a".to_string(),
                    val: Expression {
                        lhs: Int64::new(10),
                        rhs: Int64::new(2),
                        op: Operator::Div,
                    },
                },
            ),
        );
    }
}
