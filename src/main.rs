mod analysis;
mod ast;
mod codegen;
mod parsing;
mod run;

use clap::{App, Arg, SubCommand};
use diagnostics::{FileInfo, FileInterner, Reporter};
use intern::Intern;

fn main() {
    let matches = App::new("Math lang")
        .version("0.1.0")
        .subcommand(
            SubCommand::with_name("build")
                .arg(Arg::with_name("input").takes_value(true).required(true))
                .arg(Arg::with_name("output").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("run")
                .arg(Arg::with_name("input").takes_value(true).required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let input = matches.value_of("input").unwrap();
        let output = matches.value_of("output").unwrap();

        cmd_build(input, output);
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let input = matches.value_of("input").unwrap();

        cmd_run(input);
    } else {
        println!("{}", matches.usage());
    }
}

fn cmd_build(input: &str, output: &str) {
    let reporter = Reporter::default();
    let files = FileInterner::new();
    let file = if let Ok(source) = std::fs::read_to_string(input) {
        FileInfo {
            source,
            name: input.into(),
        }
    } else {
        FileInfo {
            source: input.to_string(),
            name: "<input>".into(),
        }
    }
    .intern(&files);

    match parsing::parse(&reporter, file) {
        Err(e) => {
            reporter.add(e);
            reporter.report(true);
        }
        Ok(ast) => {
            analysis::analyze(&reporter, &ast);
            codegen::compile(&ast, output);
        }
    }
}

fn cmd_run(input: &str) {
    let reporter = Reporter::default();
    let files = FileInterner::new();
    let file = if let Ok(source) = std::fs::read_to_string(input) {
        FileInfo {
            source,
            name: input.into(),
        }
    } else {
        FileInfo {
            source: input.to_string(),
            name: "<input>".into(),
        }
    }
    .intern(&files);

    match parsing::parse(&reporter, file) {
        Err(e) => {
            reporter.add(e);
            reporter.report(true);
        }
        Ok(ast) => {
            analysis::analyze(&reporter, &ast);

            let result = run::run(&ast);

            println!("{}", result);
        }
    }
}
