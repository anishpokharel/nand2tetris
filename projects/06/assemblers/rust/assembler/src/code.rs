use crate::code;
use crate::parser::Instruction;
use crate::symbol_table::SymbolTable;

pub fn instruction_to_bin(instruction: &Instruction, symbol_table: &mut SymbolTable) -> String {
    match instruction {
        Instruction::AInstruction(value) => {
            format!("{:015b}", value.parse::<u16>().unwrap())
        }
        Instruction::CInstruction { dest, comp, jump } => {
            let dest_bin = code::dest_to_bin(dest.as_deref());
            let comp_bin = code::comp_to_bin(comp);
            let jump_bin = code::jump_to_bin(jump.as_deref());
            format!("111{}{}{}", comp_bin, dest_bin, jump_bin)
        }
        Instruction::Label(_) => {
            // Labels are not converted to binary, they are used for resolving addresses later.
            String::new()
        }
        Instruction::Variable(value) => {
            // Request is variable to be converted into binary. First check if it exists in the symbol table.
            // If it does, return the address in binary. If not, add it to the symbol table and return the address in binary.
            let value_exists = symbol_table.contains(value);
            if !value_exists {
                // If the value does not exist, add it to the symbol table
                let address = symbol_table.add_variable(value.clone());
                return format!("{:015b}", address);
            } else {
                // If the value exists, get the address from the symbol table
                let address = symbol_table.get_address(value).unwrap();
                return format!("{:015b}", address);
            }
        }
    }
}

pub fn dest_to_bin(dest: Option<&str>) -> &'static str {
    match dest {
        Some("M") => "001",
        Some("D") => "010",
        Some("MD") => "011",
        Some("A") => "100",
        Some("AM") => "101",
        Some("AD") => "110",
        Some("AMD") => "111",
        _ => "000",
    }
}

pub fn comp_to_bin(comp: &str) -> &'static str {
    match comp.trim() {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        _ => panic!("Invalid comp mnemonic: {}", comp),
    }
}

pub fn jump_to_bin(jump: Option<&str>) -> &'static str {
    match jump {
        Some("JGT") => "001",
        Some("JEQ") => "010",
        Some("JGE") => "011",
        Some("JLT") => "100",
        Some("JNE") => "101",
        Some("JLE") => "110",
        Some("JMP") => "111",
        _ => "000",
    }
}
