mod interpreter;
mod parser;

//- ADD ---------------------------------------------------------------------

#[inline(always)]
pub fn add(lhs: i64, rhs: i64) -> i64 {
    lhs + rhs
}

pub fn add_checked(lhs: i64, rhs: i64) -> Result<i64, String> {
    let (result, overflow) = lhs.overflowing_add(rhs);
    if overflow {
        Err("Attempt to add with overflow: {lhs} + {rhs} = {result}".to_string())
    }
    Ok(result)
}

//- TESTS -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::parser::Expr;
    use crate::parser::types::Int64;
    use crate::parser::operators::Operator;

    #[test]
    fn expr_add() {
        assert_eq!(
            Expr::new("1 + 2"),
            (
                "",
                Expr {
                    lhs: Int64{ value: 1},
                    rhs: Int64{ value: 2},
                    op: Operator::Add,
                },
            ),
        );
    }
}
