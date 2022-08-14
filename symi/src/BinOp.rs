//! BinOp : Collect all binary operations available in C. May be extended upto
//! C++ binops.

use std::fmt;

/// Binary opcodes which BinarySymExpr are allowed to take into account.
// TODO: Rename to BinOp
pub enum Opcode {
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Shl,
    Shr,
    LT,
    GT,
    LE,
    GE,
    EQ,
    NE,
    And,
    Xor,
    Or,
    LAnd,
    LOr,
    Assign,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Rem => write!(f, "%"),
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
            Opcode::Shl => write!(f, "<<"),
            Opcode::Shr => write!(f, ">>"),
            Opcode::LT => write!(f, "<"),
            Opcode::GT => write!(f, ">"),
            Opcode::LE => write!(f, "<="),
            Opcode::GE => write!(f, ">="),
            Opcode::EQ => write!(f, "=="),
            Opcode::NE => write!(f, "!="),
            Opcode::And => write!(f, "&"),
            Opcode::Xor => write!(f, "^"),
            Opcode::Or => write!(f, "|"),
            Opcode::LAnd => write!(f, "&&"),
            Opcode::LOr => write!(f, "||"),
            Opcode::Assign => write!(f, "="),
        }
    }
}
