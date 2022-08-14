//! Expr : An expression type. This may be extended for UnarySymExprs as well.

use crate::BinOp::Opcode;
use crate::Draw::Draw;
use std::fmt;

pub struct BinarySymExpr<'a, T1, T2> {
    LHS: &'a T1,
    RHS: &'a T2,
    Op: &'a Opcode,
}

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
impl<'a, T1, T2> BinarySymExpr<'a, T1, T2> {
    pub fn new(LHS: &'a T1, RHS: &'a T2, Op: &'a Opcode) -> BinarySymExpr<'a, T1, T2> {
        BinarySymExpr { LHS, RHS, Op }
    }
}

impl<'a, T1, T2> fmt::Display for BinarySymExpr<'a, T1, T2>
where
    T1: fmt::Display,
    T2: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", *self.LHS, self.Op, *self.RHS)
    }
}

impl<'a, T1, T2> Draw for BinarySymExpr<'a, T1, T2>
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
}
