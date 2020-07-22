#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,

    Pow,

    Eql,  // Equal
    NEql, // Not equal

    BAnd, // Bitwise and
    BOr,  // Bitwise or

    LAnd, // Logical or
    LOr,  // Logical and

    BitShiftR, // Right bit shift
    BitShiftL, // Right bit shift
}

#[derive(Debug, PartialEq)]
enum Expression {
    BinOp(BinOp),
    InfixOp(InfixOp),
}

#[derive(Debug, PartialEq)]
struct BinOp {
    op: Operator,
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
struct InfixOp {
    op: Operator,
    value: Box<Expression>,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Integer,
    Operator(Operator)
}

#[derive(Debug, PartialEq)]
struct Token<'a> {
    tok_type: TokenType,
    value: &'a str,
    pos_start: usize,
    pos_end: usize
}

impl<'a> Token<'a> {
    fn new(tok_type: TokenType, value: &'a str, pos_start: usize, pos_end: usize) -> Self {
        Token {
            tok_type,
            value,
            pos_start,
            pos_end
        }
    }
}

struct Lexer<'a> {
    chars_peek: std::iter::Peekable<std::str::Chars<'a>>,
    file_contents: &'a str,
    pos: usize
}

const EOF_CHAR: char = '\0';

impl<'a> Lexer<'a> {
    fn new(file_contents: &'a str) -> Self {
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

    fn integer(&mut self) -> Token<'a> {
        let next_len = self.len_eat_while(|c| '0' <= c && c <= '9');
        Token::new(TokenType::Integer, &self.file_contents[self.pos-next_len..self.pos], self.pos-next_len, self.pos)
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

    fn tokenize(&mut self) -> Result<Vec<Token<'a>>, String> {
        let mut tokens: Vec<Token<'a>> = Vec::new();
        let mut current = self.bump_char();
        while current != EOF_CHAR {
            match current {
                '0'..='9' => {
                    tokens.push(self.integer())
                }
                _ => {
                    return Err(format!("Unrecognized character {}", current))
                }
            }
            current = self.bump_char();
        }

        Ok(tokens)
    }
}

/// Run some math expr
pub(crate) fn exec<'a>(value: &'a str) -> String {
    let tokens = match Lexer::new(value).tokenize() {
        Ok(val) => val,
        Err(why) => { return why; }
    };
    String::new()
}

#[test]
fn test_lexer() {
    println!("{:?}", Lexer::new("1984").tokenize());
}

