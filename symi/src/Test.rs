//! Test : A Test type, currently structured to display an if-statement with
//! clang_analyzer_eval calls.

use crate::Draw::Draw;
use crate::Expr::BinarySymExpr;
use std::fmt;

/// A type for denoting test cases. Each test case for two Sym x and y consists
/// of:
/// 1. A set of constraints involving x and y.
/// 2. An assertion (the test). This is a simple BinarySymExpr.
///
/// # Examples:
/// ```
/// if (x > 0 && x < 10 && y > 11 && y < 20) {
///     // x: [1, 9], y: [12, 19]
///     clang_analyzer_eval(x != y); // expected-warning{{TRUE}}
/// }
/// ```
pub struct Test<'a, 'b, T1: 'a, T2: 'a, T3: 'b, T4: 'b> {
    Conditional: &'a BinarySymExpr<'a, T1, T2>,
    Assertion: &'b BinarySymExpr<'b, T3, T4>, // Concrete types involved?
}

impl<'a, 'b, T1, T2, T3, T4> Test<'a, 'b, T1, T2, T3, T4> {
    pub fn new(
        Conditional: &'a BinarySymExpr<T1, T2>,
        Assertion: &'b BinarySymExpr<T3, T4>,
    ) -> Test<'a, 'b, T1, T2, T3, T4> {
        Test {
            Conditional,
            Assertion,
        }
    }
}

impl<'a, 'b, T1, T2, T3, T4> fmt::Display for Test<'a, 'b, T1, T2, T3, T4>
where
    T1: fmt::Display,
    T2: fmt::Display,
    T3: fmt::Display,
    T4: fmt::Display,
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

impl<'a, 'b, T1, T2, T3, T4> Draw for Test<'a, 'b, T1, T2, T3, T4>
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
}
