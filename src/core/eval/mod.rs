pub(crate) mod ast;
pub(crate) mod error;
mod lexer;
mod parser;
mod exec;

use error::Error;

/// Run some math expr
pub(crate) fn exec<'a>(value: &'a str) -> Result<String, Error> {
    let tokens = lexer::Lexer::new(value).tokenize()?;
    let mut parser = parser::Parser::new(tokens);
    parser.config();
    let ast = parser.expr(0)?;
    exec::Executer::new().eval(ast).map(|expr| expr.to_string())
}

#[test]
fn integration_test() {
    println!("{}", exec("1 * 2 + 3 - 4 * -5").expect("Failed to run"));
}


