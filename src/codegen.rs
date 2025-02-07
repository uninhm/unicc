use crate::parser::*;

pub fn generate(program: Program) -> String {
    let mut res = String::new();
    for func_decl in program.declarations {
        res.push_str(&generate_func_decl(func_decl));
    }
    res
}

fn generate_func_decl(func_decl: FunctionDeclaration) -> String {
    let mut res = String::new();
    res.push_str(&format!(".globl {}\n", func_decl.name));
    res.push_str(&format!("{}:\n", func_decl.name));
    for stmt in func_decl.body {
        res.push_str(&generate_stmt(stmt));
    }
    res
}

fn generate_stmt(stmt: Statement) -> String {
    let mut res = String::new();
    match stmt {
        Statement::Return(expr) => {
            let Expression::Int(x) = expr;
            res.push_str(&format!("mov ${}, %rax\nret\n", x));
        }
        _ => panic!("Statement {:?} code generation not implemented", stmt)
    }
    res
}
