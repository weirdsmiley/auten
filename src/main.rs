//! Fuzzing tests for Clang Static Analyzer
//! ---------------------------------------
//!
//! The aim is to construct a meta-enum storing all C data types, and
//! programmatically produce list of non-interfering test cases to be used by
//! the Clang Static Analyzer. These tests usually go in
//! [constant-folding](clang/test/Analysis/constant-folding.c) file.
//!
//! Refer Wiki for C data types
//! https://en.wikipedia.org/wiki/C_data_types#Main_types
//!
#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use symi::*;

/// Iterate over CDataTypes and each member will have two symbol names
/// associated with the type, (*1 and *2).
/// Returns a vector of those symbols.
pub(crate) fn set_of_syms() -> (Vec<Sym>, Vec<CDataTypes>) {
    let mut set: Vec<Sym> = vec![];
    let mut ty: Vec<CDataTypes> = vec![];

    set.push(Sym::new("ch1", "char"));
    set.push(Sym::new("ch2", "char"));
    ty.push(CDataTypes::Char);

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

// use atg::Symbol::Sym;
// use atg::DataType::CDataTypes;
// use atg::Draw::Draw;
// use atg::Expr::BinarySymExpr;
// use atg::BinOp::Opcode;
// use atg::Test::Test;

/// Generate symbols on the run and add them to file.
fn dump_testRemainedRules(test_file: &mut File) {
    let a = Sym::new("a", "unsigned int");
    let b = Sym::new("b", "unsigned int");
    let c = Sym::new("c", "int");
    let d = Sym::new("d", "int");
    let fn_name = "bar";

    fn_header(test_file, fn_name, &[&a, &b, &c, &d]);

    let cond1 = BinarySymExpr::new(&a, &30, &Opcode::LE);
    let cond2 = BinarySymExpr::new(&b, &50, &Opcode::LE);
    let amodb = BinarySymExpr::new(&a, &b, &Opcode::Rem);
    let bmoda = BinarySymExpr::new(&b, &a, &Opcode::Rem);
    let cond = BinarySymExpr::new(&cond1, &cond2, &Opcode::LAnd);
    let amodbl50 = BinarySymExpr::new(&amodb, &50, &Opcode::LT);
    let bmodal30 = BinarySymExpr::new(&bmoda, &30, &Opcode::LT);

    let tmp = BinarySymExpr::new(&cond, &amodbl50, &Opcode::Mul);
    println!("BSE = tmp\n{}", tmp.dump(None));

    let T1 = Test::new(&cond, &amodbl50);
    test_file.write_fmt(format_args!("{}", T1)).unwrap();
}

/// This will generate all corner cases corresponding to S1 and S2. These two
/// symbols may be of different types. It should not matter in the construction
/// of test cases. All we need to do is construct corner cases associated to
/// these two symbols. Also, put assert expressions in the file.
fn dump_all_corner_tests(test_file: &mut File, S1: &Sym, S2: &Sym, Op: &Opcode) {
    let BSE = BinarySymExpr::new(S1, S2, Op);
    // We have the BSE, an expression for which we need to generate test cases.
    // That is, we first need to construct constraints using < family of
    // operators.
    let BSE2 = BinarySymExpr::new(S1, S2, &Opcode::LE);
    let BSE3 = BinarySymExpr::new(&BSE, &BSE2, &Opcode::LAnd);
    let BSE4 = BinarySymExpr::new(&BSE, &BSE2, &Opcode::LOr);
    let BSE5 = BinarySymExpr::new(&BSE3, &BSE4, &Opcode::Xor);

    // println!("{}", BSE5.dump());

    // Logic
    // -----
    // We have two symbols of types t1 and t2, so we can figure out the limits
    // of those types respectively.
    //
    // All test cases comprise of values around those limits. Example,
    // t1 -> int and t2 -> unsigned int
    // t1 ∈ [INT_MIN, INT_MAX]
    // t2 ∈ [0, UINT_MAX]

    let test = Test::new(&BSE3, &BSE);
    test_file.write_fmt(format_args!("{}", test)).unwrap();
}

fn search_pair_of_types<'a>(
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

fn fn_header(test_file: &mut File, fn_name: &str, syms: &[&Sym]) {
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
            .write_fmt(format_args!("{}, ", sym.declare()))
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
fn cntrl_headers(test_file: &mut File, head: bool) {
    if head {
        test_file
            .write_fmt(format_args!(
                "
void clang_analyzer_eval(int);

int foo(char ch1, char ch2,
        signed char sch1, signed char sch2,
        unsigned char uch1, unsigned char uch2,
        short sh1, short sh2,
        unsigned short ush1, unsigned short ush2,
        int s1, int s2,
        unsigned int u1, unsigned int u2
        ) {{
"
            ))
            .unwrap();
    } else {
        test_file
            .write_fmt(format_args!(
                "
  return 0;
}}"
            ))
            .unwrap();
    }
}

/// Main driver for constructing symbols and their associated constraints and
/// build test cases, put them in a .c file.
fn fuzz() -> bool {
    let mut test = File::create("fuzzed-tests.c").unwrap();

    let mut test2 = File::create("testRemainedRules.c").unwrap();
    dump_testRemainedRules(&mut test2);
    cntrl_headers(&mut test2, false);
    println!("Fuzzed tests in testRemainedRules.c");

    // TODO: For generating tests crossing all types and ranges, we can use the
    // following method of generating symbols and cross-producting with the
    // available CDataTypes. We should also provide an API to construct symbols
    // in bunch and these symbols go straight to function header.
    // cntrl_headers(&mut test, true);

    // Set of symbols for fuzzing. Every data type in C will declare two symbols
    // namely, *1 and *2 (* denoting the data type's initials).
    let (Symset, AvailableTypes) = set_of_syms();
    // TODO: Either generalize fn_header or stick to something concrete!
    let RefSymset: Vec<&Sym> = Symset.iter().map(|S| S).collect();

    fn_header(&mut test, "foo", &RefSymset[..]);

    let Op = Opcode::NE;

    // Each pair with same type.
    for ty in &AvailableTypes {
        let (S1, S2) = search_pair_of_types(&Symset, ty, ty).unwrap();
        dump_all_corner_tests(&mut test, S1, S2, &Op); // dump to fuzzed-tests.c
    }

    // Handle for pairs in moving order. First type with rest types, second type
    // with rest types except first, and so on.
    for ty1_id in 0..AvailableTypes.len() {
        for ty2_id in ty1_id + 1..AvailableTypes.len() {
            let (S1, S2) =
                search_pair_of_types(&Symset, &AvailableTypes[ty1_id], &AvailableTypes[ty2_id])
                    .unwrap();

            dump_all_corner_tests(&mut test, S1, S2, &Op);
        }
    }

    cntrl_headers(&mut test, false);

    true
}

fn main() {
    if !fuzz() {
        eprintln!("Failed to fuzz tests. Aborting");
        return;
    }

    println!("Fuzzed tests in fuzzed-tests.c");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}