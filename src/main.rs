mod parser;
mod compiler;

use std::env;
use std::fs;
use std::process;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file directory supplied.");
        process::exit(1);
    }

    let source = fs::read_to_string(&args[1]).expect("Unable to read file");
    let ast = parser::parse(&source).expect("Failed to parse given program");
    let bytecode = compiler::compile(&ast, &HashMap::new());
    println!("{:#?}", bytecode);
}