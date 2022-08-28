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

impl Opcode {
    pub fn getOpcode(opcode: &str) -> Option<Opcode> {
        return match opcode {
            "*" => Some(Self::Mul),
            "/" => Some(Self::Div),
            "%" => Some(Self::Rem),
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "<<" => Some(Self::Shl),
            ">>" => Some(Self::Shr),
            "<" => Some(Self::LT),
            ">" => Some(Self::GT),
            "<=" => Some(Self::LE),
            ">=" => Some(Self::GE),
            "==" => Some(Self::EQ),
            "!=" => Some(Self::NE),
            "&" => Some(Self::And),
            "^" => Some(Self::Xor),
            "|" => Some(Self::Or),
            "&&" => Some(Self::LAnd),
            "||" => Some(Self::LOr),
            "=" => Some(Self::Assign),
            _ => None,
        };
    }
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
