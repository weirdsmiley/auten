//! Fuzzing tests for Clang Static Analyzer
//! ---------------------------------------
//!
//! The aim is to construct a meta-enum storing all C data types, and
//! probabilistically produce list of non-interfering test cases to be used by
//! the Clang Static Analyzer. These tests usually go in
//! [constant-folding](clang/test/Analysis/constant-folding.c) file.
//!
//! Refer Wiki for C data types
//! https://en.wikipedia.org/wiki/C_data_types#Main_types
//!
#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
use std::fs::File;
use std::io::{prelude::*, stdin, stdout, Error};
use std::process::exit;
use std::sync::Arc;

use symi::BinOp::Opcode;
use symi::DataType::CDataTypes;
use symi::Draw::Draw;
use symi::Expr::{BinarySymExpr, ChainedBSE};
use symi::Symbol::{Conc, Sym};
use symi::Test::Test;

mod utils;
use utils::*;

/// Generate symbols on the run and add them to file.
fn dump_testRemainedRules(test_file: &mut File) {
    let a = Sym::new("a", "unsigned int");
    let b = Sym::new("b", "unsigned int");
    let c = Sym::new("c", "int");
    let d = Sym::new("d", "int");
    let conc30 = Conc::new(30, CDataTypes::Int);
    let conc50 = Conc::new(50, CDataTypes::Int);
    let fn_name = "bar";

    let sub: Vec<Arc<Sym>> = [&a, &b, &c, &d]
        .iter()
        .map(|sym| Arc::clone(&sym))
        .collect();

    test_header(test_file, fn_name, &sub);

    let cond1 = BinarySymExpr::new(&a, &conc30, Opcode::LE);
    let cond2 = BinarySymExpr::new(&b, &conc50, Opcode::LE);
    let amodb = BinarySymExpr::new(&a, &b, Opcode::Rem);
    let bmoda = BinarySymExpr::new(&b, &a, Opcode::Rem);
    let cond = BinarySymExpr::new(&cond1, &cond2, Opcode::LAnd);
    let amodbl50 = BinarySymExpr::new(&amodb, &conc50, Opcode::LT);
    let bmodal30 = BinarySymExpr::new(&bmoda, &conc30, Opcode::LT);

    let tmp = BinarySymExpr::new(&cond, &amodbl50, Opcode::Mul);
    #[cfg(debug_assertions)]
    println!("BSE = tmp\n{}", tmp.dump(None));

    let T1 = Test::new(&cond, &amodbl50);
    test_file.write_fmt(format_args!("{}", T1)).unwrap();
}

/// This will generate all corner cases corresponding to S1 and S2. These two
/// symbols may be of different types. It should not matter in the construction
/// of test cases. All we need to do is construct corner cases associated to
/// these two symbols. Also, put assert expressions in the file.
fn dump_all_corner_tests(test_file: &mut File, S1: &Arc<Sym>, S2: &Arc<Sym>, Op: Opcode) {
    let BSE = BinarySymExpr::new(S1, S2, Op);
    // We have the BSE, an expression for which we need to generate test cases.
    // That is, we first need to construct constraints using < family of
    // operators.
    let BSE2 = BinarySymExpr::new(S1, S2, Opcode::LE);
    let BSE3 = BinarySymExpr::new(&BSE, &BSE2, Opcode::LAnd);
    let BSE4 = BinarySymExpr::new(&BSE, &BSE2, Opcode::LOr);
    let BSE5 = BinarySymExpr::new(&BSE3, &BSE4, Opcode::Xor);

    // println!("{}", BSE5.dump(None));

    // Logic
    // -----
    // We have two symbols of types t1 and t2, so we can figure out the limits
    // of those types respectively.
    //
    // All test cases comprise of values around those limits. Example,
    // t1 -> int and t2 -> unsigned int
    // t1 ∈ [INT_MIN, INT_MAX]
    // t2 ∈ [0, UINT_MAX]

    if S1.isa(S2.getType()) {
        // All possible test cases
        // <---|------------------------|--------------------|--->
        //    T_MIN                   T_MID                T_MAX
        //
        // For (T_MIN, T_MAX):
        //   1. Non overlapping        : [1,2] and [3,4]
        //   2. Partially overlapping  : [1,3] and [2,4]
        //   3. Completely overlapping : [1,3] and [1,3]
        // For overflows  : ...same three cases
        // For underflows : ...same three cases
        let (T_MIN, T_MAX) = S1.getTypeRange();
        let T_MID = (T_MIN >> 1) + (T_MAX >> 1) + (((T_MIN & 1) + (T_MAX & 1)) >> 1);

        let BSE = Sym::getConstraintsAround(S1, T_MID, 3);
        println!("BSE = {}", BSE);

        let (T_MID_LL, T_MID_LR) = (
            Conc::new(T_MID - 3, S1.getType()),
            Conc::new(T_MID - 1, S1.getType()),
        );
        let (T_MID_RL, T_MID_RR) = (
            Conc::new(T_MID + 1, S1.getType()),
            Conc::new(T_MID + 3, S1.getType()),
        );

        let C1 = BinarySymExpr::new(S1, &T_MID_LL, Opcode::GE);
        let C2 = BinarySymExpr::new(S1, &T_MID_LR, Opcode::LE);
        let C3 = BinarySymExpr::new(S2, &T_MID_RL, Opcode::GE);
        let C4 = BinarySymExpr::new(S2, &T_MID_RR, Opcode::LE);

        let T_MID_L = Conc::new(T_MID - 3, S1.getType());
        let T_MID_R = Conc::new(T_MID + 3, S1.getType());

        let LHS = BinarySymExpr::new(S1, &T_MID_L, Opcode::LE);
        let RHS = BinarySymExpr::new(S2, &T_MID_R, Opcode::GE);

        // let ThisConditional = ChainedBSE::new(&[&C1, &C2, &C3, &C4], Op).join();

        let ThisConditional = BinarySymExpr::new(&LHS, &RHS, Opcode::LAnd);
        ThisConditional.iter();

        let ThisAssert = BinarySymExpr::new(S1, S2, Op);
        let ThisTest = Test::new(&ThisConditional, &ThisAssert);
        test_file.write_fmt(format_args!("{}", ThisTest)).unwrap();
    } else if S1.getType() < S2.getType() {
        #[cfg(debug_assertions)]
        println!("{} < {}", S1.declare(), S2.declare());
    } else {
        #[cfg(debug_assertions)]
        println!("{} > {}", S1.declare(), S2.declare());
    }

    // let test = Test::new(&BSE3, &BSE);
    // test_file.write_fmt(format_args!("{}", test)).unwrap();
}

/// Main driver for constructing symbols and their associated constraints and
/// build test cases, put them in a .c file.
fn fuzz(Op: Opcode) -> Result<(), Error> {
    // TODO: Rename files if already exists.
    let mut test = File::create("fuzzed-tests.c").unwrap();

    // let mut test2 = File::create("testRemainedRules.c").unwrap();
    // dump_testRemainedRules(&mut test2);
    // test_footer(&mut test2);
    // println!("Fuzzed tests in testRemainedRules.c");

    // Set of symbols for fuzzing. Every data type in C will declare two symbols
    // namely, *1 and *2 (* denoting the data type's initials).
    let (Symset, AvailableTypes) = set_of_syms();

    test_header(&mut test, "foo", &Symset[..]);

    // Combine each pair with same type.
    for ty in AvailableTypes.iter() {
        let (S1, S2) = search_pair_of_types(&Symset, *ty, *ty).unwrap();
        dump_all_corner_tests(&mut test, &S1, &S2, Op.clone());
    }

    // Handle for pairs in moving order. First type with rest types, second type
    // with rest types except first, and so on.
    for ty1_id in 0..AvailableTypes.len() {
        for ty2_id in ty1_id + 1..AvailableTypes.len() {
            let (S1, S2) =
                search_pair_of_types(&Symset, AvailableTypes[ty1_id], AvailableTypes[ty2_id])
                    .unwrap();

            dump_all_corner_tests(&mut test, &S1, &S2, Op);
        }
    }

    test_footer(&mut test);

    Ok(())
}

fn main() {
    let mut input = String::with_capacity(5);
    loop {
        input.clear();
        print!("> Enter binary operator for which to generate tests\n> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("IO error");

        if "q\n" == input || "quit\n" == input {
            exit(0);
        }

        if let Some(opcode) = Opcode::getOpcode(input.as_str().strip_suffix('\n').unwrap()) {
            match fuzz(opcode) {
                Ok(_) => println!("Fuzzed tests for {} operator.", opcode),
                Err(_) => eprintln!("Failed to fuzz tests. Aborted!"),
            }
        } else {
            println!("Incorrect opcode! 'quit' to exit.");
        }
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
