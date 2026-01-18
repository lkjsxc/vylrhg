use super::parser::{Instr, Program};

#[derive(Debug, Default)]
pub struct Vm {
    stack: Vec<i32>,
    ip: usize,
    halted: bool,
}

#[derive(Debug, Clone)]
pub struct VmResult {
    pub halted: bool,
    pub stack: Vec<i32>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            ip: 0,
            halted: false,
        }
    }

    pub fn run(&mut self, program: &Program) -> VmResult {
        while self.ip < program.instructions.len() && !self.halted {
            match program.instructions[self.ip].clone() {
                Instr::Nop => {}
                Instr::LoadI32(value) => self.stack.push(value),
                Instr::Add => {
                    let rhs = self.stack.pop().unwrap_or(0);
                    let lhs = self.stack.pop().unwrap_or(0);
                    self.stack.push(lhs + rhs);
                }
                Instr::Halt => self.halted = true,
            }
            self.ip += 1;
        }

        VmResult {
            halted: self.halted,
            stack: self.stack.clone(),
        }
    }

    pub fn reset(&mut self) {
        self.stack.clear();
        self.ip = 0;
        self.halted = false;
    }
}
