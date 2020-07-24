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
fn integration_test1() {
    assert_eq!("25".to_string(), exec("+1 * 2 + 3 - 4 * -5").expect("Failed to run"));
}

#[test]
fn integration_test2() {
    assert_eq!("10000000100".to_string(), exec("10*10+10**10").expect("Failed to run"));
}

#[test]
fn integration_test3() {
    assert_eq!("2".to_string(), exec("(100/53) as int").expect("Failed to run"));
}

#[test]
fn integration_test4() {
    assert!(exec("(100/53) as int + (100/53)").is_err());
}

#[test]
fn integration_test5() {
    assert_eq!("2.5".to_string(), exec("1.0 + 1.5").expect("Failed to run"));
}

#[test]
fn integration_test6() {
    assert!(exec("10.1 + 1").is_err());
}

#[test]
fn integration_test7() {
    assert!(exec("10.1 * 1").is_err());
}

#[test]
fn integration_test8() {
    assert!(exec("10.1 ** 1").is_err());
}

#[test]
fn integration_test9() {
    assert!(exec("10.1 / 1").is_err());
}

#[test]
fn integration_test10() {
    assert!(exec("10.1 // 1").is_err());
}

#[test]
fn integration_test11() {
    assert_eq!("16".to_string(), exec("8 << 1").expect("Failed to  run"))
}

#[test]
fn integration_test12() {
    assert_eq!("8".to_string(), exec("16 >> 1").expect("Failed to  run"))
}

#[test]
fn integration_test13() {
    assert_eq!("18".to_string(), exec("10 ^ 24").expect("Failed to  run"))
}

#[test]
fn integration_test14() {
    assert_eq!("26".to_string(), exec("10 | 24").expect("Failed to  run"))
}

#[test]
fn integration_test15() {
    assert_eq!("8".to_string(), exec("10 & 24").expect("Failed to  run"))
}

#[test]
fn integration_test16() {
    assert_eq!("-25".to_string(), exec("~24").expect("Failed to  run"))
}

#[test]
fn integration_test17() {
    assert_eq!("false".to_string(), exec("false && true").expect("Failed to  run"))
}

#[test]
fn integration_test18() {
    assert_eq!("true".to_string(), exec("false || true").expect("Failed to  run"))
}

#[test]
fn integration_test19() {
    assert_eq!("false".to_string(), exec("!true").expect("Failed to  run"))
}

#[test]
fn integration_test20() {
    assert_eq!("true".to_string(), exec("!false").expect("Failed to  run"))
}

