#![allow(dead_code)]
use crate::parser::ast::Stmt;
use crate::parser::ast::Block;
use crate::parser::ast::Sequence;
use crate::parser::ast::SequenceStmt;
use crate::parser::ast::FuncParameter;
use crate::parser::ast::Expr;
use crate::parser::ast::DataType;
use crate::parser::ast::Literal;
use crate::parser::ast::PrimitiveOperation;
use crate::parser::ast::UnaryOperator;
use crate::parser::ast::BinaryOperator;

pub fn get_main(parsed_stmt : &Vec<Stmt>) {// assume there's only one top-level main function
    let main_fn :& Stmt = & (*parsed_stmt)[0];
    if is_function_declaration(main_fn) {
      let ptr_to_first_seq_stmt= & (*function_declaration_body(main_fn))[0];
      let it_is_expr = is_expression_statement(sequence_statement(ptr_to_first_seq_stmt));
      println!("IT IS EXPR STMT {}", it_is_expr);
    }
}
//at every =, check type of val equals type of name.


/*LET STATEMENTS Constant declarations*/
//for now assume let statement always has right hand side.
fn is_let_statement(stmt : & Stmt) -> bool { 
    match stmt {
        Stmt::LetStmt {..}
         => return true,
        _ => return false
    }
}
fn let_statement_name(stmt : & Stmt) -> & String {
    match stmt {
        Stmt::LetStmt { name, ..}
         => match name {
             Expr::IdentifierExpr(name_string, _) => return name_string,
             _ => panic!("Name of let statement is not an identifier expression. {:#?}", name),
         }
        _ => panic!("No let statement in call for its name: {:#?}", stmt),
    }
}
fn let_statement_type(stmt : & Stmt) -> &Option<DataType> {
    match stmt {
        Stmt::LetStmt { annotation, ..}
         => return annotation,
        _ => panic!("No let statement in call for its optional type: {:#?}", stmt),
    }
}
fn let_statement_value(stmt : & Stmt) -> & Option<Expr> { 
    match stmt {
        Stmt::LetStmt { value, ..}
         => return value,
        _ => panic!("No let statement in call for its value: {:#?}", stmt),
    }
}
fn non_optional_value(non_optional_let_value : & Option<Expr>) -> & Expr {// assume not optional
    match non_optional_let_value {
        Some(val) => val,
        None => panic!("No right hand side in let statement"),
    }
}


/*FUNCTION DECLARATIONS*/
fn is_function_declaration(stmt : & Stmt) -> bool {
    match stmt {
        Stmt::FuncDeclaration {..}
         => return true,
        _ => return false
    }
}
fn function_declaration_name(stmt : & Stmt) -> & String {
    match stmt {
        Stmt::FuncDeclaration { name, ..}
         => match name {
             Expr::IdentifierExpr(name_string, _) => return name_string,
             _ => panic!("Name of function definition is not an identifier expression. {:#?}", name),
         }
        _ => panic!("No function declaration in call for its name: {:#?}", stmt),
    }
}
fn function_declaration_parameters(stmt : & Stmt) -> & Vec<FuncParameter> {
    match stmt {
        Stmt::FuncDeclaration { parameters, ..}
         => return parameters,
        _ => panic!("No function declaration in call for its parameters : {:#?}", stmt)
    }
}
fn function_declaration_body(stmt : & Stmt) -> & Sequence {
    match stmt {
        Stmt::FuncDeclaration { body, ..}
         => return & body.statements,
         _ => panic!("No function declaration in call for its parameters : {:#?}", stmt)
    }
}

// assume no static statements

/* EXPRESSION STATEMENTS*/
//ExprStmt is a parent name.
fn is_expression_statement(stmt : & Stmt) -> bool { // either a uses or modifies 
    match stmt {
        Stmt::ExprStmt(..)
         => return true,
        _ => return false
    }
}
fn expression_statement(stmt : & Stmt) -> & Expr { // either a uses or modifies 
    match stmt {
        Stmt::ExprStmt(expr) => return expr,
        _ => panic!("No expression statement in call for its expression: {:#?}", stmt)
    }
}
//IDENTIFIER
fn is_identifier_expression(expr : &Expr) -> bool { // uses
    match expr {
        Expr::IdentifierExpr(..) 
            => return true,
            _ => return false,
    }
}
fn identifier(expr : & Expr) -> & String { // symbol_of_name
    match expr {
        Expr::IdentifierExpr(name_string, ..) 
            => return name_string,
            _ => panic!("Identifier expression is not present. {:#?}", expr),
    }
}
//LITERALS
fn is_literal(expr : &Expr) -> bool { // uses
    match expr {
        Expr::LiteralExpr(..) 
            => return true,
            _ => return false,
    }
}
fn literal(expr : & Expr) -> & Literal { // just literal
    match expr {
        Expr::LiteralExpr(literal, ..)
         => return literal,
        _=> panic! ("No literal in literal expression: {:#?}", expr),
    }
}
fn is_integer_literal(literal : & Literal) -> bool { 
    match literal {
        Literal::IntLiteral(..)
          => return true,
        _ => return false,
    }
}
fn is_boolean_literal(literal : & Literal) -> bool { 
    match literal {
        Literal::BoolLiteral(..)
          => return true,
        _ => return false,
    }
}
fn is_string_literal(literal : & Literal) -> bool { 
    match literal {
        Literal::StringLiteral(..)
          => return true,
        _ => return false,
    }
}
fn is_unit_literal(literal : & Literal) -> bool { 
    match literal {
        Literal::UnitLiteral
          => return true,
        _ => return false,
    }
}
fn integer_literal(literal : & Literal) -> & i64 { 
    match literal {
        Literal::IntLiteral(number)
        => number,
        _ => panic!("Integer Literal is not present. {:#?}", literal),
    }
}
// BLOCK EXPRESSIONS
fn is_block_expression(expr : &Expr) -> bool { // end with ;
    match expr {
        Expr::BlockExpr(..) 
            => return true,
            _ => return false,
    }
}
fn block_of_expression(expr : & Expr) -> & Block {
    match expr {
        Expr::BlockExpr(boxed_block, ..) 
            => return boxed_block,
            _ => panic!("Block expression is not present. {:#?}", expr),
    }
}
//OPERATIONS
fn is_primitive_operation_expression(expr : & Expr) -> bool{
    match expr {
        Expr::PrimitiveOperationExpr(..) => return true,
        _ => return false,
    }
}
fn primitive_operation(expr : & Expr) -> & PrimitiveOperation {
    match expr {
        Expr::PrimitiveOperationExpr(boxed_prim_op, ..) => return boxed_prim_op,
        _ => panic!("Primitive operation expression not present. {:#?}", expr),
    }
}
fn is_unary_operation(prim_op : & PrimitiveOperation) -> bool {
    match prim_op {
        PrimitiveOperation::UnaryOperation{..}=> return true,
        _ => return false,
    }
}
fn unary_operator_symbol(prim_op : & PrimitiveOperation) -> & UnaryOperator {
    match prim_op {
        PrimitiveOperation::UnaryOperation{operator, ..} => return operator,
        _ =>  panic!("Unary operation is not present. {:#?}", prim_op),
    }
}
fn unary_operand(prim_op : & PrimitiveOperation) -> & Expr {
    match prim_op {
        PrimitiveOperation::UnaryOperation{operand, ..} => return operand,
        _ =>  panic!("Unary operation is not present. {:#?}", prim_op),
    }
}
fn is_binary_operation(prim_op : & PrimitiveOperation) -> bool {
    match prim_op {
        PrimitiveOperation::BinaryOperation{..}=> return true,
        _ => return false,
    }
}
fn binary_operator_symbol(prim_op : & PrimitiveOperation) -> & BinaryOperator {
    match prim_op {
        PrimitiveOperation::BinaryOperation{operator, ..} => return operator,
        _ =>  panic!("Binary operation is not present. {:#?}", prim_op),
    }
}
fn binary_first_operand(prim_op : & PrimitiveOperation) -> & Expr {
    match prim_op {
        PrimitiveOperation::BinaryOperation{first_operand, ..} => return first_operand,
        _ =>  panic!("Binary operation is not present. {:#?}", prim_op),
    }
}
fn binary_second_operand(prim_op : & PrimitiveOperation) -> & Expr {
    match prim_op {
        PrimitiveOperation::BinaryOperation{second_operand, ..} => return second_operand,
        _ =>  panic!("Binary operation is not present. {:#?}", prim_op),
    }
}
fn is_println_operator(prim_op : & PrimitiveOperation) -> bool { // variadic operation can only be println
    match prim_op {
        PrimitiveOperation::VariadicOperation{..}=> return true,
        _ => return false,
    }
}

//ASSIGNMENT
fn is_assignment_expression(expr : & Expr) -> bool{
    match expr {
        Expr::AssignmentExpr{..}=> return true,
        _ => return false,
    }
}
fn assignee(expr : & Expr) -> & Expr{ // can only be *^a / a  
    match expr {
        Expr::AssignmentExpr{assignee, ..} => return assignee,
        _ =>  panic!("Assignment expression is not present. {:#?}", expr),
    }
} 
fn assignment_value(expr : & Expr) -> & Expr{  
    match expr {
        Expr::AssignmentExpr{value, ..} => return value,
        _ =>  panic!("Assignment expression is not present. {:#?}", expr),
    }
} 

// APPLICATION
fn is_function_application_expression(expr : & Expr) -> bool{
    match expr {
        Expr::ApplicationExpr{..} => return true,
        _ => return false,
    }
}
fn function_name(expr : & Expr) -> & String {   // returns the string ptr name.
    match expr {
        Expr::ApplicationExpr{callee, ..}
        => match &**callee {
            Expr::IdentifierExpr(name, ..) => return name,
            _ => panic!("Name of function application is not an identifier {:#?}", callee),
        }
        _ => panic!("Not a function application {:#?}", expr),
    }
} 
fn function_arguments(expr : & Expr) -> & Vec<Expr> {   // must check the kind of expr of arguments.
    match expr {
        Expr::ApplicationExpr{arguments, ..}
        => return arguments,
        _ => panic!("Not a function application {:#?}", expr),
    }
} 

// RETURN
fn is_return_expression(expr : & Expr) -> bool{
    match expr {
        Expr::ReturnExpr{..} => return true,
        _ => return false,
    }
}
fn return_expression(expr : & Expr) -> & Expr {
    match expr {
        Expr::ReturnExpr(expr, ..)
        => return expr,
        _ => panic!("Not a return expression {:#?}", expr),
    }
}
/* SEQUENCE and SEQUENCE STATEMENTS*/
fn is_sequence_statement(sequence_stmt : & SequenceStmt) -> bool {
    match sequence_stmt {
        SequenceStmt::Stmt(_) => true,
        SequenceStmt::Block(_) => false,
    }
}
fn is_sequence_block(sequence_stmt : & SequenceStmt) -> bool {
    match sequence_stmt {
        SequenceStmt::Stmt(_) => false,
        SequenceStmt::Block(_) => true,
    }
}
fn sequence_statement(sequence_stmt : & SequenceStmt) -> & Stmt {
    match sequence_stmt {
        SequenceStmt::Stmt(stmt) => & stmt,
        SequenceStmt::Block(_) => panic!("Not a sequence statement : {:#?}", sequence_stmt),
    }
}
fn sequence_block(sequence_stmt : & SequenceStmt) -> & Block {
    match sequence_stmt {
        SequenceStmt::Stmt(_) =>  panic!("Not a sequence block :{:#?}", sequence_stmt),
        SequenceStmt::Block(block) => & block,
    }
}





/*fn boolean_literal(literal : & Literal) -> bool { 
    match literal {
        Literal::BoolLiteral(boolean_literal)
        => *boolean_literal,
        _ => panic!("Boolean Literal is not present. {:#?}", literal),
    }
}
*/