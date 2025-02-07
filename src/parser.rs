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
pub enum Expression {
    Int(i32),
    UnaryOperation(UnaryOperator, Box<Expression>),
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
    let token = tokens.pop_front().expect("Expected expression");
    match token {
        Token::Constant(s) => {
            Expression::Int(s.parse().expect("Expected integer"))
        }
        Token::Minus | Token::LogicNot | Token::BitwiseNot => {
            let expr = parse_expression(tokens);
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
        _ => panic!("Unexpected token {token:?}. Expression expected."),
    }
}
