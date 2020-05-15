use crate::ast::*;
use faerie::{ArtifactBuilder, Decl};
use target_lexicon::HOST;

pub fn compile(ast: &Ast, output: &str) {
    let obj_filename = format!("{}.o", output);
    let obj_file = std::fs::File::create(&obj_filename).unwrap();
    let mut obj = ArtifactBuilder::new(HOST)
        .name((&obj_filename).into())
        .finish();

    obj.declare("_start", Decl::function()).unwrap();

    let mut code = Vec::new();

    gen_ast(ast, &mut code);
    gen_exit(&mut code);

    obj.define("_start", code).unwrap();
    obj.write(obj_file).unwrap();

    std::process::Command::new("ld")
        .arg(&obj_filename)
        .arg("-o")
        .arg(output)
        .output()
        .unwrap();

    std::process::Command::new("rm")
        .arg(&obj_filename)
        .output()
        .unwrap();
}

fn gen_exit(code: &mut Vec<u8>) {
    code.extend(&[0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00]);
    code.extend(&[0x5f]);
    code.extend(&[0x0f, 0x05]);
}

fn gen_ast(ast: &Ast, code: &mut Vec<u8>) {
    match ast {
        Ast::Int { val, .. } => {
            code.extend(&[0x48, 0xb8]);
            code.extend(&val.to_le_bytes()[..]);
            code.push(0x50);
        }
        Ast::Op {
            op, left, right, ..
        } => {
            gen_ast(left, code);
            gen_ast(right, code);

            code.extend(&[0x41, 0x58, 0x58]);

            match op {
                Op::Add => code.extend(&[0x4c, 0x01, 0xc0, 0x50]),
                Op::Sub => code.extend(&[0x4c, 0x29, 0xc0, 0x50]),
                Op::Mul => code.extend(&[0x49, 0xf7, 0xe0, 0x50]),
                Op::Div => code.extend(&[0x48, 0x31, 0xd2, 0x49, 0xf7, 0xf0, 0x50]),
            }
        }
        Ast::Group { expr, .. } => gen_ast(expr, code),
    }
}
