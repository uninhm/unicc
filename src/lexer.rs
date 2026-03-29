use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ReturnKW,
    IntKW,
    IfKW,
    ElseKW,
    Identifier(String),
    Constant(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Plus,
    Minus,
    Times,
    Divide,
    BitwiseNot,
    LogicNot,
    LogicAnd,
    LogicOr,
    EQ,
    NEQ,
    LT,
    LE,
    GT,
    GE,
    Assign,
    Colon,
    QuestionMark,
}

fn get_number(chars: &mut Peekable<Chars>) -> String {
    let mut number = String::new();
    while let Some(c) = chars.peek() {
        if !c.is_numeric() {
            break;
        }
        number.push(*c);
        chars.next();
    }
    number
}

fn get_word(chars: &mut Peekable<Chars>) -> String {
    let mut word = String::new();
    while let Some(c) = chars.peek() {
        if !c.is_alphanumeric() {
            break;
        }
        word.push(*c);
        chars.next();
    }
    word
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' => tokens.push(Token::Constant(get_number(&mut chars))),
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Times);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '~' => {
                tokens.push(Token::BitwiseNot);
                chars.next();
            }
            '!' => {
                chars.next();
                let c = chars.peek();
                match c {
                    Some('=') => {
                        tokens.push(Token::NEQ);
                        chars.next();
                    }
                    _ => tokens.push(Token::LogicNot),
                }
            }
            '<' => {
                chars.next();
                let c = chars.peek();
                match c {
                    Some('=') => {
                        tokens.push(Token::LE);
                        chars.next();
                    }
                    _ => tokens.push(Token::LT),
                }
            }
            '>' => {
                chars.next();
                let c = chars.peek();
                match c {
                    Some('=') => {
                        tokens.push(Token::GE);
                        chars.next();
                    }
                    _ => tokens.push(Token::GT),
                }
            }
            '=' => {
                chars.next();
                let c = chars.peek();
                match c {
                    Some('=') => {
                        tokens.push(Token::EQ);
                        chars.next();
                    }
                    _ => tokens.push(Token::Assign),
                }
            }
            '|' => {
                chars.next();
                let c = chars.peek();
                match c {
                    Some('|') => {
                        tokens.push(Token::LogicOr);
                        chars.next();
                    }
                    _ => unimplemented!("Bitwise or not implemented"),
                }
            }
            '&' => {
                chars.next();
                let c = chars.peek();
                match c {
                    Some('&') => {
                        tokens.push(Token::LogicAnd);
                        chars.next();
                    }
                    _ => unimplemented!("Bitwise and not implemented"),
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let word = get_word(&mut chars);
                tokens.push(match word.as_str() {
                    "if" => Token::IfKW,
                    "else" => Token::ElseKW,
                    "return" => Token::ReturnKW,
                    "int" => Token::IntKW,
                    _ => Token::Identifier(word),
                });
            }
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            _ => panic!("Unexpected character: '{c}'"),
        }
    }
    tokens
}
