//! Expr : An expression type. This may be extended for UnarySymExprs as well.

use crate::BinOp::Opcode;
use crate::Draw::Draw;
use std::fmt;
use std::sync::Arc;

// impl<'a, T1, T2> RecursiveIter for BinarySymExpr<'a, T1, T2>
// where
//     T1: RecursiveIter,
//     T2: RecursiveIter,
// {
// }

pub struct BinarySymExpr<T1, T2> {
    LHS: Arc<T1>,
    RHS: Arc<T2>,
    Op: Opcode,
}

// pub struct BinarySymExpr<T1, T2>(Arc<BinarySymExprInner<T1, T2>>);

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
impl<T1, T2> BinarySymExpr<T1, T2> {
    pub fn new(LHS: &Arc<T1>, RHS: &Arc<T2>, Op: Opcode) -> Arc<Self> {
        Arc::new(Self {
            LHS: Arc::clone(LHS),
            RHS: Arc::clone(RHS),
            Op,
        })
    }
}

impl<T1, T2> fmt::Display for BinarySymExpr<T1, T2>
where
    T1: fmt::Display,
    T2: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.LHS, self.Op, self.RHS)
    }
}

impl<T1, T2> Draw for BinarySymExpr<T1, T2>
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

    type OutputType = BinarySymExpr<T1, T2>;
    fn iter(&self) -> (bool, &Self::OutputType) {
        #[cfg(debug_assertions)]
        println!("BSE: {}", self);

        let (isLHSBase, _LHS) = T1::iter(&self.LHS);
        let (isRHSBase, _RHS) = T2::iter(&self.RHS);

        // TODO: Do something with LHS and RHS
        (isLHSBase && isRHSBase, self)
    }
}

/// A collection of BSE<T1, T2> joined by a single logical binary operator.
///
/// # Examples
///
/// ```text
///
/// ```
pub struct ChainedBSE<'a, T1, T2> {
    Op: Opcode,
    BSEs: &'a [BinarySymExpr<T1, T2>],
}

impl<'a, T1: fmt::Display, T2: fmt::Display> ChainedBSE<'a, T1, T2> {
    // Variable number of BSEs all joined together by one binary operator.
    pub fn new(BSEs: &'a [BinarySymExpr<T1, T2>], Op: Opcode) -> ChainedBSE<'a, T1, T2> {
        // // Combine all BSEs in recursive manner.
        // for idx in 0..BSEs.len() {
        //     // Join RHS
        //     let RecurSym = BinarySymExpr::new(BSEs[idx], BSEs[idx+1..], Op);
        // }

        ChainedBSE { Op, BSEs }
    }

    // Join all conditionals in BSEs with Op.
    // [BSE(1 <= s), BSE(s <= 3)].join = BSE((1 <= s) && (s <= 3))
    pub fn join(&self) {
        // TODO: This requires RecursiveIteration over BinarySymExpr to generate
        // something like:
        //      (((BSE1 Op BSE2) Op BSE3) Op BSE4)
        // from a vector of BSE<Sym,Sym>(s).
        println!("Joining is happening attention");
        for cond in self.BSEs {
            print!("{} ", cond);
        }
        println!("\nJoining is done");
    }
}

impl<'a, T1, T2> fmt::Display for ChainedBSE<'a, T1, T2>
where
    T1: fmt::Display,
    T2: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for BSE in self.BSEs {
            println!("{} {} ", BSE, self.Op);
        }
        write!(f, "( {} )", self.Op)
    }
}

impl<'a, T1, T2> Draw for ChainedBSE<'a, T1, T2>
where
    T1: Draw,
    T2: Draw,
{
    fn dump(&self, tabl: Option<usize>) -> String {
        let mut _ret = String::new();

        for BSE in self.BSEs {
            BSE.dump(None);
            // ret.push(!("{} {}", BSE.dump(Some(tabl.unwrap_or(0) + 4)), self.Op));
        }

        format!(
            "{}\n",
            format!("{ws}{}", self.Op, ws = " ".repeat(tabl.unwrap_or(0))),
        )
    }

    fn declare(&self) -> String {
        String::new()
    }

    type OutputType = BinarySymExpr<T1, T2>;
    fn iter(&self) -> (bool, &Self::OutputType) {
        (true, &self.BSEs[0])
    }
}
