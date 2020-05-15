use diagnostics::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Int {
        span: Span,
        val: u64,
    },
    Op {
        span: Span,
        op: Op,
        left: Box<Ast>,
        right: Box<Ast>,
    },
    Group {
        span: Span,
        expr: Box<Ast>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int { val, .. } => write!(f, "{}", val),
            Self::Op {
                op, left, right, ..
            } => write!(f, "{} {} {}", left, op, right),
            Self::Group { expr, .. } => write!(f, "({})", expr),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}
