#[allow(dead_code)]
pub mod instructions;
pub mod error;

use std::collections::{HashMap, LinkedList};
use crate::parser::ast::{Expr, Literal, Stmt, SequenceStmt, SourceLocation};
use instructions::Instruction;
use error::Error;

type Result<T> = std::result::Result<T, Error>;
type CompileResult = Result<Vec<Instruction>>;
type ExpiredLifetimes = HashMap<usize, Vec<String>>;
type IndexTable = LinkedList<(String, usize)>;

pub fn compile(ast: &Vec<Stmt>, drop_at: &ExpiredLifetimes) -> CompileResult {
    let accumulate_bytecode = |acc: CompileResult, result| match acc {
        Ok(mut program_bytecode) => match result {
            Ok(bytecode) => {
                program_bytecode.extend(bytecode);
                Ok(program_bytecode)
            },
            err@Err(_) => err,
        },
        err@Err(_) => err,
    };

    let mut index_table: IndexTable = LinkedList::new();
    let mut has_main_function = false;

    scan_declaration_names(ast)?
        .into_iter()
        .enumerate()
        .for_each(|(index, name)| {
            if name == "main" {
                has_main_function = true;
            }
            index_table.push_front((name, index));
        });

    if !has_main_function {
        return Ok(vec![Instruction::START, Instruction::DONE])
    }

    ast.iter()
        .map(|stmt| compile_top_level(stmt, drop_at, &mut index_table))
        .fold(Ok(vec![Instruction::START]), accumulate_bytecode)
}

fn scan_declaration_names(stmts: &Vec<Stmt>) -> Result<Vec<String>> {
    let scan_stmt = |stmt: &Stmt| match stmt {
        Stmt::LetStmt { name, .. } => {
            let name = get_identifier_name(name)?;
            Ok(vec![name])
        },
        Stmt::FuncDeclaration { name, .. } => {
            let name = get_identifier_name(name)?;
            Ok(vec![name])
        },
        _ => Ok(vec![]),
    };

    stmts.iter()
        .map(scan_stmt)
        .fold(Ok(vec![]), |acc, result| {
            let mut names = acc?;
            names.extend(result?);
            Ok(names)
        })
}

fn get_identifier_name(expr: &Expr) -> Result<String> {
    match expr {
        Expr::IdentifierExpr(name, position) => Ok(name.clone()),
        _ => Err(Error {
            message: String::from("Expected an identifier to get a name from"),
            position: None,
        })
    }
}

fn index_of(index_table: &IndexTable, name: &str) -> Option<usize> {
    for (corresponding_name, index) in index_table {
        if name == corresponding_name {
            return Some(index.clone())
        }
    }
    None
}

fn compile_top_level(stmt: &Stmt, drop_at: &ExpiredLifetimes, index_table: &mut IndexTable) -> CompileResult {
    match stmt {
        Stmt::FuncDeclaration { position, .. } => stmt.compile(drop_at, index_table),
        _ => Err(Error {
            message: String::from("Only function declarations are allowed at the top-level"),
            position: None,
        })
    }
}

pub trait Compile {
    fn compile(&self, drop_at: &ExpiredLifetimes, index_table: &mut IndexTable) -> CompileResult;
    fn compile_drops(&self, position: &SourceLocation, drop_at: &ExpiredLifetimes) -> CompileResult {
        drop_at.get(&position.line);
        Ok(vec![])
    }
}

impl Compile for Stmt {
    fn compile(&self, drop_at: &ExpiredLifetimes, index_table: &mut IndexTable) -> CompileResult {
        match self {
            Stmt::LetStmt { name, value, position, .. } => match value {
                Some(expr) => {
                    let mut bytecode = expr.compile(drop_at, index_table)?;
                    let name = get_identifier_name(name)?;
                    let index = match index_of(index_table, &name) {
                        Some(index) => Ok(index),
                        None => Err(Error {
                            message: format!("Unable to find \"{}\" in the index table", name),
                            position: Some(position.clone()),
                        })
                    }?;
                    bytecode.push(Instruction::ASSIGN(index));
                    bytecode.extend(self.compile_drops(position, drop_at)?);
                    Ok(bytecode)
                },
                None => Err(Error {
                    message: format!("Unbounded declaration \"{}\" found and is presently unsupported", 
                        get_identifier_name(name)?),
                    position: Some(position.clone()),
                })
            },
            stmt@Stmt::FuncDeclaration { .. } => Ok(vec![]),
            stmt@Stmt::ExprStmt { .. } => Ok(vec![]),
            _ => Err(Error {
                message: String::from("The given statement type is presently unsupported"),
                position: None,
            })
        }
    }
}

impl Compile for Expr {
    fn compile(&self, drop_at: &ExpiredLifetimes, index_table: &mut IndexTable) -> CompileResult {
        match self {
            Expr::IdentifierExpr(name, position) => match index_of(&index_table, name) {
                Some(index) => Ok(vec![Instruction::LD(index)]),
                None => Err(Error {
                    message: format!("The name \"{}\" is not found", name),
                    position: Some(position.clone()),
                })
            },
            Expr::LiteralExpr(value, position) => {
                let mut bytecode = value.compile(drop_at, index_table)?;
                bytecode.extend(self.compile_drops(position, drop_at)?);
                Ok(bytecode)
            },
            Expr::BlockExpr(block, position) => Ok(vec![]),
            Expr::PrimitiveOperationExpr(op, position) => Ok(vec![]),
            expr@Expr::AssignmentExpr { .. } => Ok(vec![]),
            expr@Expr::ApplicationExpr { .. } => Ok(vec![]),
            expr@Expr::ReturnExpr(expr_to_return, position) => Ok(vec![]),
        }
    }
}

impl Compile for Literal {
    fn compile(&self, drop_at: &ExpiredLifetimes, index_table: &mut IndexTable) -> CompileResult {
        match self {
            Literal::IntLiteral(value) => Ok(vec![Instruction::LDCI(*value)]),
            Literal::BoolLiteral(value) => Ok(vec![Instruction::LDCB(*value)]),
            // Literal::StringLiteral(value) => vec![Instruction::LDCS(*value)],
            Literal::UnitLiteral => Ok(vec![Instruction::LDCU]),
            _ => unimplemented!()
        }
    }
}