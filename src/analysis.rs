use crate::ast::*;
use diagnostics::{Diagnostic, Reporter, Severity, Span};

pub fn analyze(reporter: &Reporter, ast: &Ast) {
    analyze_ast(reporter, ast);

    reporter.report(true);
}

fn analyze_ast(reporter: &Reporter, ast: &Ast) {
    match ast {
        Ast::Int { .. } => {}
        Ast::Op {
            span,
            op,
            left,
            right,
        } => analyze_op(reporter, *span, op, left, right),
        Ast::Group { expr, .. } => analyze_ast(reporter, expr),
    }
}

fn analyze_op(reporter: &Reporter, _span: Span, op: &Op, left: &Ast, right: &Ast) {
    analyze_ast(reporter, left);
    analyze_ast(reporter, right);

    if let Op::Div = op {
        if let Ast::Int { span, val: 0 } = right {
            reporter.add(
                Diagnostic::new(Severity::Error, None, "Cannot divide by 0").label(
                    Severity::Error,
                    *span,
                    None::<String>,
                ),
            );
        }
    }
}
