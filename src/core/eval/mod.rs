pub(crate) mod ast;
pub(crate) mod error;
mod lexer;
mod parser;
mod exec;

use error::Error;

/// Run some math expr
pub(crate) fn exec<'a>(value: &'a str) -> Result<String, Error> {
    let tokens = lexer::Lexer::new(value).tokenize()?;
    let ast = parser::Parser::new(tokens).expr(0)?;
    exec::Executer::new().eval(ast)
}

#[test]
fn parser_test() {
    let tokens = lexer::Lexer::new("1 * 2 + 3 - 4 // -5").tokenize().expect("Failed to lex");
    let mut parser = parser::Parser::new(tokens);
    parser.config();
    let ast = parser.expr(0);
    println!("{:#?}", ast);
}
