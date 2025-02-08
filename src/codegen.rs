use crate::parser::*;

pub struct Code {
    code: String,
}

impl Code {
    pub fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    pub fn add_asm(&mut self, asm: &str) {
        self.code.push_str(asm);
    }

    pub fn add_asm_line(&mut self, asm: &str) {
        self.add_asm(asm);
        self.code.push('\n');
    }

    pub fn to_string(&self) -> String {
        self.code.clone()
    }

    pub fn append(&mut self, other: Self) {
        self.code.push_str(&other.code);
    }
}

pub fn generate(program: Program) -> Code {
    let mut res = Code::new();
    for func_decl in program.declarations {
        res.append(generate_func_decl(func_decl));
    }
    res
}

fn generate_func_decl(func_decl: FunctionDeclaration) -> Code {
    let mut res = Code::new();
    res.add_asm_line(&format!(".globl {}", func_decl.name));
    res.add_asm_line(&format!("{}:", func_decl.name));
    for stmt in func_decl.body {
        res.append(generate_stmt(stmt));
    }
    res
}

fn generate_stmt(stmt: Statement) -> Code {
    let mut res = Code::new();
    match stmt {
        Statement::Return(expr) => {
            res.append(generate_expr(expr));
            res.add_asm_line("ret");
        }
    }
    res
}

fn generate_expr(expr: Expression) -> Code {
    let mut res = Code::new();
    match expr {
        Expression::Int(x) => {
            res.add_asm_line(&format!("mov ${}, %rax", x));
        }
        Expression::UnaryOperation(op, expr) => {
            res.append(generate_expr(*expr));
            match op {
                UnaryOperator::Negation => res.add_asm_line("neg %rax"),
                UnaryOperator::BitwiseNot => res.add_asm_line("not %rax"),
                UnaryOperator::LogicNot => {
                    res.add_asm_line("cmp $0, %rax");
                    res.add_asm_line("mov $0, %rax");
                    res.add_asm_line("sete %al");
                }
            }
        }
        Expression::BinaryOperation(left, op, right) => {
            res.append(generate_expr(*right));
            res.add_asm_line("push %rax");
            res.append(generate_expr(*left));
            res.add_asm_line("pop %rcx");
            match op {
                BinaryOperator::Plus => res.add_asm_line("add %rcx, %rax"),
                BinaryOperator::Minus => res.add_asm_line("sub %rcx, %rax"),
                BinaryOperator::Times => res.add_asm_line("imul %rcx, %rax"),
                BinaryOperator::Divide => {
                    res.add_asm_line("cqo");
                    res.add_asm_line("idiv %rcx");
                }
                _ => todo!(),
            }
        }
    }
    res
}
