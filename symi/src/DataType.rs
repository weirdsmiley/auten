//! DataType : All available data types in C are structured here.
//!

use std::fmt;

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
