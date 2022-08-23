//! Symbol : A custom-typed symbol which stores name and statically available
//! type.

use crate::DataType::CDataTypes;
use crate::Draw::Draw;
use std::fmt;

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
            ty: CDataTypes::getType(dtype),
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

    type OutputType = Sym;
    fn iter(&self) -> (bool, &Self::OutputType) {
        println!("Base: {}", self);
        (true, self)
    }
}

// // TODO: Idiomatic way: Supertrait for collecting all integer types
// impl Draw for u32 {
//     fn dump(&self, tabl: Option<usize>) -> String {
//         format!("{ws}{}\n", self, ws = " ".repeat(tabl.unwrap_or(0)))
//     }

//     fn declare(&self) -> String {
//         format!("{} {}", self, self)
//     }

//     type OutputType = u32;
//     fn iter(&self) -> (bool, &Self::OutputType) {
//         println!("Base: {}", self);
//         (true, self)
//     }
// }

/// A concrete symbolc type.
pub struct Conc<'a, T> {
    pub ty: &'a CDataTypes,
    pub val: T,
}

impl<'a, T> Conc<'a, T> {
    // TODO: dtype can be of CDataTypes
    pub fn new(val: T, ty: &'a CDataTypes) -> Conc<T> {
        Conc {
            // ty: CDataTypes::getType(dtype),
            val,
            ty,
        }
    }
}

// TODO: Give logical operations.
// impl<T> PartialEq for Conc<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name && self.ty == other.ty
//     }
// }

impl<'a, T> fmt::Display for Conc<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<'a, T> Draw for Conc<'a, T>
where
    T: fmt::Display,
{
    fn dump(&self, tabl: Option<usize>) -> String {
        format!("{ws}{}\n", self.val, ws = " ".repeat(tabl.unwrap_or(0)))
    }

    fn declare(&self) -> String {
        // FIXME: There should not be a declare() method.
        format!("{} {}", self.ty, self.val)
    }

    type OutputType = Conc<'a, T>;
    fn iter(&self) -> (bool, &Self::OutputType) {
        println!("Base: {}", self);
        (true, self)
    }
}
