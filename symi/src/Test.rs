//! Test : A Test type, currently structured to display an if-statement with
//! clang_analyzer_eval calls.

use crate::Draw::Draw;
use crate::Expr::BinarySymExpr;
use std::fmt;
use std::sync::Arc;

/// A type for denoting test cases. Each test case for two Sym x and y consists
/// of:
/// 1. A set of constraints involving x and y.
/// 2. An assertion (the test). This is a simple BinarySymExpr.
///
/// # Examples:
/// ```c
/// if (x > 0 && x < 10 && y > 11 && y < 20) {
///     // x: [1, 9], y: [12, 19]
///     clang_analyzer_eval(x != y); // expected-warning{{TRUE}}
/// }
/// ```
pub struct Test<T1, T2, T3, T4>
where
    T1: Draw,
    T2: Draw,
    T3: Draw,
    T4: Draw,
{
    // TODO: We may need a WarningRange member for `expected-warning{{}}`.
    Conditional: Arc<BinarySymExpr<T1, T2>>,
    Assertion: Arc<BinarySymExpr<T3, T4>>, // Concrete types involved?
}

impl<T1, T2, T3, T4> Test<T1, T2, T3, T4>
where
    T1: Draw,
    T2: Draw,
    T3: Draw,
    T4: Draw,
{
    pub fn new(
        Conditional: &Arc<BinarySymExpr<T1, T2>>,
        Assertion: &Arc<BinarySymExpr<T3, T4>>,
    ) -> Arc<Test<T1, T2, T3, T4>> {
        Arc::new(Test {
            Conditional: Arc::clone(Conditional),
            Assertion: Arc::clone(Assertion),
        })
    }
}

impl<T1, T2, T3, T4> fmt::Display for Test<T1, T2, T3, T4>
where
    T1: fmt::Display + Draw,
    T2: fmt::Display + Draw,
    T3: fmt::Display + Draw,
    T4: fmt::Display + Draw,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
  if {} {{
      clang_analyzer_eval{};
  }}
  ",
            self.Conditional, self.Assertion
        )
    }
}

impl<T1, T2, T3, T4> Draw for Test<T1, T2, T3, T4>
where
    T1: Draw,
    T2: Draw,
    T3: Draw,
    T4: Draw,
{
    // TODO:
    fn dump(&self, _tabl: Option<usize>) -> String {
        // println!("This is a test from dump method");
        // format!("This is a test from dump method");
        "This is a test from dump method".to_string()
    }

    fn declare(&self) -> String {
        format!("")
    }

    type OutputType = Test<T1, T2, T3, T4>;
    fn iter(&self) -> (bool, &Self::OutputType) {
        (true, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Symbol;

    #[test]
    fn test_TestExpr() {
        let s = Symbol::Sym::new("x", "int");
        let c = Symbol::Sym::getConstraintsAround(&s, 0, 1);
        let a = Symbol::Sym::getConstraintsAround(&s, 1, 2);

        let _t = Test::new(&c, &a);
        println!("Test = {}", _t);
        println!("{s}");
        println!("{c}");
    }
}
