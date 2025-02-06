mod lexer;
mod parser;

fn main() {
    let tokens = lexer::lex("int main () { return 0; }");
    let ast = parser::parse(tokens.clone());
    println!("{:?}", ast);
}
