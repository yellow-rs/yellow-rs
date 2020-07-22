pub(crate) mod ast;
mod lexer;

/// Run some math expr
pub(crate) fn exec<'a>(value: &'a str) -> String {
    let tokens = match lexer::Lexer::new(value).tokenize() {
        Ok(val) => val,
        Err(why) => {
            return why;
        }
    };
    String::new()
}
