#![allow(dead_code)]
use crate::parser::ast::
{AST, Stmt, Block, Sequence, SequenceStmt, FuncParameter, Expr, DataType, Literal, 
    PrimitiveOperation, UnaryOperator, BinaryOperator};
use std::collections::{HashMap, LinkedList};

pub fn check(parsed_stmt : & Vec<Stmt>) -> ExpiredLifetimes{
    let mut d_f_table : HashMap<&'static str, FunctionStore> = HashMap::new();
    let mut scope : LinkedList<DecAndBorrowStack> = LinkedList::new();
    let mut v_table : HashMap<&'static str, VariableProperties> = HashMap::new();
    let mut exp_lt : ExpiredLifetimes = ExpiredLifetimes::new();
    let mut env = Environment{
        declared_functions_table : d_f_table,
        scope_info : scope,
        variables_table : v_table,
        expired_lifetimes : exp_lt,
    };
    // let all be in the main function.
    let main_fn : &Stmt = & (*parsed_stmt)[0];
    type_statement(main_fn, &mut env);
    return env.expired_lifetimes;
}

type ExpiredLifetimes = HashMap<usize, Vec<String>>;
type FunctionStore = (Vec<DataType>, DataType);
type DecAndBorrowStack = (Vec<&'static str>, LinkedList<&'static str>);
struct Environment {
    declared_functions_table : HashMap<&'static str, FunctionStore>,
    scope_info: LinkedList<DecAndBorrowStack>,
    variables_table : HashMap<&'static str, VariableProperties>,
    expired_lifetimes : ExpiredLifetimes,
}

struct VariableProperties{
    own_type : DataType,
    mutability : bool,
    is_copy_trait_mem : bool,
    // not accounting for immutable borrows for now
}

fn check_duplicate(name : & String, env : &mut Environment) {
    for (fn_name, value) in env.declared_functions_table.iter() {
        if (*fn_name).eq(name.as_str()) {
            panic!("Duplicate name: {} already declared", name);
        }
    }
    for (var_name, value) in env.variables_table.iter() {
        if (*var_name).eq(name.as_str()) {
            panic!("Duplicate name: {} already declared", name);
        }
    }
}

fn insert_expired_lifetime(env : &mut Environment, line_no : usize, var_name : &str) {
    match env.expired_lifetimes.get_mut(&line_no) {
        Some(vars) => {let o = vars.push(String::from(var_name));
        },
        None => {let o = env.expired_lifetimes.insert(line_no, vec![String::from(var_name)]);
        },
    }
}

fn update_scope_with_use(env : &mut Environment, var_name : &str) {

}

fn update_scope_with_drop(env : &mut Environment, var_name : &str) {

}

fn handle_stack(rhs : &Expr, env : &mut Environment) {
    if is_identifier_expression(rhs) {
        let name = identifier(rhs).as_str();
        let rhs_datatype = & env.variables_table.get(name).unwrap().own_type;
        if is_mem_type(rhs_datatype) { // rhs is some memory
            //drop type. then remove RHS.
            if !is_copy_type(rhs_datatype) {
                insert_expired_lifetime(env, rhs.get_source_location().line, identifier(rhs));
            }
        }
    }
}
fn type_statement(stmt : &  Stmt, env : &mut  Environment)  -> DataType {
    if is_let_statement(stmt) {
        let name = let_statement_name(stmt);
        check_duplicate(name, env); // check for duplicate in scope.

        let rhs = non_optional_value(let_statement_value(stmt));
        handle_stack(rhs, env); // handle right hand side uses only. MODIFIES STACK

        let type_of_variable = type_expression(non_optional_value(let_statement_value(stmt)), env);
        //let degree = degree(non_optional_value(let_statement_value(stmt)), env);
        let mutability = is_mutable_let_statement(stmt);
       // set_variable(name, *type_of_variable, mutability, degree);  // updates stack. MODIFIES STACK
        
        return DataType::Unit;
    } else if is_function_declaration(stmt) {
        let function_name = function_declaration_name(stmt);
       // check_duplicate(function_name, env);      // no overloading allowed
       // check_function_sanity(stmt, env); // checks on block sanity. + return type consistency
        
        let params = function_declaration_parameters(stmt);
        let return_type = function_declaration_return_type(stmt); // Unit if returns nothing.

        //set_function(function_name, params, return_type, env);
        return DataType::Unit;
    } else if is_expression_statement(stmt) {
        return type_expression(expression_statement(stmt), env);
    } else {  
        return DataType::Unit;
    }
}

//fn set_function(function_name : & String,
  //   params : & Vec<FuncParameter>, return_type : & DataType, env : &mut Environment) {
    //     env.
fn type_expression(expr : &  Expr, env : & mut Environment) -> DataType {
    if is_identifier_expression(expr) {
       unimplemented!();
        // lookup_type(identifier(expr), env);
    } else if is_literal(expr) {
        if is_integer_literal(literal(expr)) {
            return DataType::Int64;
        } else if is_boolean_literal(literal(expr)) {
            return DataType::Bool;
        } else if is_string_literal(literal(expr)) {
            return DataType::String;
        } else if is_unit_literal(literal(expr)) {
            return DataType::Unit;
        } else {
            panic!("unknown literal");
        }
    } else if is_block_expression(expr) {  //
        let mut seq_copy = statements_of_block(block_of_expression(expr)).clone();
        let (dt, hs) = type_and_handle_sequence(&mut seq_copy, env);
        return dt;
    } else if is_primitive_operation_expression(expr) {
        match expr {
            Expr::PrimitiveOperationExpr(op, position) => DataType::Unit, // TODO //op.typecheck(env),
            _ => panic!("")
        }
    } else if is_return_expression(expr) {
        return type_expression(return_expression(expr), env); // MUST DO
    } else if is_function_application_expression(expr) {
        unimplemented!();
        //type_application(function_name(expr), function_arguments(expr), env);
    } else {
        panic!("Type Error at {:#?} for {:#?}", expr.get_source_location(), expr);
    }
}

/*
fn type_application(
    function_name: & String,
    function_arguments : & Vec<Expr>,
     env: &mut Environment) -> DataType{
         

}
*/

fn is_mem_type(datatype : &DataType) -> bool {
    match datatype{
        DataType::Ref(..) | DataType::MutRef(..) => return false,
        _ => return true,
    }
}

fn is_copy_type(datatype : &DataType) -> bool {
    match datatype{
        DataType::Int64 | DataType::Bool | DataType::Str => return true,
        _ => return false,
    }
}

trait TypeCheck {
    fn typecheck(&self, env: &mut Environment) -> DataType;
}


/* parser incorrect
impl TypeCheck for PrimitiveOperation {
    fn typecheck(&self, env: &mut Environment) -> DataType {
        match self {
            PrimitiveOperation::UnaryOperation { operator, operand } => match operator {
                UnaryOperator::Not => match type_expression(operand, env) {
                    DataType::Bool => return DataType::Bool,
                    _ => panic!("Expect bool operand for not operation"),
                },
                UnaryOperator::UnaryMinus => match type_expression(operand, env) {
                    DataType::Int64 => return  DataType::Int64,
                    _ => panic!("Expect i64 for unary minus operation"),
                },
                UnaryOperator::ImmutableBorrow => match type_expression(operand, env) {
                    DataType::Func(..) => panic!("References to functions are currently unsupported"),
                    datatype@_ => return DataType::Ref(None, Box::new(datatype))
                },
                UnaryOperator::MutableBorrow => match type_expression(operand, env) {
                    DataType::Func(..) => panic!("Mutable references to functions are currently unsupported"),
                    datatype@_ => return DataType::MutRef(None, Box::new(datatype))
                },
                UnaryOperator::Dereference => match type_expression(operand, env) {
                    DataType::Ref(_, dereferenced_type) => return *dereferenced_type,
                    DataType::MutRef(_, dereferenced_type) => return *dereferenced_type,
                    _ => panic!("Dereferencing can only be performed on a mutable or immutable borrow"),
                },
                UnaryOperator::StringFrom => match type_expression(operand, env) {
                    DataType::Ref(_, boxed_type) => match *boxed_type {
                        DataType::Str => return DataType::String,
                        _ => panic!("Expect &str for string_from operation"),
                    },
                    _ => panic!("Expect &str for string_from operation"),
                },
                UnaryOperator::Len => match type_expression(operand, env) {
                    DataType::Ref(_, boxed) => match *boxed {
                        DataType::Str => return DataType::Int64,
                        _ => panic!("none found in the boxed"),
                    }
                    DataType::String => DataType::Int64,
                    _ => panic!("Expect &str or String for len operation"),
                },
                UnaryOperator::AsStr => match type_expression(operand, env) {
                    
                },,
                UnaryOperator::PushStr => panic!("parser mistaken. Push_str not supported."),
                UnaryOperator::Drop => unimplemented!(),
            },
            PrimitiveOperation::BinaryOperation { operator, first_operand, second_operand } => {
                match operator {
                    BinaryOperator::Plus => vec![Instruction::PLUS],
                    BinaryOperator::Minus => vec![Instruction::MINUS],
                    BinaryOperator::Times => vec![Instruction::TIMES],
                    BinaryOperator::Divide => vec![Instruction::DIV],
                    BinaryOperator::Equal => vec![Instruction::EQUAL],
                    BinaryOperator::NotEqual => vec![Instruction::EQUAL, Instruction::NOT],
                    BinaryOperator::Greater => vec![Instruction::GREATER],
                    BinaryOperator::GreaterOrEqual => vec![Instruction::GEQ],
                    BinaryOperator::Less => vec![Instruction::LESS],
                    BinaryOperator::LessOrEqual => vec![Instruction::LEQ],
                    BinaryOperator::And => vec![Instruction::AND],
                    BinaryOperator::Or => vec![Instruction::OR],
                }
            }
            PrimitiveOperation::VariadicOperation { operator, operands } => 
                match type_check(operands, env) {
                    VariadicOperator::Println => unimplemented!(),
                }
        }
    }
}
*/


//MUST HAVE RETURN STATEMENT SOMEWHERE. OTHERWISE RETURN NONE.
fn type_and_handle_sequence(sequence : &mut Sequence, env : & mut Environment) -> (DataType, bool) { // must have return o
    if is_empty_sequence(sequence) {
        return (DataType::Unit, false);
        //SB here
    } else if is_last_statement_of_sequence(sequence) { // MUST BE RETURN. OTHERWISE NONE. HANDLE HERE.
        match first_statement_of_sequence(sequence) {
            SequenceStmt::Stmt(stmt) => 
                match stmt {
                    Stmt::ExprStmt(expr) => 
                     if is_return_expression(expr) {
                         return (type_expression(expr, env), true);   
                     } else {
                         let curr_stmt_type = type_expression(expr, env);
                         return (DataType::Unit, false);
                     },
                    _ => {
                        let curr_stmt_type = type_statement(stmt, env);
                        return (DataType::Unit, false);
                    },
                },
            SequenceStmt::Block(block) => {
                let mut mut_seq = statements_of_block(block).clone();
                return type_and_handle_sequence(&mut mut_seq, env);
            },
        }
    } else { // IF RETURN, PANIC IF EARLY END.
        match first_statement_of_sequence(sequence) {
            SequenceStmt::Stmt(stmt) => 
                match stmt {
                    Stmt::ExprStmt(expr) => 
                    panic! ("Unreacheable statement not allowed at {:#?} for {:#?}", stmt.get_source_location(), expr),
                    _ => {let curr_stmt_type = type_statement(stmt, env);
                          return type_and_handle_sequence(rest_statements_of_sequence(sequence), env)},
                },
            SequenceStmt::Block(block) => {
                let mut mut_seq = statements_of_block(block).clone();
                let (datatype, has_return) = type_and_handle_sequence(&mut mut_seq, env);
                if has_return {
                    panic! ("Unreacheable statement not allowed for {:#?}", block);
                }
                return type_and_handle_sequence(rest_statements_of_sequence(sequence), env);
            },
        }
    }
}

fn is_empty_sequence(sequence : &mut Sequence) -> bool {
    return sequence.is_empty();
}
fn is_last_statement_of_sequence(sequence : &mut Sequence) -> bool {
    return sequence.len() == 1;
}
fn first_statement_of_sequence(sequence : &mut Sequence) -> & SequenceStmt {
    return & sequence[0];
}
fn rest_statements_of_sequence(sequence : &mut Sequence) -> &mut Sequence {
    let lastIdx = sequence.len() - 1;
    sequence.remove(0);
    return sequence;
}


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
fn is_mutable_let_statement(stmt : & Stmt) -> bool { 
    match stmt {
        Stmt::LetStmt {is_mutable, ..}
         => return *is_mutable,
         _ => panic!("No let statement in call for its mutability: {:#?}", stmt),

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
fn function_declaration_return_type(stmt : & Stmt) -> & DataType {
    match stmt {
        Stmt::FuncDeclaration {return_type, ..}
         => return return_type,
         _ => panic!("No function declaration in call for its return type : {:#?}", stmt)
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
fn statements_of_block(block : & Block) -> & Sequence {
    match block {
        Block{statements}
            => return statements,
            _ => panic!("Block expression is not present at call to sequence. {:#?}", block),
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



/*
Not,
UnaryMinus,
ImmutableBorrow,//undone
MutableBorrow,//undone
Dereference,//undone
StringFrom,
Drop,
Len,
AsStr,
PushStr, // undone



Int64,
Bool,
Str,
String,
Unit,
Ref(Option<LifetimeParameter>, Box<DataType>),
MutRef(Option<LifetimeParameter>, Box<DataType>),
Func(Vec<LifetimeParameter>, Vec<DataType>, Box<DataType>),
*/

/*fn boolean_literal(literal : & Literal) -> bool { 
    match literal {
        Literal::BoolLiteral(boolean_literal)
        => *boolean_literal,
        _ => panic!("Boolean Literal is not present. {:#?}", literal),
    }
}
*/