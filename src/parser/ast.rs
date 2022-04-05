#![allow(dead_code)]
use std::fmt::Debug;

pub trait AST {
    fn get_source_location(&self) -> SourceLocation; 
}

#[derive(Debug, Copy, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub col: usize,
}

pub type LifetimeParameter = String;

#[derive(Debug)]
pub enum DataType {
    Int64,
    Bool,
    Str,
    String,
    Unit,
    Ref(Option<LifetimeParameter>, Box<DataType>),
    MutRef(Option<LifetimeParameter>, Box<DataType>),
    Func(Vec<LifetimeParameter>, Vec<DataType>, Box<DataType>),
}

pub type Identifier = String;

#[derive(Debug)]
pub enum Literal {
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
    UnitLiteral,
}

#[derive(Debug)]
pub enum SequenceStmt {
    Stmt(Stmt),
    Block(Block),
}

pub type Sequence = Vec<SequenceStmt>;

#[derive(Debug)]
pub struct Block {
    pub statements: Sequence,
}

#[derive(Debug)]
pub enum Expr {
    IdentifierExpr(Identifier, SourceLocation),
    LiteralExpr(Literal, SourceLocation),
    BlockExpr(Box<Block>, SourceLocation),
    PrimitiveOperationExpr(Box<PrimitiveOperation>, SourceLocation),
    AssignmentExpr {
        name: Identifier,
        value: Box<Expr>,
        position: SourceLocation,
    },
    ApplicationExpr {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        position: SourceLocation,
    },
    ReturnExpr(Box<Expr>, SourceLocation),
}

impl AST for Expr {
    fn get_source_location(&self) -> SourceLocation {
        match self {
            Expr::IdentifierExpr(_, position) => position.clone(),
            Expr::LiteralExpr(_, position) => position.clone(),
            Expr::BlockExpr(_, position) => position.clone(),
            Expr::PrimitiveOperationExpr(_, position) => position.clone(),
            Expr::AssignmentExpr { position, .. } => position.clone(),
            Expr::ApplicationExpr { position, .. } => position.clone(),
            Expr::ReturnExpr(_, position) => position.clone(),
        }
    }
}

#[derive(Debug)]
pub enum PrimitiveOperation {
    UnaryOperation {
        operator: UnaryOperator,
        operand: Expr,
    },
    BinaryOperation {
        operator: BinaryOperator,
        first_operand: Expr,
        second_operand: Expr,
    },
    VariadicOperation {
        operator: VariadicOperator,
        operands: Vec<Expr>,
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
    UnaryMinus,
    ImmutableBorrow,
    MutableBorrow,
    Dereference,
    StringFrom,
    Drop,
    Len,
    AsStr,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    And,
    Or,
}

#[derive(Debug)]
pub enum VariadicOperator {
    Println,
}

pub type FuncParameter = (Identifier, DataType);

#[derive(Debug)]
pub enum Stmt {
    LetStmt {
        name: Identifier,
        is_mutable: bool,
        annotation: Option<DataType>,
        value: Option<Expr>,
        position: SourceLocation,
    },
    StaticStmt {
        name: Identifier,
        is_mutable: bool,
        annotation: DataType,
        value: Expr,
        position: SourceLocation,
    },
    FuncDeclaration {
        name: Identifier,
        lifetime_parameters: Vec<LifetimeParameter>,
        parameters: Vec<FuncParameter>,
        return_type: DataType,
        body: Block,
        position: SourceLocation,
    },
    ExprStmt(Expr),
}

impl AST for Stmt {
    fn get_source_location(&self) -> SourceLocation {
        match self {
            Stmt::LetStmt { position, .. } => position.clone(),
            Stmt::StaticStmt { position, .. } => position.clone(),
            Stmt::FuncDeclaration { position, .. } => position.clone(),
            Stmt::ExprStmt(expr) => expr.get_source_location(),
        }
    }
}