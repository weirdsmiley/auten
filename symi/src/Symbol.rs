//! Symbol : A custom-typed symbol which stores name and statically available
//! type.

use crate::DataType::CDataTypes;
use crate::Draw::Draw;
use std::fmt;

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
        #[cfg(debug_assertions)]
        println!("Symbol: {}", self);

        (true, self)
    }
}

/// A concrete symbolic type.
pub struct Conc<'a, T> {
    pub ty: &'a CDataTypes,
    pub val: T,
}

impl<'a, T> Conc<'a, T> {
    // TODO: dtype can be of CDataTypes or str, similarly in other new()
    // methods.
    pub fn new(val: T, ty: &'a CDataTypes) -> Conc<T> {
        Conc {
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
        format!("{}", self.val)
    }

    type OutputType = Conc<'a, T>;
    fn iter(&self) -> (bool, &Self::OutputType) {
        #[cfg(debug_assertions)]
        println!("Value: {}", self);

        (true, self)
    }
}
