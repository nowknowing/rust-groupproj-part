#![allow(dead_code)]
use std::fmt::Debug;

type LifetimeParameter = String;

#[derive(Debug)]
enum DataType {
    Base(BaseDataType),
    Func {
        lifetime_parameters: Vec<LifetimeParameter>,
        parameter_types: Vec<BaseDataType>,
        return_type: BaseDataType,
    },
}

#[derive(Debug)]
enum BaseDataType {
    Int64,
    Bool,
    Str,
    String,
    Unit,
    Ref(Option<LifetimeParameter>, Box<BaseDataType>),
    MutRef(Option<LifetimeParameter>, Box<BaseDataType>),
}

type Identifier = String;

#[derive(Debug)]
enum Literal {
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
    UnitLiteral,
}

#[derive(Debug)]
enum SequenceStmt {
    Stmt(Stmt),
    Block(Block),
}

type Sequence = Vec<SequenceStmt>;

#[derive(Debug)]
struct Block {
    statements: Sequence,
    last_expression: Option<Expr>,
}

#[derive(Debug)]
enum Expr {
    IdentifierExpr(Identifier),
    LiteralExpr(Literal),
    BlockExpr(Box<Block>),
    AssignmentExpr {
        name: Identifier,
        value: Box<Expr>,
    },
    ApplicationExpr {
        name: Identifier,
        arguments: Vec<Expr>,
    },
    ReturnExpr(Box<Expr>),
}

enum PrimitiveFunction {
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

enum UnaryOperator {
    Not,
    UnaryMinus,
    StringFrom,
    ImmutableBorrow,
    MutableBorrow,
    Drop,
    Len,
    AsStr,
}

enum BinaryOperator {
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

enum VariadicOperator {
    Println,
}

type FuncParameter = (Identifier, DataType);

#[derive(Debug)]
enum Stmt {
    LetStmt {
        name: Identifier,
        is_mutable: bool,
        annotation: Option<DataType>,
        value: Expr,
    },
    StaticStmt {
        name: Identifier,
        is_mutable: bool,
        annotation: DataType,
        value: Expr,
    },
    FuncDeclaration {
        name: Identifier,
        lifetime_parameters: Vec<LifetimeParameter>,
        parameters: Vec<FuncParameter>,
        return_type: DataType,
        body: Block,
    },
    ExprStmt(Expr),
}