use pest_consume::{match_nodes, Error, Parser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct OxidoParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl OxidoParser {
    fn declaration(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn mutable_specifier(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn datatype(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn reference_datatype(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn block(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn sequence(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn stmt(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn expr_stmt(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn expr(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn primary(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn assignment(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn disjunction(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn conjunction(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn equality(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn comparison(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn term(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn factor(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn unary(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn return_val(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn identifier(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn function_declaration(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn function_return_type(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn lifetime_param_list(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn lifetime_type_variable(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn function_param_list(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn function_param(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn function_app(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn function_arg_list(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn boolean_literal(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn integer_literal(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn string_literal(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn inner(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
    fn char(input: Node) -> Result<()> {
        println!("{:#?}", input);
        Ok(())
    }
}

pub fn parse(program: &str) -> Result<()> {
    let program = format!("{{ {} }}", program);
    let inputs = OxidoParser::parse(Rule::block, &program)?;
    OxidoParser::block(inputs.single()?)
}