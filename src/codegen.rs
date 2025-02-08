use crate::parser::*;

pub struct Code {
    code: String,
    label_counter: usize,
}

impl Code {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            label_counter: 0,
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

    pub fn get_label(&mut self) -> String {
        let label = format!(".L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn add_label(&mut self, label: String) {
        self.add_asm(&label);
        self.add_asm_line(":");
    }
}

pub fn generate(program: Program) -> Code {
    let mut res = Code::new();
    for func_decl in program.declarations {
        generate_func_decl(&mut res, func_decl);
    }
    res
}

fn generate_func_decl(res: &mut Code, func_decl: FunctionDeclaration) {
    res.add_asm_line(&format!(".globl {}", func_decl.name));
    res.add_asm_line(&format!("{}:", func_decl.name));
    for stmt in func_decl.body {
        generate_stmt(res, stmt);
    }
}

fn generate_stmt(res: &mut Code, stmt: Statement) {
    match stmt {
        Statement::Return(expr) => {
            generate_expr(res, expr);
            res.add_asm_line("ret");
        }
    }
}

fn generate_expr(res: &mut Code, expr: Expression) {
    match expr {
        Expression::Int(x) => {
            res.add_asm_line(&format!("mov ${}, %rax", x));
        }
        Expression::UnaryOperation(op, expr) => {
            generate_expr(res, *expr);
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
        Expression::BinaryOperation(left, BinaryOperator::LogicOr, right) => {
            let clause2 = res.get_label();
            let end = res.get_label();
            generate_expr(res, *left);
            res.add_asm_line("cmp $0, %rax");
            res.add_asm_line(&format!("je {}", clause2));
            res.add_asm_line("mov $1, %rax");
            res.add_asm_line(&format!("jmp {}", end));
            res.add_label(clause2);
            generate_expr(res, *right);
            res.add_asm_line("cmp $0, %rax");
            res.add_asm_line("mov $0, %rax");
            res.add_asm_line("setne %al");
            res.add_label(end);
        }

        Expression::BinaryOperation(left, BinaryOperator::LogicAnd, right) => {
            let clause2 = res.get_label();
            let end = res.get_label();
            generate_expr(res, *left);
            res.add_asm_line("cmp $0, %rax");
            res.add_asm_line(&format!("jne {}", clause2));
            res.add_asm_line("mov $0, %rax");
            res.add_asm_line(&format!("jmp {}", end));
            res.add_label(clause2);
            generate_expr(res, *right);
            res.add_asm_line("cmp $0, %rax");
            res.add_asm_line("mov $0, %rax");
            res.add_asm_line("setne %al");
            res.add_label(end);
        }

        Expression::BinaryOperation(left, op, right) => {
            generate_expr(res, *right);
            res.add_asm_line("push %rax");
            generate_expr(res, *left);
            res.add_asm_line("pop %rcx");
            match op {
                BinaryOperator::Plus => res.add_asm_line("add %rcx, %rax"),
                BinaryOperator::Minus => res.add_asm_line("sub %rcx, %rax"),
                BinaryOperator::Times => res.add_asm_line("imul %rcx, %rax"),
                BinaryOperator::Divide => {
                    res.add_asm_line("cqo");
                    res.add_asm_line("idiv %rcx");
                }
                BinaryOperator::EQ => {
                    res.add_asm_line("cmp %rcx, %rax");
                    res.add_asm_line("mov $0, %rax");
                    res.add_asm_line("sete %al");
                }
                BinaryOperator::NEQ => {
                    res.add_asm_line("cmp %rcx, %rax");
                    res.add_asm_line("mov $0, %rax");
                    res.add_asm_line("setne %al");
                }
                BinaryOperator::LT => {
                    res.add_asm_line("cmp %rcx, %rax");
                    res.add_asm_line("mov $0, %rax");
                    res.add_asm_line("setl %al");
                }
                BinaryOperator::GT => {
                    res.add_asm_line("cmp %rcx, %rax");
                    res.add_asm_line("mov $0, %rax");
                    res.add_asm_line("setg %al");
                }
                BinaryOperator::LE => {
                    res.add_asm_line("cmp %rcx, %rax");
                    res.add_asm_line("mov $0, %rax");
                    res.add_asm_line("setle %al");
                }
                BinaryOperator::GE => {
                    res.add_asm_line("cmp %rcx, %rax");
                    res.add_asm_line("mov $0, %rax");
                    res.add_asm_line("setge %al");
                }
                _ => todo!(),
            }
        }
    }
}
