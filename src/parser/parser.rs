use lexer::lexer::*;
use lexer::token::*;

use parser::expressions::*;
use parser::precedence::*;
use parser::statements::*;

pub struct Parser<'a> {
    pub l: &'a mut Lexer<'a>,
    pub cur_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer<'a>) -> Parser<'a> {
        let current_token = l.next_token();
        let peek_token = l.next_token();

        Parser {
            l: l,
            cur_token: current_token,
            peek_token: peek_token,
            errors: Vec::new(),
        }
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.to_owned();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();

        while self.cur_token != None {
            if let Some(stmt) = self.parse_statement() {
                program.push(stmt);
            }
            self.next_token();
        }
        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(token) = self.cur_token.to_owned() {
            return match token.kind {
                TokenType::Let => self.parse_let_statement(),
                TokenType::Return => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            };
        } else {
            return None;
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        if self.expect_peek(TokenType::Identifier) == false {
            return None;
        }

        if let Some(token) = self.cur_token.to_owned() {
            let name = Identifier(token.value.to_owned());

            if self.expect_peek(TokenType::Assign) == false {
                return None;
            }

            self.next_token();
            let value = if let Some(value) = self.parse_expression(Precedences::Lowest) {
                value
            } else {
                return None;
            };

            while self.peek_token_is(TokenType::Semicolon) {
                self.next_token();
            }

            return Some(Statement::Let(name, value));
        }
        None
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let return_value = if let Some(value) = self.parse_expression(Precedences::Lowest) {
            value
        } else {
            return None;
        };

        while self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(Statement::Return(return_value));
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = if let Some(expression) = self.parse_expression(Precedences::Lowest) {
            expression
        } else {
            return None;
        };

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(Statement::Expression(expression));
    }

    pub fn parse_identifier(&self) -> Option<Expression> {
        if let Some(token) = &self.cur_token {
            return Some(Expression::Identifier(Identifier(token.value.to_owned())));
        }
        None
    }

    pub fn parse_integer_literal(&mut self) -> Option<Expression> {
        if let Some(token) = &self.cur_token {
            if let Ok(value) = token.value.parse::<i64>() {
                return Some(Expression::IntegerLiteral(value));
            } else {
                self.errors
                    .push(format!("could not parse {:?} as integer", token.value));
            }
        }
        None
    }

    pub fn parse_string_literal(&mut self) -> Option<Expression> {
        if let Some(token) = &self.cur_token {
            return Some(Expression::StringLiteral(token.value.to_owned()));
        }
        None
    }

    pub fn parse_boolean(&mut self) -> Option<Expression> {
        return Some(Expression::Boolean(self.cur_token_is(TokenType::True)));
    }

    pub fn parse_expression(&mut self, precedence: Precedences) -> Option<Expression> {
        let mut left_exp: Option<Expression> = None;
        if let Some(token) = self.cur_token.to_owned() {
            left_exp = match token.kind {
                TokenType::Identifier => self.parse_identifier(),
                TokenType::Digit => self.parse_integer_literal(),
                TokenType::String => self.parse_string_literal(),
                TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
                TokenType::Lparen => self.parse_grouped_expression(),
                TokenType::True | TokenType::False => self.parse_boolean(),
                TokenType::If => self.parse_if_expression(),
                TokenType::Fn => self.parse_function_literal(),
                _ => {
                    self.no_prefix_parse_fn_error(token.kind);
                    return None;
                }
            };
        }

        while self.peek_token_is(TokenType::Semicolon) == false
            && precedence < self.peek_precedence()
        {
            if let Some(token) = self.peek_token.to_owned() {
                left_exp = match token.kind {
                    TokenType::Plus
                    | TokenType::Minus
                    | TokenType::Divide
                    | TokenType::Multiply
                    | TokenType::Eq
                    | TokenType::NotEq
                    | TokenType::Lt
                    | TokenType::Lte
                    | TokenType::Gt
                    | TokenType::Gte => {
                        self.next_token();
                        self.parse_infix_expression(left_exp)
                    }
                    TokenType::Lparen => {
                        self.next_token();
                        self.parse_call_expression(left_exp)
                    }
                    _ => {
                        self.no_prefix_parse_fn_error(token.kind);
                        return left_exp;
                    }
                };
            }
        }

        left_exp
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Expression> {
        if let Some(token) = self.cur_token.to_owned() {
            self.next_token();
            if let Some(right) = self.parse_expression(Precedences::Prefix) {
                if let Some(prefix) = self.convert_token_to_prefix(token.kind) {
                    return Some(Expression::Prefix(prefix, Box::new(right)));
                }
            }
        }
        None
    }

    pub fn convert_token_to_prefix(&mut self, token: TokenType) -> Option<Prefix> {
        match token {
            TokenType::Plus => Some(Prefix::Plus),
            TokenType::Minus => Some(Prefix::Minus),
            TokenType::Bang => Some(Prefix::Bang),
            _ => {
                self.errors
                    .push(format!("{:?} is not a token for prefix", token));
                None
            }
        }
    }

    pub fn convert_token_to_infix(&mut self, token: TokenType) -> Option<Infix> {
        match token {
            TokenType::Plus => Some(Infix::Plus),
            TokenType::Minus => Some(Infix::Minus),
            TokenType::Divide => Some(Infix::Divide),
            TokenType::Multiply => Some(Infix::Multiply),
            TokenType::Eq => Some(Infix::Eq),
            TokenType::NotEq => Some(Infix::NotEq),
            TokenType::Gte => Some(Infix::Gte),
            TokenType::Gt => Some(Infix::Gt),
            TokenType::Lte => Some(Infix::Lte),
            TokenType::Lt => Some(Infix::Lt),
            _ => {
                self.errors
                    .push(format!("{:?} is not a token for infix", token));
                None
            }
        }
    }

    pub fn parse_infix_expression(&mut self, left: Option<Expression>) -> Option<Expression> {
        if left.is_none() {
            return None;
        }

        if let Some(token) = self.cur_token.to_owned() {
            let precedence = self.cur_precedence();
            self.next_token();
            if let Some(right) = self.parse_expression(precedence) {
                if let Some(infix) = self.convert_token_to_infix(token.kind) {
                    return Some(Expression::Infix(
                        infix,
                        Box::new(left.unwrap()),
                        Box::new(right),
                    ));
                }
            }
        }
        None
    }

    pub fn parse_if_expression(&mut self) -> Option<Expression> {
        if self.expect_peek(TokenType::Lparen) == false {
            return None;
        }
        self.next_token();

        if let Some(condition) = self.parse_expression(Precedences::Lowest) {
            if self.expect_peek(TokenType::Rparen) == false {
                return None;
            }

            if self.expect_peek(TokenType::Lbrace) == false {
                return None;
            }

            if let Some(consequence) = self.parse_block_statement() {
                let alternative = if self.peek_token_is(TokenType::Else) {
                    self.next_token();
                    if self.expect_peek(TokenType::Lbrace) == false {
                        return None;
                    }
                    self.parse_block_statement()
                } else {
                    None
                };

                return Some(Expression::If {
                    condition: Box::new(condition),
                    consequence: consequence,
                    alternative: alternative,
                });
            }
        }
        None
    }

    pub fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = if let Some(ret) = self.parse_expression(Precedences::Lowest) {
            ret
        } else {
            return None;
        };

        if self.expect_peek(TokenType::Rparen) == false {
            return None;
        }
        Some(exp)
    }

    pub fn parse_function_literal(&mut self) -> Option<Expression> {
        if self.expect_peek(TokenType::Lparen) == false {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if self.expect_peek(TokenType::Lbrace) == false {
            return None;
        }

        if let Some(body) = self.parse_block_statement() {
            return Some(Expression::Function {
                parameters: parameters,
                body: body,
            });
        }
        None
    }

    pub fn parse_call_expression(&mut self, function: Option<Expression>) -> Option<Expression> {
        if let Some(function) = function {
            let expr = Expression::Call(Call {
                function: Box::new(function),
                arguments: self.parse_call_arguments(),
            });

            match expr.clone() {
                Expression::Function {
                    parameters: _,
                    body: _,
                } => {
                    if let Some(token) = self.peek_token.to_owned() {
                        return match token.kind {
                            TokenType::Lparen => {
                                self.next_token();
                                self.parse_call_expression(Some(expr))
                            }
                            _ => Some(expr),
                        };
                    }
                    None
                }
                _ => Some(expr),
            }
        } else {
            None
        }
    }

    pub fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();
        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return args;
        }
        self.next_token();

        if let Some(arg) = self.parse_expression(Precedences::Lowest) {
            args.push(arg);
        }

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            if let Some(arg) = self.parse_expression(Precedences::Lowest) {
                args.push(arg);
            }
        }

        if self.expect_peek(TokenType::Rparen) == false {
            return args;
        }
        args
    }

    pub fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut parameters = Vec::new();

        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return parameters;
        }
        self.next_token();

        if let Some(token) = self.cur_token.to_owned() {
            parameters.push(Identifier(token.value.to_owned()));
        }

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();

            if let Some(token) = self.cur_token.to_owned() {
                parameters.push(Identifier(token.value.to_owned()));
            }
        }

        if self.expect_peek(TokenType::Rparen) == false {
            return Vec::new();
        }

        parameters
    }

    pub fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let mut block = Vec::new();
        self.next_token();

        while self.cur_token_is(TokenType::Rbrace) == false && self.cur_token.is_none() == false {
            if let Some(stmt) = self.parse_statement() {
                block.push(stmt);
            }
            self.next_token();
        }
        return Some(block);
    }

    pub fn cur_token_is(&self, t: TokenType) -> bool {
        if let Some(token) = &self.cur_token {
            return token.kind == t;
        }
        false
    }

    pub fn peek_token_is(&self, t: TokenType) -> bool {
        if let Some(token) = &self.peek_token {
            return token.kind == t;
        }
        false
    }

    pub fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t);
            return false;
        }
    }
    pub fn peek_precedence(&mut self) -> Precedences {
        if let Some(token) = &self.peek_token {
            let token_type = token.kind;
            if PrecedenceTokenMap.contains_key(&token_type) {
                return PrecedenceTokenMap[&token_type].to_owned();
            }
        }
        Precedences::Lowest
    }

    pub fn cur_precedence(&self) -> Precedences {
        if let Some(token) = &self.cur_token {
            let token_type = token.kind;
            if PrecedenceTokenMap.contains_key(&token_type) {
                return PrecedenceTokenMap[&token_type].to_owned();
            }
        }
        Precedences::Lowest
    }

    pub fn has_error(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn emit_error(&self) -> String {
        self.errors.join("\n")
    }

    pub fn peek_error(&mut self, t: TokenType) {
        self.errors
            .push(format!("expected next token to be {:?} instead", t));
    }

    pub fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        self.errors
            .push(format!("no prefix parse function for {:?}", t));
    }
}

/* below the test implementation */
#[warn(dead_code)]
fn statement_assert(statement: &Statement, expect: &str) {
    assert!(statement.string() == expect, statement.emit_debug_info());
}

#[warn(dead_code)]
fn parse_input(input: &str) -> Program {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    parser.parse_program()
}

#[warn(dead_code)]
fn parse_and_emit_error(input: &str, error_stack: Vec<&str>) {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    if parser.has_error() == false {
      panic!("no errors found. return program is {:?}", program);
    }

    assert!(
      parser.emit_error() == error_stack.join("\n"),
      "\r\nexpected: {:?} \r\nactual: {:?}",
      parser.emit_error(),
      error_stack.join("\n")
    );
    
}

#[test]
fn test_let_statements() {
    let input = r#"
    let x = 5;
    let y = 10;
    let z = "abc";
    let foobar = 939393;
  "#;
    let program = parse_input(input);
    statement_assert(&program[0], "let x = 5");
    statement_assert(&program[1], "let y = 10");
    statement_assert(&program[2], r#"let z = "abc""#);
    statement_assert(&program[3], "let foobar = 939393");
}

#[test]
fn test_return_statements() {
    let input = r#"
    return 5;
    return 10;
    return 939393;
  "#;
    let program = parse_input(input);
    statement_assert(&program[0], "return 5");
    statement_assert(&program[1], "return 10");
    statement_assert(&program[2], "return 939393");
}

#[test]
fn test_operator_precedence_parsing() {
    let input = r#"
  -a * b;
  !-a;
  a + b + c;
  a + b - c;
  a * b * c;
  a * b / c;
  a + b / c;
  a + b * c + d / e - f;
  3 + 4 - 5 * 5;
  5 > 4 == 3 < 4;
  5 < 4 != 3 > 4;
  3 + 4 * 5 == 3 * 1 + 4 * 5;
"#;
    let program = parse_input(input);
    statement_assert(&program[0], "((-a) * b)");
    statement_assert(&program[1], "(!(-a))");
    statement_assert(&program[2], "((a + b) + c)");
    statement_assert(&program[3], "((a + b) - c)");
    statement_assert(&program[4], "((a * b) * c)");
    statement_assert(&program[5], "((a * b) / c)");
    statement_assert(&program[6], "(a + (b / c))");
    statement_assert(&program[7], "(((a + (b * c)) + (d / e)) - f)");
    statement_assert(&program[8], "((3 + 4) - (5 * 5))");
    statement_assert(&program[9], "((5 > 4) == (3 < 4))");
    statement_assert(&program[10], "((5 < 4) != (3 > 4))");
    statement_assert(&program[11], "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))");
}

#[test]
fn test_if_else_parsing() {
    let input = r#"
  if(a > b) {};
  if(a > b) { return 1; };
  if(a > b) { return 1; } else { return 0; };
"#;
    let program = parse_input(input);
    statement_assert(&program[0], "if(a > b) {  }");
    statement_assert(&program[1], "if(a > b) { return 1 }");
    statement_assert(&program[2], "if(a > b) { return 1 } else { return 0 }");
}

#[test]
fn test_boolean_parsing() {
    let input = r#"
  true;
  false;
  3 > 5 == false;
  3 < 5 == true;
"#;
    let program = parse_input(input);
    statement_assert(&program[0], "true");
    statement_assert(&program[1], "false");
    statement_assert(&program[2], "((3 > 5) == false)");
    statement_assert(&program[3], "((3 < 5) == true)");
}

#[test]
fn test_funciton_parsing() {
    let input = r#"
  fn() { };
  fn(x) {};
  fn(x, y, z) {};
"#;
    let program = parse_input(input);
    statement_assert(&program[0], "fn() {  }");
    statement_assert(&program[1], "fn(x) {  }");
    statement_assert(&program[2], "fn(x, y, z) {  }");
}

#[test]
fn test_call_parsing() {
    let input = r#"
  a + add(b * c) + d;
  add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8));
  add(a + b + c * d / f + g);
"#;
    let program = parse_input(input);
    statement_assert(&program[0], "((a + add((b * c))) + d)");
    statement_assert(
        &program[1],
        "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
    );
    statement_assert(&program[2], "add((((a + b) + ((c * d) / f)) + g))");
}

#[test]
fn test_wrong_prefix() {
    let input = r#"
    return > 3;
  "#;
    parse_and_emit_error(input, vec!["no prefix parse function for Gt"]);
}