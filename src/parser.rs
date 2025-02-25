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
    Declare(String, Option<Expression>),
    Expression(Expression),
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
    LogicAnd, LogicOr,
    EQ, NEQ, LT, GT, LE, GE,
    Assign,
}

#[derive(Debug)]
pub enum Expression {
    Int(i32),
    Variable(String),
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
    // println!("Function parse_statement called");
    // println!("Tokens: {:?}", tokens);
    let token = tokens.front().expect("Expected statement");
    match token {
        Token::Keyword(s) => match s.as_str() {
            "return" => {
                tokens.pop_front();
                let expr = parse_expression(tokens);
                expect_token(tokens, Token::Semicolon);
                Statement::Return(expr)
            }
            "int" => {
                tokens.pop_front();
                let Token::Identifier(name) = tokens.pop_front().expect("Expected variable name")
                else { panic!("Unexpected token, identifier expected") };
                if let Some(Token::Assign) = tokens.front() {
                    tokens.pop_front();
                    let expr = parse_expression(tokens);
                    expect_token(tokens, Token::Semicolon);
                    Statement::Declare(name, Some(expr))
                } else {
                    expect_token(tokens, Token::Semicolon);
                    Statement::Declare(name, None)
                }
            }
            _ => panic!("Keyword {s} not supported"),
        }
        _ => {
            let expr = parse_expression(tokens);
            expect_token(tokens, Token::Semicolon);
            Statement::Expression(expr)
        }
    }
}

fn token_to_binary_operator(token: Token) -> BinaryOperator {
    match token {
        Token::Plus => BinaryOperator::Plus,
        Token::Minus => BinaryOperator::Minus,
        Token::Times => BinaryOperator::Times,
        Token::Divide => BinaryOperator::Divide,
        Token::LogicAnd => BinaryOperator::LogicAnd,
        Token::LogicOr => BinaryOperator::LogicOr,
        Token::EQ => BinaryOperator::EQ,
        Token::NEQ => BinaryOperator::NEQ,
        Token::LT => BinaryOperator::LT,
        Token::GT => BinaryOperator::GT,
        Token::LE => BinaryOperator::LE,
        Token::GE => BinaryOperator::GE,
        Token::Assign => BinaryOperator::Assign,
        _ => unreachable!(),
    }
}

macro_rules! parse_binary_operator {
    ($func_name:ident, $next_parse:ident, $pattern:pat) => {
    fn $func_name(tokens: &mut VecDeque<Token>) -> Expression {
        // println!("Function {} called", stringify!($func_name));
        // println!("Tokens: {:?}", tokens);
        let mut left = $next_parse(tokens);
        while matches!(tokens.front(), Some($pattern)) {
            let token = tokens.pop_front().unwrap();
            let right = $next_parse(tokens);
            left = Expression::BinaryOperation(
                Box::new(left),
                token_to_binary_operator(token),
                Box::new(right),
            );
        }
        left
    }
    };
}

fn parse_expression(tokens: &mut VecDeque<Token>) -> Expression {
    let left = parse_logic_or_expression(tokens);
    match tokens.front() {
        Some(Token::Assign) => {
            tokens.pop_front();
            let right = parse_expression(tokens);
            Expression::BinaryOperation(
                Box::new(left),
                BinaryOperator::Assign,
                Box::new(right),
            )
        }
        _ => left,
    }
}

parse_binary_operator!(parse_logic_or_expression, parse_logic_and_expr, Token::LogicOr);
parse_binary_operator!(parse_logic_and_expr, parse_eq_expr, Token::LogicAnd);
parse_binary_operator!(parse_eq_expr, parse_rel_expr, Token::EQ | Token::NEQ);
parse_binary_operator!(parse_rel_expr, parse_add_expr, Token::LT | Token::GT | Token::LE | Token::GE);
parse_binary_operator!(parse_add_expr, parse_term, Token::Plus | Token::Minus);
parse_binary_operator!(parse_term, parse_factor, Token::Times | Token::Divide);

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
        Token::Identifier(s) => Expression::Variable(s),
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
