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
    Boolean(bool),
    List(Vec<Value>),
}

impl Value {
    pub fn is_truthy(self) -> bool {
        match self {
            Self::Integer(num) => num > 0,
            Self::Float(rational) => rational > 0,
            Self::String(string) => !string.is_empty(),
            Self::Ere(string) => !string.is_empty(),
            Self::Boolean(b) => b,
        }
    }

    pub fn is_falsy(self) -> bool {
        !self.is_truthy()
    }

    pub fn add_with(self, other: Value) -> Value {
        match self {
            Self::Integer(num_self) => {
                if Self::Integer(num_other) = other {
                    Self::Integer(num_self + num_other)
                } else if Self::Float(rational_other) = other {
                    Self::Integer(num_self + (rational_other as i64))
                } else {
                    panic!("Error: addition between wrong operand types");
                }
            },
            Self::Float(rational_self) => {
                if Self::Float(rational_other) = other {
                    Self::Float(rational_self + rational_other);
                } else if Self::Integer(num_other) = other {
                    Self::Float(rational_self + (num_other as f64))
                } else {
                    panic!("Error: addition between wrong operand types");
                }
            },
            Self::String(string_self) => {
                if Self::String(string_other) = other {
                    Self::String(string_self + string_other)
                } else if Self::Integer(num_other) = other {
                    Self::String(string_self + num_other.to_string())
                } else if Self::Float(rational_other) = other {
                    Self::String(string_self + rational_other.to_string())
                } else {
                    panic!("Error: addition between wrong operand types");
                }
            },
            Self::List(list_self) => {
                if Self::List(list_other) = other {
                    Self::List(list_self.extend(list_other))
                } else {
                    panic!("Error: addition between wrong operand types");
                }
            },
            _ => panic!("Error: addition is not supported between types"),
        }
    }

    pub fn sub_with(self, other: Value) -> Value {
        match self { 
        Self::Integer(num_self) => {
                if Self::Integer(num_other) = other {
                    Self::Integer(num_self - num_other)
                } else if Self::Float(rational_other) = other {
                    Self::Integer(num_self - (rational_other as i64))
                } else {
                    panic!("Error: subtraction between wrong operand types");
                }
            },
            Self::Float(rational_self) => {
                if Self::Float(rational_other) = other {
                    Self::Float(rational_self - rational_other);
                } else if Self::Integer(num_other) = other {
                    Self::Float(rational_self - (num_other as f64))
                } else {
                    panic!("Error: subtraction between wrong operand types");
                }
            },
            _ => panic!("Error: subtraction between wrong operand types"),
        }
    }

    pub fn mul_with(self, other: Value) -> Value {
        match self { 
        Self::Integer(num_self) => {
                if Self::Integer(num_other) = other {
                    Self::Integer(num_self * num_other)
                } else if Self::Float(rational_other) = other {
                    Self::Integer(num_self * (rational_other as i64))
                } else {
                    panic!("Error: multipliication between wrong operand types");
                }
            },
            Self::Float(rational_self) => {
                if Self::Float(rational_other) = other {
                    Self::Float(rational_self * rational_other);
                } else if Self::Integer(num_other) = other {
                    Self::Float(rational_self * (num_other as f64))
                } else {
                    panic!("Error: multiplication between wrong operand types");
                }
            },
            _ => panic!("Error: multiplication between wrong operand types"),
    }
    }

    pub fn mod_with(self, other: Value) -> Value {
        if Self::Integer(num_self) = self {
            if Self::Integer(num_other) = other {
                Self::Integer(num_self % num_other)
            }
        } else {
            panic!("Error: modulo between two wrong operand types");
        }
    }
    
    pub fn eq_with(self, other: Value) -> Value {
        match self {
            Self::Integer(num_self) => {
                if Self::Integer(num_other) = other {
                    Self::Boolean(num_self == num_other)
                } else if Self::Float(rational_other) {
                    Self::Boolean(num_self == (rational_other as i64)
                }
            },
            Self::Float(rational_self) => {
                if Self::Integer(num_other) = other {
                    Self::Boolean(rational_self == (num_other as f64))
                } else if Self::Float(rational_other) {
                    Self::Boolean(rational_self == rational_other)
                }
            }
        }
    }

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
