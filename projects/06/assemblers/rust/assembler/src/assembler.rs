use crate::code::instruction_to_bin;
use crate::parser;
use crate::symbol_table::SymbolTable;

    // In the process of assembling, we will parse the source code,
    // if the assembly process is successful, we will return the machine code.
pub fn assemble(source: &str, symbol_table: &mut SymbolTable) -> Result<Vec<String>, String> {
    // Parser makes two pases, in the first pass, it finds all the labels and adds them to the symbol table.
    // In the second pass, it parses the instructions and generates the machine code.
    parser::find_label(source, symbol_table);
    let parsed_instruction = parser::parse_lines(source);
    let mut machine_code: Vec<String> = vec![];
    for instruction in parsed_instruction {
        let binary_instruction: String;
        let inst = instruction.clone();
        match instruction {
            parser::Instruction::AInstruction(_) => {
                binary_instruction = instruction_to_bin(&inst, symbol_table);
            }
            parser::Instruction::CInstruction { .. } => {
                binary_instruction = instruction_to_bin(&inst, symbol_table);
            }
            parser::Instruction::Label(_) => {
                // Labels are not converted to machine code, they are just used for reference.
                continue;
            }
            parser::Instruction::Variable(_) => {
                binary_instruction = instruction_to_bin(&inst, symbol_table);
            }
        }
        if !binary_instruction.is_empty() {
        machine_code.push(binary_instruction);
        }
    }
    // Placeholder for the machine code output

    // Return the machine code as a result
    Ok(machine_code)
}