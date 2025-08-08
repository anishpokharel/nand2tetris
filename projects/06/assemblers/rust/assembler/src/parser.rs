use std::fmt;

use crate::symbol_table::SymbolTable;

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

pub fn find_label(source: &str, symbol_table: &mut SymbolTable) {
    let mut trimmed_source: Vec<String> = Vec::new();
    let mut instruction_number = 0;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue; // Skip empty lines and comments
        }
        let stripped: &str;
        if trimmed.contains("//") {
            stripped = strip_comment(trimmed).trim();
        } else {
            stripped = trimmed;
        }
        trimmed_source.push(stripped.to_string());
    }

    for line in trimmed_source {
        if line.starts_with('(') && line.ends_with(')') {
            let label = line[1..line.len() - 1].to_string();
            if symbol_table.contains(&label) {
                // Do nothing if the label already exists
            } else {
                // Create a label instruction to supress compiler warning.
                Instruction::Label(label.clone());
                // Add label to symbol table with the current instruction number
                symbol_table.add_entry(label.clone(), instruction_number);
            }
        } else {
        instruction_number += 1; // Increment instruction number for non-label lines
        }
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
            stripped = strip_comment(trimmed).trim();
        } else {
            stripped = trimmed;
        }

        if stripped.starts_with('@') {
            // THIS IS WHERE YOUR THOUGHT PROCESS GOES
            // Here it could be an A-instruction or a variable
            // If it starts with '@' and is followed by a number, it's an A-instruction
            let value: String = stripped[1..].trim().to_string();
            let mut a_number = -2;

            // If parsing to i32 is okay, also check if it's less than maximum allowed value.
            // Anything above that number should be treated as a variable.
            if value.parse::<i32>().is_ok() {
                a_number = value.parse::<i32>().unwrap();
            }
            // Maximum allowed value for A-instruction is 24,576 (2^15 - 1).
            if (a_number != -2) && a_number < 32_768 {
                // If it's all digits, it's an A-instruction
                instructions.push(Instruction::AInstruction(value));
            } else {
                // If it's alphanumeric or underscore, it's a variable
                // If it starts with '@' and is followed by a variable name, it's a variable
                instructions.push(Instruction::Variable(value));
            }
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
                Some(parts[1].trim().to_string())
            } else {
                None
            };
            instructions.push(Instruction::CInstruction { dest, comp, jump });
        }
    }
    instructions
}
