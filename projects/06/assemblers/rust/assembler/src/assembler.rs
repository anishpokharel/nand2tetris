use crate::code::instruction_to_bin;
use crate::parser;
use crate::symbol_table::SymbolTable;

pub fn assemble(source: &str, symbol_table: &mut SymbolTable) -> Result<Vec<String>, String> {
    // In the process of assembling, we will parse the source code,
    // if the assembly process is successful, we will return the machine code.
    parser::find_label(source, symbol_table);
    let parsed_instruction = parser::parse_lines(source, symbol_table);
    let mut machine_code: Vec<String> = vec![];
    for instruction in parsed_instruction {
        let mut binary_instruction: String = "".to_string();
        let inst = instruction.clone();
        match instruction {
            parser::Instruction::AInstruction(value) => {
                // If it's an A-instruction, convert it to binary
                println!("Assembling A-instruction: {}", value);
                binary_instruction = instruction_to_bin(&inst, symbol_table);
            }
            parser::Instruction::CInstruction { dest, comp, jump } => {
                println!(
                    "Assembling C-instruction: dest: {:?}, comp: {}, jump: {:?}",
                    dest, comp, jump
                );
                binary_instruction = instruction_to_bin(&inst, symbol_table);
            }
            parser::Instruction::Label(name) => {
                println!("Assemble label? Think how.: {}", name);
            }
            parser::Instruction::Variable(value) => {
                println!("Assembling variable: {}", value);
                binary_instruction = instruction_to_bin(&inst, symbol_table);
            }
        }
        machine_code.push(binary_instruction);
    }
    // Placeholder for the machine code output

    // Return the machine code as a result
    Ok(machine_code)
}
