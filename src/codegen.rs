use std::collections::HashMap;

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

#[derive(Clone)]
pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub symbols: HashMap<String, i64>,
    pub stack_index: i64,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            parent: None,
            symbols: HashMap::new(),
            stack_index: -4,
        }
    }

    pub fn from_parent(parent: Scope) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            symbols: HashMap::new(),
            stack_index: -4,
        }
    }

    pub fn contains_symbol(&self, name: &str) -> bool {
        if self.symbols.contains_key(name) {
            true
        } else if let Some(parent) = &self.parent {
            parent.contains_symbol(name)
        } else {
            false
        }
    }

    pub fn add_symbol(&mut self, name: String) {
        assert!(!self.contains_symbol(&name));
        self.symbols.insert(name, self.stack_index);
        self.stack_index -= 4;
    }

    pub fn get_symbol(&self, name: &str) -> i64 {
        if let Some(offset) = self.symbols.get(name) {
            *offset
        } else {
            panic!("Symbol {} not found", name);
        }
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
        let mut scope = Scope::new();
        for func_decl in program.declarations {
            self.generate_func_decl(&mut scope, func_decl);
        }
    }

    fn generate_func_decl(&mut self, parent_scope: &mut Scope, func_decl: FunctionDeclaration) {
        let mut scope = Scope::from_parent(parent_scope.clone());
        self.code.add_asm_line(&format!(".globl {}", func_decl.name));
        self.code.add_asm_line(&format!("{}:", func_decl.name));
        self.code.add_asm_line("push %rbp");
        self.code.add_asm_line("mov %rsp, %rbp");
        for stmt in func_decl.body {
            self.generate_stmt(&mut scope, stmt);
        }
        self.code.add_asm_line("xor %rax, %rax");
        self.code.add_asm_line("mov %rbp, %rsp");
        self.code.add_asm_line("pop %rbp");
        self.code.add_asm_line("ret");
    }

    fn generate_stmt(&mut self, scope: &mut Scope, stmt: Statement) {
        match stmt {
            Statement::Return(expr) => {
                self.generate_expr(scope, expr);
                self.code.add_asm_line("mov %rbp, %rsp");
                self.code.add_asm_line("pop %rbp");
                self.code.add_asm_line("ret");
            }
            Statement::Declare(name, None) => {
                self.code.add_asm_line("sub $4, %rsp");
                scope.add_symbol(name);
            }
            Statement::Declare(name, Some(expr)) => {
                let offset = scope.stack_index;
                scope.add_symbol(name);
                self.generate_expr(scope, expr);
                self.code.add_asm_line("sub $4, %rsp");
                self.code.add_asm_line(&format!("mov %eax, {}(%rbp)", offset));
            }
            Statement::Expression(expr) => {
                self.generate_expr(scope, expr);
            }
        }
    }

    fn generate_expr(&mut self, scope: &mut Scope, expr: Expression) {
        match expr {
            Expression::Int(x) => {
                self.code.add_asm_line(&format!("mov ${}, %rax", x));
            }
            Expression::UnaryOperation(op, expr) => {
                self.generate_expr(scope, *expr);
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
                self.generate_expr(scope, *left);
                self.code.add_asm_line("cmp $0, %rax");
                self.code.add_asm_line(&format!("je {}", clause2));
                self.code.add_asm_line("mov $1, %rax");
                self.code.add_asm_line(&format!("jmp {}", end));
                self.code.add_label(clause2);
                self.generate_expr(scope, *right);
                self.code.add_asm_line("cmp $0, %rax");
                self.code.add_asm_line("mov $0, %rax");
                self.code.add_asm_line("setne %al");
                self.code.add_label(end);
            }

            Expression::BinaryOperation(left, BinaryOperator::LogicAnd, right) => {
                let clause2 = self.get_label();
                let end = self.get_label();
                self.generate_expr(scope, *left);
                self.code.add_asm_line("cmp $0, %rax");
                self.code.add_asm_line(&format!("jne {}", clause2));
                self.code.add_asm_line("mov $0, %rax");
                self.code.add_asm_line(&format!("jmp {}", end));
                self.code.add_label(clause2);
                self.generate_expr(scope, *right);
                self.code.add_asm_line("cmp $0, %rax");
                self.code.add_asm_line("mov $0, %rax");
                self.code.add_asm_line("setne %al");
                self.code.add_label(end);
            }

            Expression::BinaryOperation(left, BinaryOperator::Assign, right) => {
                if let Expression::Variable(name) = *left {
                    self.generate_expr(scope, *right);
                    self.code.add_asm_line(
                        &format!(
                            "movl %eax, {}(%rbp)",
                            scope.get_symbol(&name)
                        )
                    );
                } else {
                    panic!("Invalid assignment");
                }
            }

            Expression::BinaryOperation(left, op, right) => {
                self.generate_expr(scope, *right);
                self.code.add_asm_line("push %rax");
                self.generate_expr(scope, *left);
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
                    _ => unreachable!(),
                }
            }

            Expression::Variable(name) => {
                let offset = scope.get_symbol(&name);
                self.code.add_asm_line("xor %rax, %rax");
                self.code.add_asm_line(&format!("movl {}(%rbp), %eax", offset));
            }
        }
    }
}
