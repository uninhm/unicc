mod lexer;
mod parser;
mod codegen;

fn main() {
    let tokens = lexer::lex("int main () { return 0; }");
    let ast = parser::parse(tokens);
    let code = codegen::generate(ast);
    println!("{}", code);
}
