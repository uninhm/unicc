use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Constant(String),
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Semicolon,
    Plus, Minus,
    Times, Divide,
    BitwiseNot,
    LogicNot,
}

const KEYWORDS: &[&str] = &[
    "int", "return",
];

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
            '(' | ')' | '{' | '}' | ';' | '-' | '~' | '!' | '+' | '*' | '/' => {
                tokens.push(match c {
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    '{' => Token::LeftBrace,
                    '}' => Token::RightBrace,
                    ';' => Token::Semicolon,
                    '+' => Token::Plus,
                    '*' => Token::Times,
                    '/' => Token::Divide,
                    '-' => Token::Minus,
                    '~' => Token::BitwiseNot,
                    '!' => Token::LogicNot,
                    _ => unreachable!(),
                });
                chars.next();
            }
            'a'..='z' | 'A'..='Z' => {
                let word = get_word(&mut chars);
                if KEYWORDS.contains(&word.as_str()) {
                    tokens.push(Token::Keyword(word));
                } else {
                    tokens.push(Token::Identifier(word));
                }
            }
            ' ' | '\t' | '\n' => {
                chars.next();
            },
            _ => panic!("Unexpected character: '{}'", c),
        }
    }
    tokens
}
