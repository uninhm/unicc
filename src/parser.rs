use crate::lexer::Token;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<FunctionDeclaration>,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub return_type: String,
    pub name: String,
    // pub parameters: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negation,
    BitwiseNot,
    LogicNot,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus, Minus,
    Times, Divide,
}

#[derive(Debug)]
pub enum Expression {
    Int(i32),
    UnaryOperation(UnaryOperator, Box<Expression>),
    BinaryOperation(Box<Expression>, BinaryOperator, Box<Expression>),
}

pub fn parse(tokens: Vec<Token>) -> Program {
    Program {
        declarations: vec![parse_function_declaration(tokens)],
    }
}

fn expect_token(tokens: &mut VecDeque<Token>, expected: Token) {
    assert_eq!(
        tokens.pop_front().expect("Expected token {expected:?}"),
        expected
    );
}

fn parse_function_declaration(tks: Vec<Token>) -> FunctionDeclaration {
    let mut tokens = VecDeque::from(tks);
    let return_type_tok = tokens.pop_front().expect("Expected function return type");
    let return_type: String;
    match return_type_tok {
        Token::Keyword(s) => {
            assert_eq!(s, "int");
            return_type = s;
        },
        _ => panic!("Unexpected return type {return_type_tok:?}"),
    }
    let name_tok = tokens.pop_front().expect("Expected function name");
    let name: String;
    match name_tok {
        Token::Identifier(s) => {
            name = s;
        },
        _ => panic!("Unexpected token {name_tok:?}. Function name expected"),
    }
    expect_token(&mut tokens, Token::LeftParen);
    expect_token(&mut tokens, Token::RightParen);
    expect_token(&mut tokens, Token::LeftBrace);
    let body = parse_statements(&mut tokens);
    expect_token(&mut tokens, Token::RightBrace);
    FunctionDeclaration {
        return_type,
        name,
        body,
    }
}

fn parse_statements(tokens: &mut VecDeque<Token>) -> Vec<Statement> {
    let mut statements = Vec::new();
    while let Some(token) = tokens.front() {
        // TODO: Handle nested blocks
        if *token == Token::RightBrace {
            break;
        }
        statements.push(parse_statement(tokens));
    }
    statements
}

fn parse_statement(tokens: &mut VecDeque<Token>) -> Statement {
    let token = tokens.pop_front().expect("Expected statement");
    match token {
        Token::Keyword (s) => {
            assert_eq!(s, "return");
            let expr = parse_expression(tokens);
            expect_token(tokens, Token::Semicolon);
            Statement::Return(expr)
        }
        _ => panic!("Unexpected token {token:?}"),
    }
}

fn parse_expression(tokens: &mut VecDeque<Token>) -> Expression {
    let mut left = parse_term(tokens);
    while matches!(tokens.front(), Some(Token::Plus | Token::Minus)) {
        let token = tokens.pop_front().unwrap();
        let operator = match token {
            Token::Plus => BinaryOperator::Plus,
            Token::Minus => BinaryOperator::Minus,
            _ => unreachable!(),
        };
        let right = parse_term(tokens);
        left = Expression::BinaryOperation(
            Box::new(left),
            operator,
            Box::new(right),
        );
    }
    left
}

fn parse_term(tokens: &mut VecDeque<Token>) -> Expression {
    let mut left = parse_factor(tokens);
    while matches!(tokens.front(), Some(Token::Times | Token::Divide)) {
        let token = tokens.pop_front().unwrap();
        let operator = match token {
            Token::Times => BinaryOperator::Times,
            Token::Divide => BinaryOperator::Divide,
            _ => unreachable!(),
        };
        let right = parse_factor(tokens);
        left = Expression::BinaryOperation(
            Box::new(left),
            operator,
            Box::new(right),
        )
    }
    left
}

fn parse_factor(tokens: &mut VecDeque<Token>) -> Expression {
    let token = tokens.pop_front().expect("Expected a factor");
    match token {
        Token::Constant(s) => {
            Expression::Int(s.parse().expect("Expected integer"))
        }
        Token::LeftParen => {
            let expr = parse_expression(tokens);
            expect_token(tokens, Token::RightParen);
            expr
        }
        Token::Minus | Token::LogicNot | Token::BitwiseNot => {
            let expr = parse_factor(tokens);
            let operator = match token {
                Token::Minus => UnaryOperator::Negation,
                Token::LogicNot => UnaryOperator::LogicNot,
                Token::BitwiseNot => UnaryOperator::BitwiseNot,
                _ => unreachable!(),
            };
            Expression::UnaryOperation(
                operator,
                Box::new(expr)
            )
        }
        _ => panic!("Unexpected token {token:?}. Factor expected."),
    }
}
