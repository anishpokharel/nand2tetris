use crate::code::instruction_to_bin;
use crate::parser;
use crate::symbol_table::{ SymbolTable};

pub fn assemble(source: &str,  symbol_table: &mut SymbolTable) -> Result<Vec<String>, String> {
    // In the process of assembling, we will parse the source code,
    // if the assembly process is successful, we will return the machine code.
    let parsed_instruction = parser::parse_lines(source);
    let machine_code: Vec<String> = vec![];

    // Create a new symbol table to store symbols and their addresses.
    // This will be used to resolve symbols in A-instructions and labels.

    for instruction in parsed_instruction {
        let mut binary_instruction: String = "".to_string();
        let inst = instruction.clone();
        match instruction {
            parser::Instruction::AInstruction(value) => match value.parse::<i32>() {
                Ok(number_value) => {
                    println!("Parsed number: {}", number_value);
                    binary_instruction = instruction_to_bin(&inst, symbol_table);
                }
                Err(e) => {
                    println!("Failed to parse '{}': {}", value, e);
                    symbol_table.add_variable(value.clone());
                }
            },
            parser::Instruction::CInstruction { dest, comp, jump } => {
                println!("Here: dest: {:?}, comp: {}, jump: {:?}", dest, comp, jump);
            }
            parser::Instruction::Label(name) => {
                println!("Found label: {}", name);
            }
            parser::Instruction::Variable(value) => {
                println!("Found variable: {}", value);
                if !symbol_table.contains(&value) {
                    let address = symbol_table.add_variable(value.clone());
                    binary_instruction = format!("{:015b}", address);
                } else {
                    binary_instruction = instruction_to_bin(&inst, symbol_table);
                }
            }
        }
        println!("Converting Instruction:{} {}", inst, binary_instruction);
    }
    // Placeholder for the machine code output

    // Return the machine code as a result
    print!("Machine code generated: {:?}", machine_code);
    Ok(machine_code)
}
