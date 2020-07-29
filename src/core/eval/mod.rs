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
    assert_eq!("16".to_string(), exec("8 << 1").expect("Failed to run"))
}

#[test]
fn integration_test12() {
    assert_eq!("8".to_string(), exec("16 >> 1").expect("Failed to run"))
}

#[test]
fn integration_test13() {
    assert_eq!("18".to_string(), exec("10 ^ 24").expect("Failed to run"))
}

#[test]
fn integration_test14() {
    assert_eq!("26".to_string(), exec("10 | 24").expect("Failed to run"))
}

#[test]
fn integration_test15() {
    assert_eq!("8".to_string(), exec("10 & 24").expect("Failed to run"))
}

#[test]
fn integration_test16() {
    assert_eq!("-25".to_string(), exec("~24").expect("Failed to run"))
}

#[test]
fn integration_test17() {
    assert_eq!("false".to_string(), exec("false && true").expect("Failed to run"))
}

#[test]
fn integration_test18() {
    assert_eq!("true".to_string(), exec("false || true").expect("Failed to run"))
}

#[test]
fn integration_test19() {
    assert_eq!("false".to_string(), exec("!true").expect("Failed to run"))
}

#[test]
fn integration_test20() {
    assert_eq!("true".to_string(), exec("!false").expect("Failed to run"))
}

#[test]
fn integration_test21() {
    assert_eq!("false".to_string(), exec("10 == 24").expect("Failed to run"))
}

#[test]
fn integration_test22() {
    assert_eq!("true".to_string(), exec("10 == 10").expect("Failed to run"))
}

#[test]
fn integration_test23() {
    assert_eq!("true".to_string(), exec("10 != 24").expect("Failed to run"))
}

#[test]
fn integration_test24() {
    assert_eq!("false".to_string(), exec("10 != 10").expect("Failed to run"))
}

#[test]
fn integration_test25() {
    assert_eq!("false".to_string(), exec("10 > 24").expect("Failed to run"))
}

#[test]
fn integration_test26() {
    assert_eq!("true".to_string(), exec("10 < 24").expect("Failed to run"))
}

#[test]
fn integration_test27() {
    assert_eq!("true".to_string(), exec("10 >= 10").expect("Failed to run"))
}

#[test]
fn integration_test28() {
    assert_eq!("true".to_string(), exec("10 <= 10").expect("Failed to run"))
}

#[test]
fn integration_test29() {
    assert_eq!("3".to_string(), exec("pi as int").expect("Failed to run"))
}

#[test]
fn integration_test30() {
    assert_eq!("1".to_string(), exec("11 % 10").expect("Failed to run"))
}

#[test]
fn integration_test31() {
    assert_eq!("10.5".to_string(), exec("1123123.5 % 123.0").expect("Failed to run"))
}

#[test]
fn integration_test32() {
    assert!(exec("10.1 << 1").is_err());
}

#[test]
fn integration_test33() {
    assert!(exec("10.1 >> 1").is_err());
}

#[test]
fn integration_test34() {
    assert!(exec("true >= false").is_err());
}

#[test]
fn integration_test35() {
    assert!(exec("false <= true").is_err());
}

#[test]
fn integration_test36() {
    assert!(exec("false > false").is_err());
}

#[test]
fn integration_test37() {
    assert!(exec("true < true").is_err());
}

#[test]
fn integration_test38() {
    assert!(exec("120391203918441204981*123212319382148102482948").is_err());
}

#[test]
fn integration_test39() {
    assert!(exec("1230<<123213").is_err());
}

#[test]
fn integration_test40() {
    assert!(exec("170141183460469231731687303715884105726+100").is_err());
}

#[test]
fn integration_test41() {
    assert!(exec("1230>>123213").is_err());
}

