use crate::core::eval::ast;

pub(crate) struct Lexer<'a> {
    chars_peek: std::iter::Peekable<std::str::Chars<'a>>,
    file_contents: &'a str,
    pos: usize,
}

const EOF_CHAR: char = '\0';

/// Check if character is a whitespace character
pub fn is_whitespace(c: char) -> bool {
    match c {
        | '\u{0009}' // \t
            | '\u{000A}' // \n
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space

            // NEXT LINE from latin1
            | '\u{0085}'

            // Bidi markers
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK

            // Dedicated whitespace characters from Unicode
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}' // PARAGRAPH SEPARATOR
            => true,
        _ => false,
    }
}

/// Check if ID is continue
fn is_id_continue(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || ('0' <= c && c <= '9') || c == '_'
}

/// Get start of ID (excluding number)
fn is_id_start(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_'
}

macro_rules! double_match {
    ($tokens: ident, $self: ident, $first: expr, $($second: expr => $op_type: expr),*) => {
        match $self.peek_char() {
            $(
                $second => {
                    $tokens.push($self.double_op($op_type));
                }
             )*
            _ => { $tokens.push($self.new_literal($first)); }
        }
    }
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(file_contents: &'a str) -> Self {
        Self {
            chars_peek: file_contents.chars().peekable(),
            file_contents,
            pos: 0,
        }
    }

    /// Advances in character stream
    fn bump_char(&mut self) -> char {
        self.pos += 1;
        self.chars_peek.next().unwrap_or(EOF_CHAR)
    }

    /// Doesn't advance
    fn peek_char(&mut self) -> char {
        *self.chars_peek.peek().unwrap_or(&EOF_CHAR)
    }

    fn integer(&mut self) -> ast::Token<'a> {
        let next_len = self.len_eat_while(|c| '0' <= c && c <= '9') + 1;
        ast::Token::new(
            ast::TokenType::Integer,
            &self.file_contents[self.pos - next_len..self.pos],
            self.pos - next_len,
            self.pos,
        )
    }

    fn identifier(&mut self) -> ast::Token<'a> {
        let next_len = self.len_eat_while(|c| is_id_continue(c)) + 1;
        let ident = &self.file_contents[self.pos - next_len..self.pos];
        ast::Token::new(
            match ident {
                "as" => ast::TokenType::Operator(ast::Operator::As),
                _ => ast::TokenType::Identifier
            },
            ident,
            self.pos - next_len,
            self.pos,
        )
    }

    fn len_eat_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let mut eaten: usize = 0;
        let mut val = self.peek_char();
        while predicate(val) && val != EOF_CHAR {
            self.bump_char();
            eaten += 1;
            val = self.peek_char();
        }

        eaten
    }

    /// One character literals
    fn new_literal(&mut self, c: char) -> ast::Token<'a> {
        ast::Token::new(
            ast::TokenType::Operator(match c {
                '+' => ast::Operator::Add,
                '-' => ast::Operator::Sub,
                '/' => ast::Operator::Div,
                '*' => ast::Operator::Mul,
                '%' => ast::Operator::Mod,

                '&' => ast::Operator::BAnd,
                '|' => ast::Operator::BOr,
                '~' => ast::Operator::BNot,
                '!' => ast::Operator::LNot,

                '>' => ast::Operator::GT,
                '<' => ast::Operator::LT,

                '^' => ast::Operator::BXor,

                _ => panic!("Bad operator given!"),
            }),
            "",
            self.pos - 1,
            self.pos,
        )
    }

    fn double_op(&mut self, tok_type: ast::Operator) -> ast::Token<'a> {
        let tok = ast::Token::new(
            ast::TokenType::Operator(tok_type),
            "",
            self.pos - 1,
            self.pos + 1,
        );
        self.bump_char();
        tok
    }

    fn e(current: char) -> Result<(), String> {
        Err(format!("Unrecognized character {}", current))
    }

    pub(crate) fn tokenize(&mut self) -> Result<Vec<ast::Token<'a>>, String> {
        let mut tokens: Vec<ast::Token<'a>> = Vec::new();
        let mut current = self.bump_char();
        while current != EOF_CHAR {
            match current {
                '0'..='9' => tokens.push(self.integer()),

                c if is_whitespace(c) => {
                    // Character is whitespace
                    // Just do nothing here
                }

                c if is_id_start(c) => {
                    // Start of id
                    tokens.push(self.identifier());
                }

                '+' | '-' | '~' | '^' | '%' => {
                    tokens.push(self.new_literal(current));
                }

                '!' => double_match! {
                    tokens, self,
                    '!',
                    '=' => ast::Operator::NEql
                },

                '|' => double_match! {
                    tokens, self,
                    '|',
                    '|' => ast::Operator::LOr
                },

                '&' => double_match! {
                    tokens, self,
                    '&',
                    '&' => ast::Operator::LAnd
                },

                '*' => double_match! {
                    tokens, self,
                    '*',
                    '*' => ast::Operator::Pow
                },

                '/' => double_match! {
                    tokens, self,
                    '/',
                    '/' => ast::Operator::IntDiv
                },

                '=' => match self.peek_char() {
                    '=' => {
                        tokens.push(self.double_op(ast::Operator::Eql));
                    }
                    _ => Self::e(current)?,
                },

                '>' => double_match! {
                    tokens, self,
                    '>',
                    '=' => ast::Operator::GE,
                    '>' => ast::Operator::BitShiftR
                },

                '<' => double_match! {
                    tokens, self,
                    '<',
                    '=' => ast::Operator::LE,
                    '<' => ast::Operator::BitShiftL
                },

                _ => Self::e(current)?,
            }
            current = self.bump_char();
        }

        Ok(tokens)
    }
}

#[test]
fn integer() {
    let tokens = Lexer::new("12831984").tokenize().expect("Failed to parse");
    assert_eq!(tokens[0].value, "12831984");
}

#[test]
fn integer_ws() {
    let tokens = Lexer::new("1283 1984").tokenize().expect("Failed to parse");
    assert_eq!(tokens[0].value, "1283");
    assert_eq!(tokens[1].value, "1984")
}

#[test]
fn integer_ws_single() {
    let tokens = Lexer::new("1 9").tokenize().expect("Failed to parse");
    assert_eq!(tokens[0].value, "1");
    assert_eq!(tokens[1].value, "9")
}

#[test]
fn integer_single() {
    let tokens = Lexer::new("8").tokenize().expect("Failed to parse");
    assert_eq!(tokens[0].value, "8");
}

#[test]
fn indent() {
    let tokens = Lexer::new("pi * 8 as float ** 2").tokenize().expect("Failed to parse");
    assert_eq!(tokens[0].value, "pi");
    assert_eq!(tokens[0].tok_type, ast::TokenType::Identifier);
    assert_eq!(
        tokens[1].tok_type,
        ast::TokenType::Operator(ast::Operator::Mul)
    );

    assert_eq!(tokens[2].value, "8");

    assert_eq!(
        tokens[3].tok_type,
        ast::TokenType::Operator(ast::Operator::As)
    );

    assert_eq!(tokens[4].value, "float");
    assert_eq!(tokens[4].tok_type, ast::TokenType::Identifier);

    assert_eq!(
        tokens[5].tok_type,
        ast::TokenType::Operator(ast::Operator::Pow)
    );

    assert_eq!(tokens[6].value, "2");
}

#[test]
fn integer_op() {
    let tokens = Lexer::new("// && & | || + - == != / > < >> << >= <= ! ~ ** ^ %")
        .tokenize()
        .expect("Failed to parse");

    assert_eq!(
        tokens[0].tok_type,
        ast::TokenType::Operator(ast::Operator::IntDiv)
    );
    assert_eq!(
        tokens[1].tok_type,
        ast::TokenType::Operator(ast::Operator::LAnd)
    );
    assert_eq!(
        tokens[2].tok_type,
        ast::TokenType::Operator(ast::Operator::BAnd)
    );
    assert_eq!(
        tokens[3].tok_type,
        ast::TokenType::Operator(ast::Operator::BOr)
    );
    assert_eq!(
        tokens[4].tok_type,
        ast::TokenType::Operator(ast::Operator::LOr)
    );
    assert_eq!(
        tokens[5].tok_type,
        ast::TokenType::Operator(ast::Operator::Add)
    );
    assert_eq!(
        tokens[6].tok_type,
        ast::TokenType::Operator(ast::Operator::Sub)
    );
    assert_eq!(
        tokens[7].tok_type,
        ast::TokenType::Operator(ast::Operator::Eql)
    );
    assert_eq!(
        tokens[8].tok_type,
        ast::TokenType::Operator(ast::Operator::NEql)
    );
    assert_eq!(
        tokens[9].tok_type,
        ast::TokenType::Operator(ast::Operator::Div)
    );
    assert_eq!(
        tokens[10].tok_type,
        ast::TokenType::Operator(ast::Operator::GT)
    );
    assert_eq!(
        tokens[11].tok_type,
        ast::TokenType::Operator(ast::Operator::LT)
    );
    assert_eq!(
        tokens[12].tok_type,
        ast::TokenType::Operator(ast::Operator::BitShiftR)
    );
    assert_eq!(
        tokens[13].tok_type,
        ast::TokenType::Operator(ast::Operator::BitShiftL)
    );
    assert_eq!(
        tokens[14].tok_type,
        ast::TokenType::Operator(ast::Operator::GE)
    );
    assert_eq!(
        tokens[15].tok_type,
        ast::TokenType::Operator(ast::Operator::LE)
    );
    assert_eq!(
        tokens[16].tok_type,
        ast::TokenType::Operator(ast::Operator::LNot)
    );
    assert_eq!(
        tokens[17].tok_type,
        ast::TokenType::Operator(ast::Operator::BNot)
    );
    assert_eq!(
        tokens[18].tok_type,
        ast::TokenType::Operator(ast::Operator::Pow)
    );
    assert_eq!(
        tokens[19].tok_type,
        ast::TokenType::Operator(ast::Operator::BXor)
    );
    assert_eq!(
        tokens[20].tok_type,
        ast::TokenType::Operator(ast::Operator::Mod)
    );
}
