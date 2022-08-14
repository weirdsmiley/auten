#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

// If there are n possible types (for which assertions are to be constructed),
// then total number of assertions will be:
//      n (n + 1)
//      ─────────
//          2
//
// There will be 'n' assertions for each pair of variables in type i (1≤i≤n).
// For each type i, there will be pairs with other types (constrained previously
// types are not paired).
// TODO: Rename to _builtin (will provide custom data type interface later)
#[derive(PartialEq)]
pub enum CDataTypes {
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    // TODO
    // long, unsigned long, long long, unsigned long long( int), float, double,
    // long double
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    LongDouble,
}

impl fmt::Display for CDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CDataTypes::Char => write!(f, "char"),
            CDataTypes::SignedChar => write!(f, "signed char"),
            CDataTypes::UnsignedChar => write!(f, "unsigned char"),
            CDataTypes::Short => write!(f, "short"),
            CDataTypes::UnsignedShort => write!(f, "unsigned short"),
            CDataTypes::Int => write!(f, "int"),
            CDataTypes::UnsignedInt => write!(f, "unsigned int"),
            CDataTypes::Long => write!(f, "long"),
            CDataTypes::UnsignedLong => write!(f, "unsigned long"),
            CDataTypes::LongLong => write!(f, "long long"),
            CDataTypes::UnsignedLongLong => write!(f, "unsigned long long"),
            CDataTypes::Float => write!(f, "float"),
            CDataTypes::Double => write!(f, "double"),
            CDataTypes::LongDouble => write!(f, "long double"),
        }
    }
}

/// Binary opcodes which BinarySymExpr are allowed to take into account.
// TODO: Rename to BinOp
pub enum Opcode {
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Shl,
    Shr,
    LT,
    GT,
    LE,
    GE,
    EQ,
    NE,
    And,
    Xor,
    Or,
    LAnd,
    LOr,
    Assign,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Rem => write!(f, "%"),
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
            Opcode::Shl => write!(f, "<<"),
            Opcode::Shr => write!(f, ">>"),
            Opcode::LT => write!(f, "<"),
            Opcode::GT => write!(f, ">"),
            Opcode::LE => write!(f, "<="),
            Opcode::GE => write!(f, ">="),
            Opcode::EQ => write!(f, "=="),
            Opcode::NE => write!(f, "!="),
            Opcode::And => write!(f, "&"),
            Opcode::Xor => write!(f, "^"),
            Opcode::Or => write!(f, "|"),
            Opcode::LAnd => write!(f, "&&"),
            Opcode::LOr => write!(f, "||"),
            Opcode::Assign => write!(f, "="),
        }
    }
}

/// Handle all kinds of formatting apart from fmt::Display here.
// Rust allows to implement two traits with two exactly same methods(and
// its signatures) but it prefers one over the other.
//
// GH Issue: https://github.com/rust-lang/rust/issues/26080
// Docs:
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
// https://doc.rust-lang.org/std/fmt/trait.Write.html
pub trait Draw: fmt::Display {
    /// Dump type in a tree-like structure.
    // TODO : Implementing Write trait can be beneficial. There are two traits
    // one in fmt::Write, other in io::Write
    // https://doc.rust-lang.org/std/fmt/trait.Write.html
    fn dump(&self, tabl: Option<usize>) -> String;

    /// Dump type's declaration(s) into f Formatter.
    fn declare(&self) -> String;

    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

// TODO: Concrete types are working at least on face-value level. Use something
// like
//      BinarySymExpr::new(&Symbol, &50, Opcode::LE)
//
// So we should not worry about this struct.
//
// 1. But before that, check in debugger if &50 like literals are being assigned
//    correct type. What is the correct type?
//    &50 is of no CDataType, instead it is just a literal.
//
// 2. Should this be dependent on type of Symbol?
//
// /// A concrete type for denoting numerals.
// struct Conc<T> {
//     ty: CDataTypes,
//     name: T, // these are concrete values
// }

// // This template is for rust data types. We need a C data types. Those types
// // should store its limits _MAX, _MIN.
// // impl<T> Conc<T> {
// //     fn new(name: T
// // }

/// A symbol to reference any declaration.
pub struct Sym {
    pub ty: CDataTypes,
    pub name: String,
}

impl Sym {
    pub fn new(name: &str, dtype: &str) -> Sym {
        Sym {
            name: name.to_string(),
            ty: match dtype {
                "char" => CDataTypes::Char,
                "signed char" => CDataTypes::SignedChar,
                "unsigned char" => CDataTypes::UnsignedChar,
                "short" => CDataTypes::Short,
                "unsigned short" => CDataTypes::UnsignedShort,
                "int" => CDataTypes::Int,
                "signed int" => CDataTypes::Int,
                "unsigned int" => CDataTypes::UnsignedInt,
                "long" => CDataTypes::Long,
                "unsigned long" => CDataTypes::UnsignedLong,
                "long long" => CDataTypes::LongLong,
                "unsigned long long" => CDataTypes::UnsignedLongLong,
                "float" => CDataTypes::Float,
                "double" => CDataTypes::Double,
                "long double" => CDataTypes::LongDouble,
                _ => CDataTypes::Char,
            },
        }
    }
}

impl PartialEq for Sym {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ty == other.ty
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Draw for Sym {
    fn dump(&self, tabl: Option<usize>) -> String {
        format!("{ws}{}\n", self.name, ws = " ".repeat(tabl.unwrap_or(0)))
    }

    fn declare(&self) -> String {
        format!("{} {}", self.ty, self.name)
    }
}

// TODO: Idiomatic way: Supertrait for collecting all integer types
impl Draw for u32 {
    fn dump(&self, tabl: Option<usize>) -> String {
        format!("{ws}{}\n", self, ws = " ".repeat(tabl.unwrap_or(0)))
    }

    fn declare(&self) -> String {
        format!("{} {}", self, self)
    }
}

pub struct BinarySymExpr<'a, T1, T2> {
    LHS: &'a T1,
    RHS: &'a T2,
    Op: &'a Opcode,
}

//      Sym     Sym           Sym     Sym  <--- constructed in heap
//      'w'     'x'           'y'     'z'
//        \     /               \     /
//         \   /                 \   /
//          \ /                   \ /
//     BinarySymExpr         BinarySymExpr <--- while constructing, we should
//       'BSE<Sym>'            'BSE<Sym>'       create boxes of Sym and put
//         w != x                y != z         those boxes as member vals
//            \                   /
//             \                 /
//              \               /
//               \             /
//                \  Op : &&  /
//        BinarySymExpr<BinarySymExpr<Sym>> <--- while constructing, pick
//            (w != x) && (y != z)               above two members and box them
//                                               then put them as member vals
//
// Essentially, we need to box values inside the new() method, and not expect
// that boxed values will be passed to new(). We are returning
// BinarySymExpr<T> and not BinarySymExpr<Box<T>>.
impl<'a, T1, T2> BinarySymExpr<'a, T1, T2> {
    pub fn new(LHS: &'a T1, RHS: &'a T2, Op: &'a Opcode) -> BinarySymExpr<'a, T1, T2> {
        BinarySymExpr { LHS, RHS, Op }
    }
}

impl<'a, T1, T2> fmt::Display for BinarySymExpr<'a, T1, T2>
where
    T1: fmt::Display,
    T2: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", *self.LHS, self.Op, *self.RHS)
    }
}

impl<'a, T1, T2> Draw for BinarySymExpr<'a, T1, T2>
where
    T1: Draw,
    T2: Draw,
{
    /// Print BinarySymExpr in a tree-like structure. This is for debugging
    /// purposes and differs from fmt::Display as that method is for generating
    /// cases.
    fn dump(&self, tabl: Option<usize>) -> String {
        format!(
            "{}{}\n{}",
            self.LHS.dump(Some(tabl.unwrap_or(0) + 4)),
            format!("{ws}{}", self.Op, ws = " ".repeat(tabl.unwrap_or(0))),
            self.RHS.dump(Some(tabl.unwrap_or(0) + 4))
        )
    }

    fn declare(&self) -> String {
        // A BinarySymExpr is complex type. We need to distinguish between Sym
        // and BinarySymExpr<T> (where T ≠ Sym). We should consider case where
        // to put semicolon and comma.
        format!("")
    }
}


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
    fn dump(&self, tabl: Option<usize>) -> String {
        // println!("This is a test from dump method");
        // format!("This is a test from dump method");
        "This is a test from dump method".to_string()
    }

    fn declare(&self) -> String {
        format!("")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
