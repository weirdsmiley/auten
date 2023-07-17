//! This crate provides two structures: a symbolic value and a concrete value.
//!
//! * Symbolic: contains a named symbol, an associated C type
//! * Concrete: contains a literal value with determined type
//!
//! Each API call will create a symbol and store it in the Symbol Table. The
//! users will be returned a reference to this symbol. They can write a
//! probabilistic model to randomnly fuzz this symbol again in a test case. If
//! they want to create some complex expression using this already generated
//! symbol, then they can call any respective complex expression generator (in
//! symi) by this symbol reference. The complex expression generator
//! (ChainedBSE is one of them) will use this reference, lookup the symbol
//! table, and use Arc::clone to create a new expression value which will also
//! own the original symbol. This newly generated complex expression
//! asynchronously owns the original symbol, so that during the fuzzing part, we
//! can separately fuzz the symbol and complex expression.

use crate::BinOp::Opcode;
use crate::DataType::CDataTypes;
use crate::Draw::Draw;
use crate::Expr::BinarySymExpr;
use std::fmt;
use std::sync::Arc;

/// A symbol to reference any declaration.
pub struct Sym {
    pub ty: CDataTypes,
    pub name: String,
    // TODO: we need a range member
}

/// A binary expression with a symbol and a concrete value.
type SimpleBSE = BinarySymExpr<Sym, Conc<i64>>;

impl Sym {
    /// Returns a new symbolic object with said C data type. If no type is
    /// provided, then it sets the symbol to a signed integer (the default `int`
    /// in C).
    #[inline]
    pub fn new(name: &str, dtype: &str) -> Arc<Self> {
        Arc::new(Sym {
            name: name.to_string(),
            ty: CDataTypes::getType(dtype).unwrap_or(CDataTypes::Int),
        })
    }

    pub fn isa(&self, ty: CDataTypes) -> bool {
        return self.ty == ty;
    }

    pub fn getName(&self) -> &String {
        return &self.name;
    }

    pub fn getType(&self) -> CDataTypes {
        return self.ty;
    }

    pub fn getTypeRange(&self) -> (i64, i64) {
        self.ty.getRange()
    }

    /// Construct a binary symbolic expression for symbol S, constrained from
    /// both sides around a `pivot`, and with a width of `2 * away`, `away` from
    /// each side.
    ///
    /// ```text
    ///         pivot - away <= s <= pivot + away
    /// ```
    pub fn getConstraintsAround<'a>(
        sym: &Arc<Sym>,
        pivot: i64,
        away: i64,
    ) -> Arc<BinarySymExpr<SimpleBSE, SimpleBSE>> {
        let (T_MIN, T_MAX) = sym.ty.getRange();

        if T_MIN <= (pivot - away) && (pivot + away) <= T_MAX {
            let conc_before = Conc::new(pivot - away, sym.ty);
            let conc_after = Conc::new(pivot + away, sym.ty);
            let lhs = BinarySymExpr::new(&sym, &conc_before, Opcode::GE);
            let rhs = BinarySymExpr::new(&sym, &conc_after, Opcode::LE);
            BinarySymExpr::new(&lhs, &rhs, Opcode::LAnd)
        } else {
            let conc_before = Conc::new(pivot - away, sym.ty);
            let conc_after = Conc::new(pivot + away, sym.ty);
            let lhs = BinarySymExpr::new(&sym, &conc_before, Opcode::GE);
            let rhs = BinarySymExpr::new(&sym, &conc_after, Opcode::LE);
            BinarySymExpr::new(&lhs, &rhs, Opcode::LAnd)
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
pub struct Conc<T> {
    pub ty: CDataTypes,
    pub val: T,
}

impl<T> Conc<T> {
    // TODO: dtype can be of CDataTypes or str, similarly in other new()
    // methods.
    pub fn new(val: T, ty: CDataTypes) -> Arc<Conc<T>> {
        Arc::new(Conc { ty, val })
    }

    // pub fn new(val: T, dtype: &str) -> Conc<T> {
    //     Conc(Arc::new(ConcInner {
    //         ty: CDataTypes::getType(dtype).unwrap_or(CDataTypes::Int),
    //         val
    //     }))
    // }
}

// // TODO: Give logical operations.
// impl<T> PartialEq for Conc<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.val == other.0.val && self.0.ty == other.0.ty
//     }
// }

impl<T> fmt::Display for Conc<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<T> Draw for Conc<T>
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

    type OutputType = Conc<T>;
    fn iter(&self) -> (bool, &Self::OutputType) {
        #[cfg(debug_assertions)]
        println!("Value: {}", self);

        (true, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Sym() {
        let s = Sym::new("x", "unsigned int");
        assert!(s.name == "x" && s.ty == CDataTypes::UnsignedInt);
    }

    #[test]
    fn test_getConstraintsAround() {
        let s = Sym::new("x", "unsigned int");
        let _b = Sym::getConstraintsAround(&s, 0, 1);
        println!("{}", _b);
        println!("{}", s);
    }
}
