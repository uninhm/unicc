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

    pub fn add_label(&mut self, label: String) {
        self.add_asm(&label);
        self.add_asm_line(":");
    }
}

pub struct CodeGenerator {
    pub code: Code,
    label_count: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            code: Code::new(),
            label_count: 0,
        }
    }

    pub fn get_label(&mut self) -> String {
        let label = format!(".L{}", self.label_count);
        self.label_count += 1;
        label
    }


    pub fn generate(&mut self, program: Program) {
        for func_decl in program.declarations {
            self.generate_func_decl(func_decl);
        }
    }

    fn generate_func_decl(&mut self, func_decl: FunctionDeclaration) {
        self.code.add_asm_line(&format!(".globl {}", func_decl.name));
        self.code.add_asm_line(&format!("{}:", func_decl.name));
        for stmt in func_decl.body {
            self.generate_stmt(stmt);
        }
    }

    fn generate_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::Return(expr) => {
                self.generate_expr(expr);
                self.code.add_asm_line("ret");
            }
            Statement::Declare(name, None) => todo!(),
            Statement::Declare(name, Some(expr)) => todo!(),
        }
    }

    fn generate_expr(&mut self, expr: Expression) {
        match expr {
            Expression::Int(x) => {
                self.code.add_asm_line(&format!("mov ${}, %rax", x));
            }
            Expression::UnaryOperation(op, expr) => {
                self.generate_expr(*expr);
                match op {
                    UnaryOperator::Negation => self.code.add_asm_line("neg %rax"),
                    UnaryOperator::BitwiseNot => self.code.add_asm_line("not %rax"),
                    UnaryOperator::LogicNot => {
                        self.code.add_asm_line("cmp $0, %rax");
                        self.code.add_asm_line("mov $0, %rax");
                        self.code.add_asm_line("sete %al");
                    }
                }
            }
            Expression::BinaryOperation(left, BinaryOperator::LogicOr, right) => {
                let clause2 = self.get_label();
                let end = self.get_label();
                self.generate_expr(*left);
                self.code.add_asm_line("cmp $0, %rax");
                self.code.add_asm_line(&format!("je {}", clause2));
                self.code.add_asm_line("mov $1, %rax");
                self.code.add_asm_line(&format!("jmp {}", end));
                self.code.add_label(clause2);
                self.generate_expr(*right);
                self.code.add_asm_line("cmp $0, %rax");
                self.code.add_asm_line("mov $0, %rax");
                self.code.add_asm_line("setne %al");
                self.code.add_label(end);
            }

            Expression::BinaryOperation(left, BinaryOperator::LogicAnd, right) => {
                let clause2 = self.get_label();
                let end = self.get_label();
                self.generate_expr(*left);
                self.code.add_asm_line("cmp $0, %rax");
                self.code.add_asm_line(&format!("jne {}", clause2));
                self.code.add_asm_line("mov $0, %rax");
                self.code.add_asm_line(&format!("jmp {}", end));
                self.code.add_label(clause2);
                self.generate_expr(*right);
                self.code.add_asm_line("cmp $0, %rax");
                self.code.add_asm_line("mov $0, %rax");
                self.code.add_asm_line("setne %al");
                self.code.add_label(end);
            }

            Expression::BinaryOperation(left, op, right) => {
                self.generate_expr(*right);
                self.code.add_asm_line("push %rax");
                self.generate_expr(*left);
                self.code.add_asm_line("pop %rcx");
                match op {
                    BinaryOperator::Plus => self.code.add_asm_line("add %rcx, %rax"),
                    BinaryOperator::Minus => self.code.add_asm_line("sub %rcx, %rax"),
                    BinaryOperator::Times => self.code.add_asm_line("imul %rcx, %rax"),
                    BinaryOperator::Divide => {
                        self.code.add_asm_line("cqo");
                        self.code.add_asm_line("idiv %rcx");
                    }
                    BinaryOperator::EQ => {
                        self.code.add_asm_line("cmp %rcx, %rax");
                        self.code.add_asm_line("mov $0, %rax");
                        self.code.add_asm_line("sete %al");
                    }
                    BinaryOperator::NEQ => {
                        self.code.add_asm_line("cmp %rcx, %rax");
                        self.code.add_asm_line("mov $0, %rax");
                        self.code.add_asm_line("setne %al");
                    }
                    BinaryOperator::LT => {
                        self.code.add_asm_line("cmp %rcx, %rax");
                        self.code.add_asm_line("mov $0, %rax");
                        self.code.add_asm_line("setl %al");
                    }
                    BinaryOperator::GT => {
                        self.code.add_asm_line("cmp %rcx, %rax");
                        self.code.add_asm_line("mov $0, %rax");
                        self.code.add_asm_line("setg %al");
                    }
                    BinaryOperator::LE => {
                        self.code.add_asm_line("cmp %rcx, %rax");
                        self.code.add_asm_line("mov $0, %rax");
                        self.code.add_asm_line("setle %al");
                    }
                    BinaryOperator::GE => {
                        self.code.add_asm_line("cmp %rcx, %rax");
                        self.code.add_asm_line("mov $0, %rax");
                        self.code.add_asm_line("setge %al");
                    }
                    _ => todo!(),
                }
            }

            Expression::Variable(name) => todo!(),
        }
    }
}
