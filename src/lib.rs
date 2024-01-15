mod interpreter;
mod parser;

//- ADD ---------------------------------------------------------------------

use std::arch::asm;

// just toying with asm
#[inline]
pub fn asm_add(mut lhs: i64, rhs: i64) -> i64 {
    unsafe {
        asm!("add {0:r}, {1:r}", inout(reg) lhs, in(reg) rhs);
    }
    lhs
}

#[cfg(debug_assertions)]
pub fn gem_add(lhs: i64, rhs: i64) -> i64 {
    let (rust_result, overflow) = lhs.overflowing_add(rhs);
    if overflow {
        panic!("attempt to add with overflow: {lhs} + {rhs} = {rust_result}");
    }
    let asm_result = asm_add(lhs, rhs);
    assert_eq!(rust_result, asm_result, "rust result and asm result are different on addition: {rust_result} != {asm_result}");
    asm_result
}

#[cfg(not(debug_assertions))]
pub fn gem_add(num1: i64, num2: i64) -> i64 {
    asm_add(num1, num2)
}

//- TESTS -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{asm_add, gem_add};
    use crate::parser::Expr;
    use crate::parser::types::Int64;
    use crate::parser::operators::Operator;

    #[test]
    fn add_positive() {
        let result = gem_add(1, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn add_negative() {
        let result = gem_add(-1, -2);
        assert_eq!(result, -3);
    }

    #[test]
    fn add_max_i32() {
        let result = gem_add(i32::MAX as i64, i32::MAX as i64);
        assert_eq!(result, i32::MAX as i64 +  i32::MAX as i64);
    }

    #[test]
    fn add_max_i64() {
        let result = asm_add(i64::MAX, i64::MAX);
        assert_eq!(result, -2); // Overflow!
    }

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
