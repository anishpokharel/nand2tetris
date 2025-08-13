use std::fmt;

#[derive(Debug, Clone)]
pub enum VmCommand {
    PushPop {
        d_type: String, // type, either push or pop.
        segment: String,
        value: String,
    },
    AirthLogic(String),
    Branching(String),
    Function(String),
}

impl fmt::Display for VmCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmCommand::PushPop {
                d_type,
                segment,
                value,
            } => {
                write!(f, "{} {} {}", d_type, segment, value)
            }
            VmCommand::AirthLogic(value) => write!(f, "{}", value),
            VmCommand::Branching(value) => write!(f, "{}", value),
            VmCommand::Function(value) => write!(f, "{}", value),
        }
    }
}

/* pub fn generate_code(source_line: &str) -> String {
    let machine_code = source_line.to_string();
    // Detect Command Type.
    return machine_code;
} */

pub fn categorize_commands(command: &str) -> VmCommand {
    let vm_command: VmCommand;
    if command.starts_with("push") || command.starts_with("pop") {
        let splitted_command: Vec<&str> = command.split(" ").collect();
        let d_type = splitted_command[0];
        let segment = splitted_command[1];
        let value = splitted_command[2];
        vm_command = VmCommand::PushPop {
            d_type: d_type.to_string(),
            segment: segment.to_string(),
            value: value.to_string(),
        };
    } else if command.starts_with("add")
        || command.starts_with("sub")
        || command.starts_with("neg")
        || command.starts_with("eq")
        || command.starts_with("gt")
        || command.starts_with("lt")
        || command.starts_with("and")
        || command.starts_with("or")
        || command.starts_with("not")
    {
        vm_command = VmCommand::AirthLogic(command.to_string());
    } else if command.starts_with("fn") {
        return VmCommand::Function("change-me".to_string());
    } else {
        return VmCommand::Branching("me-as-well".to_string());
    }
    return vm_command;
}

pub fn generate_machine_code(vm_command: Vec<VmCommand>) -> Vec<String> {
    let mut machine_code = Vec::new();
    let initiliaze_stack_base = "@256\nD=A\n@SP\nM=D".to_string();
    machine_code.push(initiliaze_stack_base);

    for (i, command) in vm_command.iter().enumerate() {
        let line_asm_code: String;
        match command {
            VmCommand::AirthLogic(command) => {
                match command.as_str() {
                    "add" => {
                        // Pop x & y from stack, add them up and put it back on stack.
                        // Resulting operation will decrease stack size by 1.
                        line_asm_code = "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=D+M\n@SP\nA=M\nM=D\n@SP\nM=M+1".to_string();
                        machine_code.push(line_asm_code);
                    }
                    "sub" => {
                        line_asm_code = "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@SP\nA=M\nM=D\n@SP\nM=M+1".to_string();
                        machine_code.push(line_asm_code);
                    }
                    "neg" => {
                        line_asm_code =
                            "@SP\nM=M-1\nA=M\nA=M\nD=A\nD=-D\n@SP\nA=M\nM=D\n@SP\nM=M+1".to_string();
                        machine_code.push(line_asm_code);
                    }
                    "eq" => {
                        line_asm_code = format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nA=M\nD=D-A\n@EQ_RETURN.{}\nD;JEQ\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@END_EQ.{}\n0;JMP\n(EQ_RETURN.{})\n@SP\nA=M\nM=-1\n@SP\nM=M+1\n@END_EQ.{}\n0;JMP\n(END_EQ.{})\n@SP", i, i, i, i, i);
                        machine_code.push(line_asm_code);
                    }
                    "gt" => {
                        line_asm_code = format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nA=M\nD=A-D\n@POSITIVE_GT.{}\nD;JGT\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@END_GT.{}\n0;JMP\n(POSITIVE_GT.{})\n@SP\nA=M\nM=-1\n@SP\nM=M+1\n@END_GT.{}\n0;JMP\n(END_GT.{})\n@SP", i, i, i, i, i);
                        machine_code.push(line_asm_code);
                    }
                    "lt" => {
                        line_asm_code = format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nA=M\nD=D-A\n@POSITIVE_LT.{}\nD;JGT\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@END_LT.{}\n0;JMP\n(POSITIVE_LT.{})\n@SP\nA=M\nM=-1\n@SP\nM=M+1\n@END_LT.{}\n0;JMP\n(END_LT.{})\n@SP", i, i, i,  i, i);
                        machine_code.push(line_asm_code);
                    }
                    "and" => {
                        line_asm_code = "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nA=M\nD=D&A\n@SP\nA=M\nM=D\n@SP\nM=M+1".to_string();
                        machine_code.push(line_asm_code);
                    }
                    "or" => {
                        line_asm_code = "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nA=M\nD=D|A\n@SP\nA=M\nM=D\n@SP\nM=M+1".to_string();
                        machine_code.push(line_asm_code);
                    }
                    "not" => {
                        line_asm_code =
                            "@SP\nM=M-1\nA=M\nD=M\nD=!D\n@SP\nA=M\nM=D\n@SP\nM=M+1"
                                .to_string();
                        machine_code.push(line_asm_code);
                    }
                    _ => {
                        println!("This should not be printed in console. Investigate!")
                    }
                }
            }
            VmCommand::PushPop {
                d_type,
                segment,
                value,
            } => {
                if d_type.eq("push") {
                    match segment.as_str() {
                        "argument" => {
                            println!("{} not implemented yet.", "argument");
                        }
                        "local" => {
                            println!("{} not implemented yet.", "local");
                        }
                        "static" => {
                            println!("{} not implemented yet.", "static");
                        }
                        "constant" => {
                            line_asm_code = format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1", value);
                            machine_code.push(line_asm_code);
                        }
                        "this" => {
                            println!("{} not implemented yet.", "this");
                        }
                        "that" => {
                            println!("{} not implemented yet.", "that");
                        }
                        "pointer" => {
                            println!("{} not implemented yet.", "pointer");
                        }
                        "temp" => {
                            println!("{} not implemented yet.", "temp");
                        }
                        _ => {}
                    }
                } else {
                    // For Pop off the stack.
                }
            }
            VmCommand::Function(_command) => {}
            VmCommand::Branching(_command) => {}
        }
    }
    return machine_code;
}
