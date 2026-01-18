#[derive(Debug, Clone)]
pub enum Instr {
    Nop,
    LoadI32(i32),
    Add,
    Halt,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub instructions: Vec<Instr>,
}

pub fn parse_program(source: &str) -> Program {
    let mut instructions = Vec::new();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["nop"] => instructions.push(Instr::Nop),
            ["load.i32", value] => {
                let parsed = value.parse::<i32>().unwrap_or(0);
                instructions.push(Instr::LoadI32(parsed));
            }
            ["add"] => instructions.push(Instr::Add),
            ["halt"] => instructions.push(Instr::Halt),
            _ => instructions.push(Instr::Nop),
        }
    }
    Program { instructions }
}
