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
    line: usize,
    column: usize,
}

impl ParserError {
    fn new(msg: impl Into<String>, line: usize, column: usize) -> ParserError {
        ParserError {
            msg: msg.into(),
            line,
            column,
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    current_token_position: (usize, usize),
    peek_token: Token,
    peek_token_position: (usize, usize),
    errors: Vec<ParserError>,
}

impl Parser {
    fn assert_peek(&mut self, expected: Token) -> Result<(), ParserError> {
        if self.peek_token == expected {
            self.next_token();
            return Ok(());
        }
        Err(ParserError::new(
            format!("expected {:?}, got {:?} instead", expected, self.peek_token,),
            self.peek_token_position.0,
            self.peek_token_position.1,
        ))
    }
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            current_token: current_token.0,
            current_token_position: current_token.1,
            peek_token: peek_token.0,
            peek_token_position: current_token.1,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        std::mem::swap(
            &mut self.current_token_position,
            &mut self.peek_token_position,
        );
        let peek_token = self.lexer.next_token();
        self.peek_token = peek_token.0;
        self.peek_token_position = peek_token.1;
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from(&self.peek_token)
    }

    fn current_precedence(&self) -> Precedence {
        Precedence::from(&self.current_token)
    }

    pub fn check_errors(&self) {
        if !self.errors.is_empty() {
            println!("parser has {} errors", self.errors.len());

            self.errors
                .iter()
                .for_each(|err| println!("parser error: {} {}:{}", err.msg, err.line, err.column))
        }
    }

    pub fn parse_program(&mut self) -> ast::program::Program {
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
            token => Err(ParserError::new(
                format!("expected Token::Ident, got {:?} instead", token),
                self.peek_token_position.0,
                self.peek_token_position.1,
            )),
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
            Token::String(string) => Ok(Expression::String(string.to_string())),
            Token::Ident(value) => Ok(Expression::Identifier(value.to_owned())),
            Token::Int(value) => Ok(Expression::Int(value.to_owned())),
            Token::False => Ok(Expression::Bool(false)),
            Token::True => Ok(Expression::Bool(true)),
            Token::Minus | Token::Bang => self.parse_prefix_expression(),
            Token::Lbracket => self.parse_array_literal(),
            Token::Lbrace => self.parse_hash_literal(),
            Token::Lparen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_function_literal(),
            token => Err(ParserError::new(
                format!("i dont now what is this: {:?}", token),
                self.current_token_position.0,
                self.current_token_position.1,
            )),
        }
    }

    fn parse_hash_literal(&mut self) -> Result<Expression, ParserError> {
        let mut hash: Vec<(Expression, Expression)> = vec![];
        while self.peek_token != Token::Rbrace {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;

            self.assert_peek(Token::Colon)?;

            self.next_token();

            let value = self.parse_expression(Precedence::Lowest)?;

            hash.push((key, value));

            if self.peek_token != Token::Rbrace {
                self.assert_peek(Token::Comma)?;
            }
        }
        self.assert_peek(Token::Rbrace)?;
        Ok(Expression::Hash(hash))
    }

    fn parse_array_literal(&mut self) -> Result<Expression, ParserError> {
        let elements = self.parse_expression_list(Token::Rbracket)?;
        Ok(Expression::Array(elements))
    }

    fn parse_expression_list(&mut self, end: Token) -> Result<Vec<Expression>, ParserError> {
        let mut list: Vec<Expression> = vec![];

        self.next_token();
        if self.current_token == end {
            return Ok(list);
        }

        list.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?)
        }

        self.assert_peek(end)?;

        Ok(list)
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token != Token::Rparen {
            return Err(ParserError::new(
                format!("expected Token::Rparen, got: {:?}", self.peek_token),
                self.peek_token_position.0,
                self.peek_token_position.1,
            ));
        }
        self.next_token();
        Ok(expression)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = match &self.current_token {
            Token::Bang => PrefixOperator::Not,
            Token::Minus => PrefixOperator::Negative,
            value => {
                return Err(ParserError::new(
                    format!("this is not a valid PrefixOperator: {:?}", value),
                    self.current_token_position.0,
                    self.current_token_position.1,
                ))
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
        if self.current_token == Token::Rparen {
            return Ok(parameters);
        }

        match &self.current_token {
            Token::Ident(param) => {
                parameters.push(param.to_string());
            }
            value => {
                return Err(ParserError::new(
                    format!("expected Token::Ident, got: {:?}", value),
                    self.current_token_position.0,
                    self.current_token_position.1,
                ))
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
                    return Err(ParserError::new(
                        format!("expected Token::Ident, got: {:?}", value),
                        self.current_token_position.0,
                        self.current_token_position.1,
                    ))
                }
            }
        }
        self.assert_peek(Token::Rparen)?;

        Ok(parameters)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParserError> {
        let arguments = self.parse_expression_list(Token::Rparen)?;
        Ok(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }

    // fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
    //     let mut args = Vec::new();
    //     self.next_token();
    //     if self.current_token == Token::Rparen {
    //         return Ok(args);
    //     }
    //     args.push(self.parse_expression(Precedence::Lowest)?);
    //
    //     while self.peek_token == Token::Comma {
    //         self.next_token();
    //         self.next_token();
    //         args.push(self.parse_expression(Precedence::Lowest)?)
    //     }
    //     self.assert_peek(Token::Rparen)?;
    //     Ok(args)
    // }

    fn parse_index_expression(&mut self, lhs: Expression) -> Result<Expression, ParserError> {
        self.next_token();
        let idx = self.parse_expression(Precedence::Lowest)?;

        self.assert_peek(Token::Rbracket)?;

        Ok(Expression::Index {
            lhs: Box::new(lhs),
            index: Box::new(idx),
        })
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
            Token::Lbracket => return self.parse_index_expression(lhs),
            Token::Lparen => return self.parse_call_expression(lhs),
            value => {
                return Err(ParserError::new(
                    format!("This is not a valid InfixOperator: {:?}", value),
                    self.current_token_position.0,
                    self.current_token_position.1,
                ))
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
