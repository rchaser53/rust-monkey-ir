#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TokenType {
    Identifier,
    Digit,

    Eof,
    Assign,
    Colon,
    Comma,
    Period,
    Semicolon,

    String,

    Lparen, // (
    Rparen, // )
    Lbrace, // {
    Rbrace, // }

    // preserve word
    Fn,     // fn
    True,   // true
    False,  // false
    If,     // if
    Else,   // else
    Let,    // let
    Return, // return
    While,  // while
    Break,  // break

    // for Arithmetic
    Eq,       // =
    NotEq,    // !=
    Lt,       // <
    Lte,      // <=
    Gt,       // >
    Gte,      // >=
    Plus,     // +
    Minus,    // -
    Divide,   // /
    Multiply, // *
    Bang,     // !
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub current_row: usize,
}

impl Token {
    pub fn new(kind: TokenType, value: String, row: usize) -> Token {
        Token {
            kind: kind,
            value: value,
            current_row: row,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}
