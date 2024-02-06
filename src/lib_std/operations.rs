//- ADD ---------------------------------------------------------------------

#[inline(always)]
pub(crate) fn add(lhs: i64, rhs: i64) -> i64 {
    lhs + rhs
}

pub(crate) fn add_checked(lhs: i64, rhs: i64) -> Result<i64, String> {
    let (result, overflow) = lhs.overflowing_add(rhs);
    if overflow {
        return Err("Attempt to add with overflow: {lhs} + {rhs} = {result}".to_string());
    }
    Ok(result)
}

//- TESTS -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::parser::operator::Operator;
    use crate::parser::primitive::Int64;
    use crate::parser::expression::Expression;

    #[test]
    fn expr_add() {
        assert_eq!(
            Expression::new("1 + 2"),
            (
                "",
                Expression {
                    lhs: Int64 { value: 1 },
                    rhs: Int64 { value: 2 },
                    op: Operator::Add,
                },
            ),
        );
    }
}
