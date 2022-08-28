use std::fs::File;
use std::io::prelude::*;

use symi::BinOp::Opcode;
use symi::DataType::CDataTypes;
use symi::Draw::Draw;
use symi::Expr::BinarySymExpr;
use symi::Symbol::{Conc, Sym};
use symi::Test::Test;

type SimpleBSE<'a> = BinarySymExpr<'a, Sym, Conc<'a, i64>>;

/// Construct a binary symbolic expression for symbol S, constrained from both
/// sides of T_VAL, Away far.
///         T_VAL - Away <= S <= T_VAL + Away
pub(crate) fn getConstraintsAround(
    S: &Sym,
    T_VAL: i64,
    Away: i64,
) -> Option<BinarySymExpr<SimpleBSE, SimpleBSE>> {
    let (T_MIN, T_MAX) = S.ty.getRange();

    if T_MIN <= (T_VAL - Away) && (T_VAL + Away) <= T_MAX {
        let C1 = Conc::new(T_VAL - Away, &S.ty);
        let C2 = Conc::new(T_VAL + Away, &S.ty);
        let BSE1 = BinarySymExpr::new(S, &C1, &Opcode::GE);
        let BSE2 = BinarySymExpr::new(S, &C2, &Opcode::LE);
        let BSE = BinarySymExpr::new(&BSE1, &BSE2, &Opcode::LAnd);

        None
    } else {
        // wrap around the range
        None
    }
}

/// Iterate over CDataTypes and each member will have two symbol names
/// associated with the type, (*1 and *2).
/// Returns a vector of those symbols.
pub(crate) fn set_of_syms() -> (Vec<Sym>, Vec<CDataTypes>) {
    let mut set: Vec<Sym> = vec![];
    let mut ty: Vec<CDataTypes> = vec![];

    // set.push(Sym::new("ch1", "char"));
    // set.push(Sym::new("ch2", "char"));
    // ty.push(CDataTypes::Char);

    set.push(Sym::new("sch1", "signed char"));
    set.push(Sym::new("sch2", "signed char"));
    ty.push(CDataTypes::SignedChar);

    set.push(Sym::new("uch1", "unsigned char"));
    set.push(Sym::new("uch2", "unsigned char"));
    ty.push(CDataTypes::UnsignedChar);

    set.push(Sym::new("sh1", "short"));
    set.push(Sym::new("sh2", "short"));
    ty.push(CDataTypes::Short);

    set.push(Sym::new("ush1", "unsigned short"));
    set.push(Sym::new("ush2", "unsigned short"));
    ty.push(CDataTypes::UnsignedShort);

    set.push(Sym::new("u1", "unsigned int"));
    set.push(Sym::new("u2", "unsigned int"));
    ty.push(CDataTypes::UnsignedInt);

    set.push(Sym::new("s1", "signed int"));
    set.push(Sym::new("s2", "signed int"));
    ty.push(CDataTypes::Int);

    (set, ty)
}

/// Given a vector of symbols, find first pair of symbols for certain given
/// types.
pub(crate) fn search_pair_of_types<'a>(
    Symset: &'a Vec<Sym>,
    ty1: &CDataTypes,
    ty2: &CDataTypes,
) -> Option<(&'a Sym, &'a Sym)> {
    for sym1 in Symset {
        if sym1.ty == *ty1 {
            for sym2 in Symset {
                if sym1 != sym2 && sym2.ty == *ty2 {
                    return Some((sym1, sym2));
                }
            }
        }
    }

    None
}

/// Main routine to dump headers for test file.
pub(crate) fn test_header(test_file: &mut File, fn_name: &str, syms: &[&Sym]) {
    test_file
        .write_fmt(format_args!(
            "
void clang_analyzer_eval(int);

int {}(",
            fn_name
        ))
        .unwrap();

    let mut it = syms.iter().peekable();
    while let Some(sym) = it.next() {
        // TODO: Optimize this! This condition is true for last element only.
        // Check if peek() is alread optimized.
        if it.peek().is_none() {
            test_file
                .write_fmt(format_args!("{}", sym.declare()))
                .unwrap();
            continue;
        }
        test_file
            .write_fmt(format_args!("\n\t{}, ", sym.declare()))
            .unwrap();
    }

    test_file.write_fmt(format_args!(") {{")).unwrap();
}

// TODO: We need more flexibility while inserting headers and functions.
// 1. We need variable-length arguments in functions. Eg:
//      int foo(int a, int b) // <- Custom declarations, instead of fixing 7x8
//
// 2. More asserts should be allowable inside one conditional. For this, will we
//    have to separate Conditional and Assertion in Test? Eg:
//      if () { clang_analyzer_eval(assert1); clang_analyzer_eval(assert2); }
//
// 3. Constructing symbol declarations at any point will be helpful.
/// Dump function header and return values (if any) for the to-be tested
/// function.
// TODO : Redundant, replace with fn_header
pub(crate) fn test_footer(test_file: &mut File) {
    test_file
        .write_fmt(format_args!(
            "
  return 0;
}}"
        ))
        .unwrap();
}
