extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct OxidoParser;

fn main() {
    let successful_parse = OxidoParser::parse(Rule::function_declaration, "fn main () {}");
    println!("{:?}", successful_parse);
}