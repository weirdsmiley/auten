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
#[derive(Copy, Clone, PartialEq, PartialOrd)]
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

impl CDataTypes {
    pub fn getType(dtype: &str) -> Option<CDataTypes> {
        return match dtype {
            "char" => Some(CDataTypes::Char),
            "signed char" => Some(CDataTypes::SignedChar),
            "unsigned char" => Some(CDataTypes::UnsignedChar),
            "short" => Some(CDataTypes::Short),
            "unsigned short" => Some(CDataTypes::UnsignedShort),
            "int" => Some(CDataTypes::Int),
            "signed int" => Some(CDataTypes::Int),
            "unsigned int" => Some(CDataTypes::UnsignedInt),
            "long" => Some(CDataTypes::Long),
            "unsigned long" => Some(CDataTypes::UnsignedLong),
            "long long" => Some(CDataTypes::LongLong),
            "unsigned long long" => Some(CDataTypes::UnsignedLongLong),
            "float" => Some(CDataTypes::Float),
            "double" => Some(CDataTypes::Double),
            "long double" => Some(CDataTypes::LongDouble),
            _ => None,
        };
    }

    // FIXME: We want to return tuples of various types.
    // 1. Use Option
    //
    // Instead of returning builtin types, we can return Conc<T> and implement
    // shifting operators over those types.
    pub fn getRange(&self) -> (i64, i64) {
        match self {
            CDataTypes::Char => (0, u8::MAX as i64), // FIXME: Char's type is
            // dependent and we need to
            // redefine it based on
            // unsigned/signed-ness
            // Look at limits.h
            CDataTypes::SignedChar => (i8::MIN as i64, i8::MAX as i64),
            CDataTypes::UnsignedChar => (0, u8::MAX as i64),
            CDataTypes::Short => (i16::MIN as i64, i16::MAX as i64),
            CDataTypes::UnsignedShort => (u16::MIN as i64, u16::MAX as i64),
            CDataTypes::Int => (i32::MIN as i64, i32::MAX as i64),
            CDataTypes::UnsignedInt => (u32::MIN as i64, u32::MAX as i64),
            _ => unreachable!(),
        }
    }
}

// impl PartialEq for CDataTypes {
//     fn eq(&self, other: &Self) -> bool {
//         self.getRange() == other.getRange()
//     }

//     fn ne(&self, other: &Self) -> bool {
//         self.getRange() != other.getRange()
//     }
// }

// TODO: Can we simply implement a method in impl
// pub trait Range {
//     fn getRange(&self) -> (i64, i64);
//     // fn getMax(&self) -> Option<(T,T)>;
// }

// impl Range for CDataTypes {
//     fn getRange(&self) -> (i64, i64) {
//         match self {
//             CDataTypes::Char => (0, u8::MAX as i64),
//             CDataTypes::Int => (i32::MIN as i64, i32::MAX as i64),
//             CDataTypes::Short => (u16::MIN as i64, u16::MAX as i64),
//             CDataTypes::SignedChar => (0, 0),
//             CDataTypes::UnsignedChar => (0, 0),
//             CDataTypes::UnsignedInt => (0, 0),
//             CDataTypes::UnsignedShort => (0, 0),
//             _ => (u32::MIN as i64, u32::MAX as i64)
//         }
//     }
// }

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
