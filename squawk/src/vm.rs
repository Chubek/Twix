use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Opcode {
    LoadConstant(Value),
    LoadVariable(String),
    StoreVariable(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Print,
    JumpIfTrue(usize),
    JumpIfFalse(usize),
    Jump(usize),
    Halt,
    MatchEre(String),
    Print,
    Printf,
    ReadLine,
}

#[derive(Debug, Clone)]
enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Ere(String),
}

struct AwkVM {
    opcodes: Vec<Opcode>,
    stack: Vec<Value>,
    environ: HashMap<String, Value>,
}

impl AwkVM {
    fn new(opcodes: Vec<Opcode>) -> Self {
        AwkVM {
            opcodes,
            stack: Vec::new(),
            environ: HashMap::new(),
        }
    }

    fn run(&mut self) {
        let mut ip = 0;
        while ip < self.opcodes.len() {
            match &self.opcodes[ip] {
                Opcode::LoadConstant(value) => {
                    self.stack.push(value.clone());
                }
                Opcode::LoadVariable(variable) => {
                    if let Some(value) = self.environ.get(variable) {
                        self.stack.push(value.clone());
                    } else {
                        panic!("Variable {} not found", variable);
                    }
                }
                Opcode::StoreVariable(variable) => {
                    let value = self.stack.pop().expect("Stack underflow");
                    self.environ.insert(variable.clone(), value);
                }
                Opcode::Add => {
                    let right = self.stack.pop().expect("Stack underflow");
                    let left = self.stack.pop().expect("Stack underflow");
                    self.stack.push(self.add_values(left, right));
                }
                Opcode::Print => {
                    let value = self.stack.pop().expect("Stack underflow");
                    println!("{}", self.format_value(&value));
                }
                Opcode::JumpIfTrue(target) => {
                    let condition = self.stack.pop().expect("Stack underflow");
                    if self.is_truthy(&condition) {
                        ip = *target;
                        continue;
                    }
                }
                Opcode::Jump(target) => {
                    ip = *target;
                    continue;
                }
                Opcode::Halt => {
                    break;
                }
            }
            ip += 1;
        }
    }

}
