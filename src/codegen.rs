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
            res.push_str(&generate_expr(expr));
            res.push_str("ret\n");
        }
        _ => panic!("Statement {:?} code generation not implemented", stmt)
    }
    res
}

fn generate_expr(expr: Expression) -> String {
    let mut res = String::new();
    match expr {
        Expression::Int(x) => {
            res.push_str(&format!("mov ${x}, %rax\n"));
        }
        Expression::UnaryOperation(op, expr) => {
            res.push_str(&generate_expr(*expr));
            match op {
                UnaryOperator::Negation => res.push_str("neg %rax\n"),
                UnaryOperator::BitwiseNot => res.push_str("not %rax\n"),
                UnaryOperator::LogicNot => {
                    res.push_str("cmp $0, %rax\n");
                    res.push_str("mov $0, %rax\n");
                    res.push_str("sete %al\n");
                }
                _ => panic!("Unary operator {:?} code generation not implemented", op)
            }
        }
    }
    res
}
