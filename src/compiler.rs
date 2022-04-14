#[allow(dead_code)]
pub mod instructions;

use crate::parser::ast::Stmt;
use instructions::Instruction;

pub fn compile(ast: &Vec<Stmt>) -> Result<Vec<Instruction>, ()> {
    Ok(vec![])
}