mod ast;

use pest_consume::{match_nodes, Error, Parser};
use ast::{
    AST,
    Expr,
    Literal,
    DataType,
    PrimitiveOperation,
    UnaryOperator,
    BinaryOperator, 
    SourceLocation,
    LifetimeParameter,
    FuncParameter,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct OxidoParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl OxidoParser {
    // TODO: Shion.
    fn declaration(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    // TODO: Shion.
    fn static_declaration(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn mutable_specifier(input: Node) -> Result<bool> {
        Ok(true)
    }
    fn datatype(input: Node) -> Result<DataType> {
        Ok(match input.as_str() {
            "i32" => DataType::Int64,
            "bool" => DataType::Bool,
            "str" => DataType::Str,
            "String" => DataType::String,
            "()" => DataType::Unit,
            _ => match_nodes!(input.into_children();
                    [function_datatype(f)] => f,
                    [reference_datatype(d)] => d),
        })
    }
    fn reference_datatype(input: Node) -> Result<DataType> {
        let create_reference_type = |lifetime, is_mutable, datatype| match is_mutable {
            true => DataType::MutRef(lifetime, Box::from(datatype)),
            false => DataType::Ref(lifetime, Box::from(datatype)),
        };

        Ok(match_nodes!(input.into_children();
            [datatype(d)] =>
                create_reference_type(None, false, d),
            [lifetime_type_variable(l), datatype(d)] =>
                create_reference_type(Some(l), false, d),
            [mutable_specifier(_m), datatype(d)] => 
                create_reference_type(None, true, d),
            [lifetime_type_variable(l), mutable_specifier(_m), datatype(d)] => 
                create_reference_type(Some(l), true, d),
        ))
    }
    fn function_datatype(input: Node) -> Result<DataType> {
        Err(input.error("Function pointers are currently unsupported"))

        // Uncomment the following implementation when function pointers are supported. 
        /* Ok(match_nodes!(input.into_children();
            [function_datatype_param_list(params), function_return_type(mut r)..] =>
                match r.next() {
                    None =>
                        DataType::Func(vec![], params, Box::from(DataType::Unit)), 
                    Some(return_type) =>
                        DataType::Func(vec![], params, Box::from(return_type)),
                }
        )) */
    }
    fn function_datatype_param_list(input: Node) -> Result<Vec<DataType>> {
        Ok(match_nodes!(input.into_children();
            [datatype(d)..] => d.collect(),
        ))
    }
    fn block(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn sequence(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    // TODO: Shion.
    fn stmt(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    // TODO: Shion.
    fn expr_stmt(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn expr(input: Node) -> Result<Expr> {
        Ok(match_nodes!(input.into_children();
            [assignment(expr)] => expr,
        ))
    }
    fn primary(input: Node) -> Result<Expr> {
        // TODO: match all cases.
        Ok(match_nodes!(input.into_children();
            [integer_literal(expr)] => expr,
            [string_literal(expr)] => expr,
            [boolean_literal(expr)] => expr,
            [identifier(expr)] => expr,
        ))
    }
    fn assignment(input: Node) -> Result<Expr> {
        let (line, col) = input.as_span().start_pos().line_col();
        match_nodes!(input.children();
            [identifier(ident), assignment(value)] => {
                if let Expr::IdentifierExpr(name, _) = ident {
                    Ok(Expr::AssignmentExpr {
                        name,
                        value: Box::from(value),
                        position: SourceLocation { line, col },
                    })
                } else {
                    Err(input.error("Left-hand side of an assignment must be an identifier"))
                }
            },
            [disjunction(d)] => Ok(d),
        )
    }
    fn disjunction(input: Node) -> Result<Expr> {
        let create_binary_expr = |operator, first_operand, second_operand, src_location| 
            Expr::PrimitiveOperationExpr(
                Box::from(PrimitiveOperation::BinaryOperation {
                    operator,
                    first_operand,
                    second_operand,
                }),
                src_location,
            );

        match_nodes!(input.children();
            [conjunction(initial_operand), conjunction(repetitions)..] => {
                let mut repetitions = repetitions.rev().peekable();
                match repetitions.next() {
                    Some(expr) => {
                        let mut second_operand = expr;

                        if repetitions.peek().is_none() {
                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                BinaryOperator::Or,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        } else {
                            for first_operand in repetitions {
                                let src_location = first_operand.get_source_location();
                                second_operand = create_binary_expr(
                                    BinaryOperator::Or,
                                    first_operand,
                                    second_operand,
                                    src_location,
                                );
                            }

                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                BinaryOperator::Or,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        }
                    },
                    None => Ok(initial_operand),
                }
            },
        )
    }
    fn conjunction(input: Node) -> Result<Expr> {
        let create_binary_expr = |operator, first_operand, second_operand, src_location| 
            Expr::PrimitiveOperationExpr(
                Box::from(PrimitiveOperation::BinaryOperation {
                    operator,
                    first_operand,
                    second_operand,
                }),
                src_location,
            );

        match_nodes!(input.children();
            [equality(initial_operand), equality(repetitions)..] => {
                let mut repetitions = repetitions.rev().peekable();
                match repetitions.next() {
                    Some(expr) => {
                        let mut second_operand = expr;

                        if repetitions.peek().is_none() {
                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                BinaryOperator::And,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        } else {
                            for first_operand in repetitions {
                                let src_location = first_operand.get_source_location();
                                second_operand = create_binary_expr(
                                    BinaryOperator::And,
                                    first_operand,
                                    second_operand,
                                    src_location,
                                );
                            }

                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                BinaryOperator::And,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        }
                    },
                    None => Ok(initial_operand),
                }
            },
        )
    }
    fn equality(input: Node) -> Result<Expr> {
        let create_binary_expr = |operator, first_operand, second_operand, src_location| 
            Expr::PrimitiveOperationExpr(
                Box::from(PrimitiveOperation::BinaryOperation {
                    operator,
                    first_operand,
                    second_operand,
                }),
                src_location,
            );

        match_nodes!(input.children();
            [comparison(initial_operand), equality_helper(repetitions)..] => {
                let mut repetitions = repetitions.rev().peekable();
                match repetitions.next() {
                    Some((op, expr)) => {
                        let mut current_op = op;
                        let mut second_operand = expr;

                        if repetitions.peek().is_none() {
                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                current_op,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        } else {
                            for (op, first_operand) in repetitions {
                                let src_location = first_operand.get_source_location();
                                second_operand = create_binary_expr(
                                    current_op,
                                    first_operand,
                                    second_operand,
                                    src_location,
                                );
                                current_op = op;
                            }

                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                current_op,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        }
                    },
                    None => Ok(initial_operand),
                }
            },
        )
    }
    fn equality_operator(input: Node) -> Result<BinaryOperator> {
        match input.as_str() {
            "!=" => Ok(BinaryOperator::NotEqual),
            "==" => Ok(BinaryOperator::Equal),
            unsupported_op@_ => {
                let msg = format!("The \"{}\" operator is unsupported", unsupported_op);
                Err(input.error(msg))
            }
        }
    }
    fn equality_helper(input: Node) -> Result<(BinaryOperator, Expr)> {
        Ok(match_nodes!(input.into_children();
            [equality_operator(op), comparison(expr)] => (op, expr),
        ))
    }
    fn comparison(input: Node) -> Result<Expr> {
        let create_binary_expr = |operator, first_operand, second_operand, src_location| 
            Expr::PrimitiveOperationExpr(
                Box::from(PrimitiveOperation::BinaryOperation {
                    operator,
                    first_operand,
                    second_operand,
                }),
                src_location,
            );

        match_nodes!(input.children();
            [term(initial_operand), comparison_helper(repetitions)..] => {
                let mut repetitions = repetitions.rev().peekable();
                match repetitions.next() {
                    Some((op, expr)) => {
                        let mut current_op = op;
                        let mut second_operand = expr;

                        if repetitions.peek().is_none() {
                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                current_op,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        } else {
                            for (op, first_operand) in repetitions {
                                let src_location = first_operand.get_source_location();
                                second_operand = create_binary_expr(
                                    current_op,
                                    first_operand,
                                    second_operand,
                                    src_location,
                                );
                                current_op = op;
                            }

                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                current_op,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        }
                    },
                    None => Ok(initial_operand),
                }
            },
        )
    }
    fn comparison_operator(input: Node) -> Result<BinaryOperator> {
        match input.as_str() {
            ">" => Ok(BinaryOperator::Greater),
            ">=" => Ok(BinaryOperator::GreaterOrEqual),
            "<" => Ok(BinaryOperator::Less),
            "<=" => Ok(BinaryOperator::LessOrEqual),
            unsupported_op@_ => {
                let msg = format!("The \"{}\" operator is unsupported", unsupported_op);
                Err(input.error(msg))
            }
        }
    }
    fn comparison_helper(input: Node) -> Result<(BinaryOperator, Expr)> {
        Ok(match_nodes!(input.into_children();
            [comparison_operator(op), term(expr)] => (op, expr),
        ))
    }
    fn term(input: Node) -> Result<Expr> {
        let create_binary_expr = |operator, first_operand, second_operand, src_location| 
            Expr::PrimitiveOperationExpr(
                Box::from(PrimitiveOperation::BinaryOperation {
                    operator,
                    first_operand,
                    second_operand,
                }),
                src_location,
            );

        match_nodes!(input.children();
            [factor(initial_operand), term_helper(repetitions)..] => {
                let mut repetitions = repetitions.rev().peekable();
                match repetitions.next() {
                    Some((op, expr)) => {
                        let mut current_op = op;
                        let mut second_operand = expr;

                        if repetitions.peek().is_none() {
                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                current_op,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        } else {
                            for (op, first_operand) in repetitions {
                                let src_location = first_operand.get_source_location();
                                second_operand = create_binary_expr(
                                    current_op,
                                    first_operand,
                                    second_operand,
                                    src_location,
                                );
                                current_op = op;
                            }

                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                current_op,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        }
                    },
                    None => Ok(initial_operand),
                }
            },
        )
    }
    fn term_operator(input: Node) -> Result<BinaryOperator> {
        match input.as_str() {
            "-" => Ok(BinaryOperator::Minus),
            "+" => Ok(BinaryOperator::Plus),
            unsupported_op@_ => {
                let msg = format!("The \"{}\" operator is unsupported", unsupported_op);
                Err(input.error(msg))
            }
        }
    }
    fn term_helper(input: Node) -> Result<(BinaryOperator, Expr)> {
        Ok(match_nodes!(input.into_children();
            [term_operator(op), factor(expr)] => (op, expr),
        ))
    }
    fn factor(input: Node) -> Result<Expr> {
        let create_binary_expr = |operator, first_operand, second_operand, src_location| 
            Expr::PrimitiveOperationExpr(
                Box::from(PrimitiveOperation::BinaryOperation {
                    operator,
                    first_operand,
                    second_operand,
                }),
                src_location,
            );

        match_nodes!(input.children();
            [unary(initial_operand), factor_helper(repetitions)..] => {
                let mut repetitions = repetitions.rev().peekable();
                match repetitions.next() {
                    Some((op, expr)) => {
                        let mut current_op = op;
                        let mut second_operand = expr;

                        if repetitions.peek().is_none() {
                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                current_op,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        } else {
                            for (op, first_operand) in repetitions {
                                let src_location = first_operand.get_source_location();
                                second_operand = create_binary_expr(
                                    current_op,
                                    first_operand,
                                    second_operand,
                                    src_location,
                                );
                                current_op = op;
                            }

                            let src_location = initial_operand.get_source_location();
                            Ok(create_binary_expr(
                                current_op,
                                initial_operand,
                                second_operand,
                                src_location,
                            ))
                        }
                    },
                    None => Ok(initial_operand),
                }
            },
        )
    }
    fn factor_operator(input: Node) -> Result<BinaryOperator> {
        match input.as_str() {
            "/" => Ok(BinaryOperator::Divide),
            "*" => Ok(BinaryOperator::Times),
            unsupported_op@_ => {
                let msg = format!("The \"{}\" operator is unsupported", unsupported_op);
                Err(input.error(msg))
            }
        }
    }
    fn factor_helper(input: Node) -> Result<(BinaryOperator, Expr)> {
        Ok(match_nodes!(input.into_children();
            [factor_operator(op), unary(expr)] => (op, expr),
        ))
    }
    fn unary(input: Node) -> Result<Expr> {
        let create_unary_expr = |operator, operand, line, col| Expr::PrimitiveOperationExpr(
            Box::from(PrimitiveOperation::UnaryOperation {
                operator,
                operand,
            }),
            SourceLocation { line, col },
        );

        let (line, col) = input.as_span().start_pos().line_col();

        Ok(match_nodes!(input.into_children();
            [unary_operator(op), unary(expr)] 
                => create_unary_expr(op, expr, line, col),
            [primary(expr)] => expr,
        ))
    }
    fn unary_operator(input: Node) -> Result<UnaryOperator> {
        match input.as_str() {
            "!" => Ok(UnaryOperator::Not),
            "-" => Ok(UnaryOperator::UnaryMinus),
            "&mut " => Ok(UnaryOperator::MutableBorrow),
            "&" => Ok(UnaryOperator::ImmutableBorrow),
            "*" => Ok(UnaryOperator::Dereference),
            unsupported_op@_ => {
                let msg = format!("The \"{}\" operator is unsupported", unsupported_op);
                Err(input.error(msg))
            }
        }
    }
    fn return_val(input: Node) -> Result<Expr> {
        let (line, col) = input.as_span().start_pos().line_col();
        Ok(match_nodes!(input.into_children();
            [expr(expr)] => Expr::ReturnExpr(
                Box::from(expr),
                SourceLocation { line, col },
            ),
        ))
    }
    fn identifier(input: Node) -> Result<Expr> {
        let (line, col) = input.as_span().start_pos().line_col();
        let identifier = String::from(input.as_str());
        let ident_expr = Expr::IdentifierExpr(
            identifier,
            SourceLocation { line, col }
        );
        Ok(ident_expr)
    }
    // TODO: Shion.
    fn function_declaration(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn function_return_type(input: Node) -> Result<DataType> {
        Ok(match_nodes!(input.into_children();
            [datatype(d)] => d,
        ))
    }
    fn lifetime_param_list(input: Node) -> Result<Vec<LifetimeParameter>> {
        input.into_children()
            .map(OxidoParser::lifetime_type_variable)
            .collect()
    }
    fn lifetime_type_variable(input: Node) -> Result<LifetimeParameter> {
        Ok(String::from(input.as_str()))
    }
    fn function_param_list(input: Node) -> Result<Vec<FuncParameter>> {
        input.into_children()
            .map(OxidoParser::function_param)
            .collect()
    }
    fn function_param(input: Node) -> Result<FuncParameter> {
        match_nodes!(input.children();
            [identifier(ident), datatype(param_type)] => {
                if let Expr::IdentifierExpr(name, _) = ident {
                    Ok((name, param_type))
                } else {
                    Err(input.error("Function parameter should start with an identifier"))
                }
            },
        )
    }
    fn function_app(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn function_arg_list(input: Node) -> Result<Vec<Expr>> {
        input.into_children()
            .map(OxidoParser::expr)
            .collect()
    }
    fn boolean_literal(input: Node) -> Result<Expr> {
        input.as_str()
            .trim()
            .parse::<bool>()
            .map(|b| -> Expr {
                let (line, col) = input.as_span().start_pos().line_col();
                Expr::LiteralExpr(
                    Literal::BoolLiteral(b),
                    SourceLocation { line, col }
                )
            })
            .map_err(|e| input.error(e))
    }
    fn integer_literal(input: Node) -> Result<Expr> {
        input.as_str()
            .trim()
            .parse::<i64>()
            .map(|i| -> Expr {
                let (line, col) = input.as_span().start_pos().line_col();
                Expr::LiteralExpr(
                    Literal::IntLiteral(i),
                    SourceLocation { line, col }
                )
            })
            .map_err(|e| input.error(e))
    }
    fn string_literal(input: Node) -> Result<Expr> {
        let (line, col) = input.as_span().start_pos().line_col();
        let s = input.into_children().as_pairs().as_str();
        let str_expr = Expr::LiteralExpr(
            Literal::StringLiteral(String::from(s)),
            SourceLocation { line, col }
        );
        Ok(str_expr)
    }
}

pub fn parse(program: &str) -> Result<Vec<Expr>> {
    // let program = format!("{{ {} }}", program);
    let inputs = OxidoParser::parse(Rule::function_arg_list, &program)?;
    OxidoParser::function_arg_list(inputs.single()?)
}

