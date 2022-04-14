mod parser;
mod static_checker;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file directory supplied.");
        process::exit(1);
    }

    let source = fs::read_to_string(&args[1]).expect("Unable to read file");
    let result = parser::parse(&source);    
    //let ast = parser::parse(&source).expect("Failed to parse given program");
    
/*
    match result {
        Ok(v) => static_checker::get_main(&v),
        _=> println!("{:#?}", result),

    }
    */
    
    println!("{:#?}", result);
}