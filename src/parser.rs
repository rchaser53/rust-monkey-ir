mod lexer;
use lexer::*;

struct Node {}
impl Node {
  token_literal(&mut self) -> String {
    "".to_string()
  }
}

struct Statement {
  node: Node
}
impl Statement {
  statement_node(&mut self) -> Node {
    self.node
  }
}

struct Expression {
  node: Node
}
impl Expression {
  expression_node(&mut self) -> Node {
    self.node
  }
}

struct Program {
  statements: Vec<Statement>
}
impl Program {
  pub fn token_literal(&mut self) -> String {
     if self.statements.len() > 0 {
      self.statements[0].token_literal()
    } else {
      "".to_string()
    }
  }
}

struct LetStatement {
  token: TokenType,
  value: Expression,
  name: Identifier,
}
impl LetStatement {
  pub fn statement_node() {}

  pub fn token_literal(&mut self) -> String {
    // return self.token.literal
    self.token.value
  }
}

struct Identifier {
  token: TokenType,
  value: String,
}

impl Identifier {
  pub fn expression_node() {}
  pub fn token_literal(&mut self) -> String {
    // return self.token.literal
    self.token.value
  }
}

struct Parser {
  l: Lexer,
  cur_token: Token,
  peek_token: Token,
}

impl Parser {
  pub fn New(l: lexer) -> Parser {
    let peek_token = lexer.next_token();
    let current_token = lexer.next_token();

    Parser{
      l: l,
      cur_token: current_token,
      peek_token: peek_token,
    }
  }

  pub next_token(&mut self) {
    self.cur_token = self.peek_token;
    self.peek_token = self.l.next_token();
  }

  pub fn parse_program(&mut self) -> Program {
    let mut program = Program{
      statements: Vec::new()
    };
    // while self.cur_token.type != token.EOF {
    while self.cur_token != None {
      if let Some(stmt) = self.parse_statement() {
        program.statements.push(stmt);
      }
      p.next_token();
    }
    program
  }

  pub fn parse_statement() -> Option<Statement> {
    match p.cur_token.type {
      token.LET => {
        Some(p.parse_let_statement())
      },
      _ => {
        None
      }
    }
  }

  pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
    let mut stmt := LetStatement{
      token: self.cur_token.clone()
    };

    if self.expect_peek(TokenType::Identifier) == false {
      return None;
    }
    stmt.name = Identifier{
      token: self.cur_token.clone(),
      value: p.cur_token.value,
    };
    
    // TODO: セミコロンに遭遇するまで式を読み飛ばしてしまっている for !p.curTokenIs(token.SEMICOLON) {
    if self.expect_peek(TokenType::ASSIGN) == false {
      return self.next_token();
    }
    stmt
  }

  pub fn cur_token_is(&mut self, t: TokenType) -> bool {
    self.cur_token.type == t
  }

  pub fn peek_token_is(&mut self, t: TokenType) -> bool {
    self.peek_token.type == t
  }

  pub fn expect_peek(&mut self, t: TokenType) -> bool {
    if self.peek_token_is(t) {
      self.next_token();
      return true;
    } else {
      return false;
    }
  }
}
