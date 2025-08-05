use std::fmt;

#[derive(Debug, Clone)]
pub enum Instruction {
    AInstruction(String),
    CInstruction {
        dest: Option<String>,
        comp: String,
        jump: Option<String>,
    },
    Label(String),
    Variable(String),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::AInstruction(value) => write!(f, "@{}", value),
            Instruction::CInstruction { dest, comp, jump } => {
                let dest_str = dest.as_ref().map_or(String::new(), |d| format!("{}=", d));
                let jump_str = jump.as_ref().map_or(String::new(), |j| format!(";{}", j));
                write!(f, "{}{}{}", dest_str, comp, jump_str)
            }
            Instruction::Label(name) => write!(f, "({})", name),
            Instruction::Variable(value) => write!(f, "{}", value),
        }
    }
}

fn strip_comment(line: &str) -> &str {
    match line.find("//") {
        Some(index) => &line[..index],
        None => line,
    }
}

pub fn parse_lines(source: &str) -> Vec<Instruction> {
    // Parse each lines of the source code and generate Vector of Instructions.
    let mut instructions = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue; // Skip empty lines and comments
        }
        let stripped: &str;
        if trimmed.contains("//") {
            stripped = strip_comment(trimmed);
        } else {
            stripped = trimmed;
        }

        if stripped.starts_with('@') {
            // THIS IS WHERE YOUR THOUGHT PROCESS GOES
            // Here it could be an A-instruction or a variable
            // If it starts with '@' and is followed by a number, it's an A-instruction
            let value: String = stripped[1..].to_string();
            if value.chars().all(|c| c.is_digit(10)) {
                // If it's all digits, it's an A-instruction
                instructions.push(Instruction::AInstruction(value));
            } else if value.chars().all(|c| c.is_alphanumeric() || c == '_') {
                // If it's alphanumeric or underscore, it's a variable
                // If it starts with '@' and is followed by a variable name, it's a variable
                instructions.push(Instruction::Variable(value));
            }
            instructions.push(Instruction::AInstruction(stripped[1..].to_string()));
        } else if stripped.contains('=') || stripped.contains(';') {
            let parts: Vec<&str> = stripped.split(';').collect();
            let comp_dest: Vec<&str> = parts[0].split('=').collect();
            let dest = if comp_dest.len() > 1 {
                Some(comp_dest[0].to_string())
            } else {
                None
            };
            let comp = comp_dest.last().unwrap().to_string();
            let jump = if parts.len() > 1 {
                Some(parts[1].to_string())
            } else {
                None
            };
            instructions.push(Instruction::CInstruction { dest, comp, jump });
        } else if stripped.starts_with('(') && stripped.ends_with(')') {
            instructions.push(Instruction::Label(
                stripped[1..stripped.len() - 1].to_string(),
            ));
        }
    }
    instructions
}
