use crate::ast::*;
use diagnostics::{FileId, Reporter, Span};
use parser::error::Result;
use parser::literal::IntLiteral;
use parser::parse::ParseStream;

parser::token![punct "+" TAdd/1];
parser::token![punct "-" TSub/1];
parser::token![punct "*" TMul/1];
parser::token![punct "/" TDiv/1];

parser::token![punct "(" TLParen/1];
parser::token![punct ")" TRParen/1];

pub fn parse(reporter: &Reporter, file: FileId) -> Result<Ast> {
    let mut lexer = parser::lexer::Lexer::new(&file.source, file, reporter);
    let tokens = lexer.run();
    let buffer = parser::parse::ParseBuffer::new(tokens.begin(), reporter, (), Span::empty(file));

    Ast::parse_add_sub(&buffer)
}

impl Ast {
    fn parse_add_sub(input: ParseStream) -> Result<Self> {
        let start = input.span();
        let mut result = Self::parse_mul_div(input)?;

        while !input.is_empty() && (input.peek::<TAdd>() || input.peek::<TSub>()) {
            if let Ok(_) = input.parse::<TAdd>() {
                let right = Self::parse_mul_div(input)?;

                result = Self::Op {
                    span: start.to(input.prev_span()),
                    op: Op::Add,
                    left: Box::new(result),
                    right: Box::new(right),
                };
            } else {
                input.parse::<TSub>()?;

                let right = Self::parse_int(input)?;

                result = Self::Op {
                    span: start.to(input.prev_span()),
                    op: Op::Sub,
                    left: Box::new(result),
                    right: Box::new(right),
                };
            }
        }

        Ok(result)
    }

    fn parse_mul_div(input: ParseStream) -> Result<Self> {
        let start = input.span();
        let mut result = Self::parse_int(input)?;

        while !input.is_empty() && (input.peek::<TMul>() || input.peek::<TDiv>()) {
            if let Ok(_) = input.parse::<TMul>() {
                let right = Self::parse_int(input)?;

                result = Self::Op {
                    span: start.to(input.prev_span()),
                    op: Op::Mul,
                    left: Box::new(result),
                    right: Box::new(right),
                };
            } else {
                input.parse::<TDiv>()?;

                let right = Self::parse_int(input)?;

                result = Self::Op {
                    span: start.to(input.prev_span()),
                    op: Op::Div,
                    left: Box::new(result),
                    right: Box::new(right),
                };
            }
        }

        Ok(result)
    }

    fn parse_int(input: ParseStream) -> Result<Self> {
        if let Ok(lparen) = input.parse::<TLParen>() {
            let sub = Self::parse_add_sub(input)?;

            input.parse::<TRParen>()?;

            Ok(Self::Group {
                span: lparen.span.to(input.prev_span()),
                expr: Box::new(sub),
            })
        } else {
            let lit = input.parse::<IntLiteral>()?;

            Ok(Self::Int {
                span: lit.span,
                val: lit.int as u64,
            })
        }
    }
}
