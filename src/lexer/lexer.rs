use lexer::token::*;

#[derive(Debug)]
pub struct Lexer<'a> {
    pub bytes: &'a [u8],
    pub position: usize,
    pub current_row: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let bytes = input.as_bytes();
        Lexer {
            bytes: bytes,
            position: 0,
            current_row: 0,
        }
    }

    pub fn create_eof_token(&mut self) -> Token {
        Token::new(TokenType::Eof, String::new(), self.position)
    }

    pub fn handle_reserved_word(&self, word: &str, token: TokenType) -> TokenType {
        match word {
            "while" => TokenType::While,
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            "break" => TokenType::Break,
            "boolean" => TokenType::LLVMTokenType(LLVMTokenType::Boolean),
            "int" => TokenType::LLVMTokenType(LLVMTokenType::Int),
            "string" => TokenType::LLVMTokenType(LLVMTokenType::String),
            "null" => TokenType::LLVMTokenType(LLVMTokenType::Null),
            _ => token,
        }
    }

    pub fn get_next_char(&mut self) -> Option<u8> {
        if self.position < self.bytes.len() {
            return Some(self.bytes[self.position]);
        }
        None
    }

    pub fn consume_comment(&mut self) {
        loop {
            if let Some(byte) = self.get_next_char() {
                self.position += 1;
                if byte == b'*' {
                    if let Some(next) = self.get_next_char() {
                        if next == b'/' {
                            self.position += 1;
                            break;
                        }
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn consumue_character(&mut self, first_byte: u8, mut num_flag: bool) -> Token {
        let mut temp_vec: Vec<u8> = Vec::new();
        temp_vec.push(first_byte);
        loop {
            if let Some(byte) = self.get_next_char() {
                let break_flg = match byte {
                    b'0'...b'9' => {
                        self.position += 1;
                        temp_vec.push(byte);
                        false
                    }
                    b'a'...b'z' | b'A'...b'Z' => {
                        self.position += 1;
                        temp_vec.push(byte);
                        num_flag = false;
                        false
                    }
                    _ => true,
                };

                if break_flg == true {
                    break;
                }
            } else {
                break;
            }
        }

        let token_type = if num_flag == true {
            TokenType::Digit
        } else {
            TokenType::Identifier
        };

        self.create_token_by_value(token_type, temp_vec)
    }

    pub fn create_token_by_value(&mut self, token: TokenType, value_vec: Vec<u8>) -> Token {
        let ret_string = String::from_utf8(value_vec).unwrap();
        Token::new(
            self.handle_reserved_word(&ret_string, token),
            ret_string.to_owned(),
            self.current_row,
        )
    }

    pub fn consume_slash(&mut self, target_token: Token) -> (Token, bool) {
        if let Some(next) = self.get_next_char() {
            if next == b'*' {
                self.position += 1;
                self.consume_comment();
                return (target_token, false);
            }
        }
        (
            self.create_token_by_value(TokenType::Divide, vec![b'/']),
            true,
        )
    }

    pub fn consume_equal(&mut self) -> Token {
        if let Some(next) = self.get_next_char() {
            if next == b'=' {
                self.position += 1;
                return self.create_token_by_value(TokenType::Eq, vec![b'=', b'=']);
            }
        }
        self.create_token_by_value(TokenType::Assign, vec![b'='])
    }

    pub fn consume_ban(&mut self) -> Token {
        if let Some(next) = self.get_next_char() {
            if next == b'=' {
                self.position += 1;
                return self.create_token_by_value(TokenType::NotEq, vec![b'!', b'=']);
            }
        }
        self.create_token_by_value(TokenType::Bang, vec![b'!'])
    }

    pub fn consume_lt(&mut self) -> Token {
        if let Some(next) = self.get_next_char() {
            if next == b'=' {
                self.position += 1;
                return self.create_token_by_value(TokenType::Lte, vec![b'<', b'=']);
            }
        }
        self.create_token_by_value(TokenType::Lt, vec![b'<'])
    }

    pub fn consume_gt(&mut self) -> Token {
        if let Some(next) = self.get_next_char() {
            if next == b'=' {
                self.position += 1;
                return self.create_token_by_value(TokenType::Gte, vec![b'>', b'=']);
            }
        }
        self.create_token_by_value(TokenType::Gt, vec![b'>'])
    }

    pub fn consume_string(&mut self) -> Token {
        let mut char_vec = Vec::new();
        loop {
            if let Some(next_char) = self.get_next_char() {
                self.position += 1;
                if next_char == b'"' {
                    break;
                }
                char_vec.push(next_char);
            } else {
                break;
            }
        }
        self.create_token_by_value(TokenType::String, char_vec)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut ret_val: Token = self.create_eof_token();
        loop {
            if let Some(byte) = self.get_next_char() {
                self.position += 1;
                let flag = match byte {
                    b'0'...b'9' => {
                        ret_val = self.consumue_character(byte, true);
                        true
                    }
                    b'a'...b'z' | b'A'...b'Z' => {
                        ret_val = self.consumue_character(byte, false);
                        true
                    }
                    b'"' => {
                        ret_val = self.consume_string();
                        true
                    }
                    b'/' => {
                        let (temp_ret, flag) = self.consume_slash(ret_val);
                        ret_val = temp_ret;
                        flag
                    }
                    b'=' => {
                        ret_val = self.consume_equal();
                        true
                    }
                    b',' => {
                        ret_val = self.create_token_by_value(TokenType::Comma, vec![byte]);
                        true
                    }
                    b'.' => {
                        ret_val = self.create_token_by_value(TokenType::Period, vec![byte]);
                        true
                    }
                    b'{' => {
                        ret_val = self.create_token_by_value(TokenType::Lbrace, vec![byte]);
                        true
                    }
                    b'}' => {
                        ret_val = self.create_token_by_value(TokenType::Rbrace, vec![byte]);
                        true
                    }
                    b'(' => {
                        ret_val = self.create_token_by_value(TokenType::Lparen, vec![byte]);
                        true
                    }
                    b')' => {
                        ret_val = self.create_token_by_value(TokenType::Rparen, vec![byte]);
                        true
                    }
                    b'[' => {
                        ret_val = self.create_token_by_value(TokenType::Lbracket, vec![byte]);
                        true
                    }
                    b']' => {
                        ret_val = self.create_token_by_value(TokenType::Rbracket, vec![byte]);
                        true
                    }
                    b'!' => {
                        ret_val = self.consume_ban();
                        true
                    }
                    b'*' => {
                        ret_val = self.create_token_by_value(TokenType::Multiply, vec![byte]);
                        true
                    }
                    b'%' => {
                        ret_val = self.create_token_by_value(TokenType::Rem, vec![byte]);
                        true
                    }
                    b'+' => {
                        ret_val = self.create_token_by_value(TokenType::Plus, vec![byte]);
                        true
                    }
                    b'-' => {
                        ret_val = self.create_token_by_value(TokenType::Minus, vec![byte]);
                        true
                    }
                    b'<' => {
                        ret_val = self.consume_lt();
                        true
                    }
                    b'>' => {
                        ret_val = self.consume_gt();
                        true
                    }
                    b':' => {
                        ret_val = self.create_token_by_value(TokenType::Colon, vec![byte]);
                        true
                    }
                    b';' => {
                        ret_val = self.create_token_by_value(TokenType::Semicolon, vec![byte]);
                        true
                    }
                    b'\n' | b'\r' => {
                        self.current_row += 1;
                        false
                    }
                    b' ' => false,
                    _ => {
                        panic!("{} cannot be handled.", byte);
                    }
                };

                if flag == true {
                    break;
                }
            } else {
                return None;
            }
        }
        Some(ret_val)
    }
}

// below is test implementation

#[allow(dead_code)]
fn lexer_assert(token: Token, token_type: TokenType, value: &str) {
    let expected = Token::new(token_type, value.to_string(), 0);
    assert!(
        token == expected,
        "\r\nexpected: {:?} \r\nactual: {:?}",
        expected,
        token
    );
}

#[test]
fn digit() {
    let mut lexer = Lexer::new(
        r#"
    123 456
    "#,
    );
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "123");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "456");
}

#[test]
fn identifier() {
    let mut lexer = Lexer::new(
        r#"
    123 abc 45d6
    "#,
    );
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "123");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Identifier, "abc");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Identifier, "45d6");
}

#[test]
fn string() {
    let mut lexer = Lexer::new(
        r#"
    "abc" "def"
    "#,
    );
    lexer_assert(lexer.next_token().unwrap(), TokenType::String, "abc");
    lexer_assert(lexer.next_token().unwrap(), TokenType::String, "def");
}

#[test]
fn array() {
    let mut lexer = Lexer::new(
        r#"
    [ 1, 2, 3 ]
    "#,
    );
    lexer_assert(lexer.next_token().unwrap(), TokenType::Lbracket, "[");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "1");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Comma, ",");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "2");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Comma, ",");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "3");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Rbracket, "]");
}

#[test]
fn comment() {
    let mut lexer = Lexer::new(
        r#"
    0 /* 123 */ 2
    "#,
    );
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "0");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "2");
}

#[test]
fn ban() {
    let mut lexer = Lexer::new(
        r#"
    let abc = !abc
    "#,
    );
    lexer_assert(lexer.next_token().unwrap(), TokenType::Let, "let");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Identifier, "abc");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Assign, "=");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Bang, "!");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Identifier, "abc");
}

#[test]
fn division_multiple() {
    let mut lexer = Lexer::new(
        r#"
    1 / 323 * 3 / 2
    "#,
    );
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "1");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Divide, "/");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "323");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Multiply, "*");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "3");
}

#[test]
fn gt() {
    let mut lexer = Lexer::new(
        r#"
    123 <= 456
    "#,
    );
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "123");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Lte, "<=");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "456");
}

#[test]
fn if_test() {
    let mut lexer = Lexer::new(
        r#"
    if 123 == 456
    "#,
    );
    lexer_assert(lexer.next_token().unwrap(), TokenType::If, "if");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "123");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Eq, "==");
    lexer_assert(lexer.next_token().unwrap(), TokenType::Digit, "456");
}

#[test]
fn llvm_token_test() {
    let mut lexer = Lexer::new(
        r#"
    int string boolean null
    "#,
    );
    lexer_assert(
        lexer.next_token().unwrap(),
        TokenType::LLVMTokenType(LLVMTokenType::Int),
        "int",
    );
    lexer_assert(
        lexer.next_token().unwrap(),
        TokenType::LLVMTokenType(LLVMTokenType::String),
        "string",
    );
    lexer_assert(
        lexer.next_token().unwrap(),
        TokenType::LLVMTokenType(LLVMTokenType::Boolean),
        "boolean",
    );
    lexer_assert(
        lexer.next_token().unwrap(),
        TokenType::LLVMTokenType(LLVMTokenType::Null),
        "null",
    );
}
