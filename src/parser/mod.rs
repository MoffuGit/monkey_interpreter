mod precedence;
#[cfg(test)]
mod tests;

use crate::{
    ast::{
        self,
        expression::Expression,
        operator::{InfixOperator, PrefixOperator},
        program::Program,
        statement::Statement,
    },
    lexer::{token::Token, Lexer},
    parser::precedence::Precedence,
};

struct ParserError {
    msg: String,
}

impl ParserError {
    fn new(msg: impl Into<String>) -> ParserError {
        ParserError { msg: msg.into() }
    }
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

impl Parser {
    fn assert_peek(&mut self, expected: Token) -> Result<(), ParserError> {
        if self.peek_token == expected {
            self.next_token();
            return Ok(());
        }
        Err(ParserError::new(format!(
            "expected {:?}, got {:?} instead",
            expected, self.peek_token
        )))
    }
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from(&self.peek_token)
    }

    fn current_precedence(&self) -> Precedence {
        Precedence::from(&self.current_token)
    }

    fn check_errors(&self) {
        if !self.errors.is_empty() {
            println!("parser has {} errors", self.errors.len());

            self.errors
                .iter()
                .for_each(|err| println!("parser error: {}", err.msg))
        }
    }

    fn parse_program(&mut self) -> ast::program::Program {
        let mut program = Program::default();
        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(err) => self.errors.push(err),
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let name = match &self.peek_token {
            Token::Ident(name) => Ok(name.to_owned()),
            token => Err(ParserError::new(format!(
                "expected Token::Ident, got {:?} instead",
                token
            ))),
        }?;

        self.next_token();

        self.assert_peek(Token::Assign)?;

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token == Token::Semicolon {
            self.next_token()
        }

        Ok(Statement::Let { name, value })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Statement::Return(expression))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Statement::Expression(expression))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let mut lhs = self.parse_prefix()?;

        while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
            self.next_token();
            lhs = self.parse_infix_expression(lhs)?;
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expression, ParserError> {
        match &self.current_token {
            Token::Ident(value) => Ok(Expression::Identifier(value.to_owned())),
            Token::Int(value) => Ok(Expression::Int(value.to_owned())),
            Token::False => Ok(Expression::Bool(false)),
            Token::True => Ok(Expression::Bool(true)),
            Token::Minus | Token::Bang => self.parse_prefix_expression(),
            Token::Lparen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_function_literal(),
            token => Err(ParserError::new(format!(
                "i dont now what is this: {:?}",
                token
            ))),
        }
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token != Token::Rparen {
            return Err(ParserError::new(format!(
                "expected Token::Rparen, got: {:?}",
                self.peek_token
            )));
        }
        self.next_token();
        Ok(expression)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = match &self.current_token {
            Token::Bang => PrefixOperator::Not,
            Token::Minus => PrefixOperator::Negative,
            value => {
                return Err(ParserError::new(format!(
                    "this is not a valid PrefixOperator: {:?}",
                    value
                )))
            }
        };
        self.next_token();
        let rhs = self.parse_expression(Precedence::Prefix)?;

        Ok(Expression::Prefix {
            rhs: Box::new(rhs),
            operator,
        })
    }

    fn parse_block_statement(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();

        self.next_token();

        while self.current_token != Token::Rbrace && self.current_token != Token::Eof {
            statements.push(self.parse_statement()?);

            self.next_token()
        }

        Ok(statements)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        self.assert_peek(Token::Lparen)?;
        self.next_token();

        let condition = self.parse_expression(Precedence::Lowest)?;

        self.assert_peek(Token::Rparen)?;
        self.assert_peek(Token::Lbrace)?;
        let consequence = self.parse_block_statement()?;

        if self.peek_token == Token::Else {
            self.next_token();

            self.assert_peek(Token::Lbrace)?;
            let alternative = self.parse_block_statement()?;
            return Ok(Expression::If {
                condition: Box::new(condition),
                consequence,
                alternative: Some(alternative),
            });
        }
        Ok(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative: None,
        })
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParserError> {
        self.assert_peek(Token::Lparen)?;

        let parameters = self.parse_function_parameters()?;

        self.assert_peek(Token::Lbrace)?;
        let body = self.parse_block_statement()?;

        Ok(Expression::Fn { parameters, body })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<String>, ParserError> {
        let mut parameters = Vec::new();

        self.next_token();
        if self.peek_token == Token::Rparen {
            return Ok(parameters);
        }

        match &self.current_token {
            Token::Ident(param) => {
                parameters.push(param.to_string());
            }
            value => {
                return Err(ParserError::new(format!(
                    "expected Token::Ident, got: {:?}",
                    value
                )))
            }
        }
        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            match &self.current_token {
                Token::Ident(param) => {
                    parameters.push(param.to_string());
                }
                value => {
                    return Err(ParserError::new(format!(
                        "expected Token::Ident, got: {:?}",
                        value
                    )))
                }
            }
        }
        self.assert_peek(Token::Rparen)?;

        Ok(parameters)
    }

    fn parse_infix_expression(&mut self, lhs: Expression) -> Result<Expression, ParserError> {
        let operator = match &self.current_token {
            Token::Plus => InfixOperator::Add,
            Token::Minus => InfixOperator::Sub,
            Token::Asterisk => InfixOperator::Mul,
            Token::Slash => InfixOperator::Div,
            Token::Eq => InfixOperator::Equal,
            Token::NotEq => InfixOperator::NotEqual,
            Token::Gt => InfixOperator::GreaterThan,
            Token::GtorEq => InfixOperator::GreaterThanOrEqual,
            Token::Lt => InfixOperator::LessThan,
            Token::LtorEq => InfixOperator::LessThanOrEqual,
            Token::Percent => InfixOperator::Modulo,
            value => {
                return Err(ParserError::new(format!(
                    "This is not a valid InfixOperator: {:?}",
                    value
                )))
            }
        };
        let precedence = self.current_precedence();
        self.next_token();
        let rhs = self.parse_expression(precedence)?;

        Ok(Expression::Infix {
            lhs: Box::new(lhs),
            operator,
            rhs: Box::new(rhs),
        })
    }
}
