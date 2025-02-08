mod lexer;
mod parser;
mod codegen;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: unicc <filename>");
        return;
    }

    let filepath = Path::new(&args[1]);
    let contents = fs::read_to_string(filepath).expect("File not found");

    let tokens = lexer::lex(&contents);
    if args.len() >= 3 && args[2] == "-lex" {
        println!("{:?}", tokens);
        return;
    }

    let ast = parser::parse(tokens);
    if args.len() >= 3 && args[2] == "-parse" {
        println!("{:?}", ast);
        return;
    }

    let code = codegen::generate(ast);
    println!("{}", code.to_string());
}
