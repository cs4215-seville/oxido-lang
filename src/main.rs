mod parser;
mod static_checker;
mod compiler;

use std::fs;
use std::process;
use std::collections::HashMap;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    file_dir: String,

    #[clap(long)]
    skip_typecheck: bool,

    #[clap(long)]
    skip_compile: bool,

    #[clap(long)]
    skip_execute: bool,
}

fn main() {
    let args = Cli::parse();
    let source = fs::read_to_string(args.file_dir).expect("Unable to read file");
    println!("Parsing...\n");
    let ast = parser::parse(&source).expect("Failed to parse given program");
    if ast.len() < 1 {
        println!("Program has no executable units. To compile your program, please add a function.");
        process::exit(0);
    }
    if !args.skip_typecheck {
        println!("Typechecking...\n");
        static_checker::check(&ast);
    }
    if !args.skip_compile {
        println!("Compiling...\n");
        let bytecode = compiler::compile(&ast, &HashMap::new());
        match bytecode {
            Ok(result) => println!("{:#?}", result),
            Err(err) => println!("{}", err),
        }
    }
}