#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TokenType {
    LLVMTokenType(LLVMTokenType),
    Identifier,
    Digit,

    Eof,
    Assign,
    Colon,
    Comma,
    Period,
    Semicolon,

    String,

    Lparen,   // (
    Rparen,   // )
    Lbrace,   // {
    Rbrace,   // }
    Lbracket, // [
    Rbracket, // ]

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
    Rem,      // %
    Bang,     // !
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LLVMTokenType {
    Int,
    String,
    Boolean,
    Null,
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
