use crate::ast::*;

pub fn run(ast: &Ast) -> u64 {
    match ast {
        Ast::Int { val, .. } => *val,
        Ast::Op {
            op, left, right, ..
        } => {
            let left = run(left);
            let right = run(right);

            match op {
                Op::Add => left + right,
                Op::Sub => left - right,
                Op::Mul => left * right,
                Op::Div => left / right,
            }
        }
        Ast::Group { expr, .. } => run(expr),
    }
}
