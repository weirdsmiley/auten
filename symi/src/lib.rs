#![allow(non_snake_case)]

//! symi: fuzz symbolic expressions
//!
//! This crate provides methods to generate symbolic expressions based on C
//! grammar.
//!
//! # Examples
//! ```
//! use symi::Symbol::{Sym, Conc};
//! use symi::DataType::CDataTypes;
//! use symi::Expr::BinarySymExpr;
//! use symi::BinOp::Opcode;
//!
//! fn main() {
//!     let sym  = Sym::new("x", "unsigned int");
//!     let conc = Conc::new(42, CDataTypes::Int);
//!     let bse  = BinarySymExpr::new(&sym, &conc, Opcode::LAnd);
//!     println!("{sym}");
//!     println!("{conc}");
//!     println!("{bse}");
//! }
//! ```

// auten is a fuzzer (initially for CSA) but generally for C programs. And a
// fuzzer requires two components: symbolic representation, and probabilistic
// methods to fuzz those symbols. symi should be the one true representation of
// symbols. Every other crate can use it to generate symbols in a determined
// manner. And these other crates can also pass a probabilistic model (how?)
// according to which symi will generate these symbols.

pub mod BinOp;
pub mod DataType;
pub mod Draw;
pub mod Expr;
pub mod Symbol;
pub mod Test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symi() {
        let sym = Symbol::Sym::new("x", "unsigned int");
        let conc = Symbol::Conc::new(42, DataType::CDataTypes::Int);
        let bse = Expr::BinarySymExpr::new(&sym, &conc, BinOp::Opcode::LAnd);
        let bse2 = Expr::BinarySymExpr::new(&bse, &sym, BinOp::Opcode::Mul);
        let _test = Test::Test::new(&bse, &bse2);
    }
}
