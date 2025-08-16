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
    } else if command.starts_with("function")
        || command.starts_with("return")
        || command.starts_with("call")
    {
        return VmCommand::Function(command.to_string());
    } else {
        // VM Branching commands are, goto, if-goto and label
        // Anything else should be invalid VM Command.
        return VmCommand::Branching(command.to_string());
    }
    return vm_command;
}

pub fn generate_machine_code(vm_command: Vec<VmCommand>, file_name: String) -> Vec<String> {
    let mut machine_code = Vec::new();
    let f_name = file_name.as_str();
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
                            "@SP\nM=M-1\nA=M\nA=M\nD=A\nD=-D\n@SP\nA=M\nM=D\n@SP\nM=M+1"
                                .to_string();
                        machine_code.push(line_asm_code);
                    }
                    "eq" => {
                        // Basically, it's a small algo to check if given operands are equal.
                        // Checks if both operands negates themselves to zero, if yes equals, else not.
                        // Included index to make the label unique to each vm command.
                        line_asm_code = format!(
                            "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nA=M\nD=D-A\n@EQ_RETURN.{}\nD;JEQ\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@END_EQ.{}\n0;JMP\n(EQ_RETURN.{})\n@SP\nA=M\nM=-1\n@SP\nM=M+1\n@END_EQ.{}\n0;JMP\n(END_EQ.{})\n@SP",
                            i, i, i, i, i
                        );
                        machine_code.push(line_asm_code);
                    }
                    "gt" => {
                        line_asm_code = format!(
                            "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nA=M\nD=A-D\n@POSITIVE_GT.{}\nD;JGT\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@END_GT.{}\n0;JMP\n(POSITIVE_GT.{})\n@SP\nA=M\nM=-1\n@SP\nM=M+1\n@END_GT.{}\n0;JMP\n(END_GT.{})\n@SP",
                            i, i, i, i, i
                        );
                        machine_code.push(line_asm_code);
                    }
                    "lt" => {
                        line_asm_code = format!(
                            "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nA=M\nD=D-A\n@POSITIVE_LT.{}\nD;JGT\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@END_LT.{}\n0;JMP\n(POSITIVE_LT.{})\n@SP\nA=M\nM=-1\n@SP\nM=M+1\n@END_LT.{}\n0;JMP\n(END_LT.{})\n@SP",
                            i, i, i, i, i
                        );
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
                            "@SP\nM=M-1\nA=M\nD=M\nD=!D\n@SP\nA=M\nM=D\n@SP\nM=M+1".to_string();
                        machine_code.push(line_asm_code);
                    }
                    _ => {
                        println!(
                            "This should not be printed in console. Investigate! {} TEST",
                            command
                        );
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
                            // ARG is RAM[2]
                            line_asm_code = format!(
                                "@{}\nD=A\n@2\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "local" => {
                            // Implement push local.
                            // local is RAM[1]
                            line_asm_code = format!(
                                "@{}\nD=A\n@1\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "static" => {
                            line_asm_code =
                                format!("@{}.{}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1", f_name, value);
                            machine_code.push(line_asm_code);
                        }
                        "constant" => {
                            // Constants are well, constants! So the value is being pushed here.
                            line_asm_code = format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1", value);
                            machine_code.push(line_asm_code);
                        }
                        "this" => {
                            line_asm_code = format!(
                                // this is RAM[3]
                                "@{}\nD=A\n@3\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "that" => {
                            line_asm_code = format!(
                                // that is RAM[4]
                                "@{}\nD=A\n@4\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "pointer" => {
                            match value.as_str() {
                                "0" => {
                                    // pointer 0 is THIS Pointer, which is RAM[3]
                                    line_asm_code = format!(
                                        // this is RAM[3]
                                        "@3\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1"
                                    );
                                    machine_code.push(line_asm_code);
                                }
                                "1" => {
                                    // pointer 1 is THAT Pointer, which is RAM[4]
                                    line_asm_code = format!(
                                        // this is RAM[3]
                                        "@4\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1"
                                    );
                                    machine_code.push(line_asm_code);
                                }
                                _ => {
                                    println!(
                                        "This should not be printed in console. Investigate!{}",
                                        command
                                    );
                                }
                            }
                        }
                        "temp" => {
                            line_asm_code = format!(
                                // TEMP is a segment not address.
                                // RAM[5]-RAM[12]. That being said, any accesss to temp[i] should be
                                // translated into temp[i+5]
                                "@5\nD=A\n@{}\nD=D+A\nA=D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        _ => {}
                    }
                } else {
                    // For Pop off the stack.
                    match segment.as_str() {
                        "argument" => {
                            // Similar implementation.
                            line_asm_code = format!(
                                "@SP\nM=M-1\nA=M\nD=M\n@13\nM=D\n@{}\nD=A\n@2\nD=D+M\n@14\nM=D\n@13\nD=M\n@14\nA=M\nM=D",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "local" => {
                            // Pop off the stack and put it into said local memory segment
                            // Here RAM[13] and RAM[14] are used as temporary assignment.
                            line_asm_code = format!(
                                "@SP\nM=M-1\nA=M\nD=M\n@13\nM=D\n@{}\nD=A\n@1\nD=D+M\n@14\nM=D\n@13\nD=M\n@14\nA=M\nM=D",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "static" => {
                            line_asm_code =
                                format!("@SP\nM=M-1\nA=M\nD=M\n@{}.{}\nM=D", f_name, value);
                            machine_code.push(line_asm_code);
                        }
                        "constant" => {
                            // There is no implementation for this.
                        }
                        "this" => {
                            line_asm_code = format!(
                                "@SP\nM=M-1\nA=M\nD=M\n@13\nM=D\n@{}\nD=A\n@3\nD=D+M\n@14\nM=D\n@13\nD=M\n@14\nA=M\nM=D",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "that" => {
                            line_asm_code = format!(
                                "@SP\nM=M-1\nA=M\nD=M\n@13\nM=D\n@{}\nD=A\n@4\nD=D+M\n@14\nM=D\n@13\nD=M\n@14\nA=M\nM=D",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "pointer" => match value.as_str() {
                            "0" => {
                                line_asm_code = format!("@SP\nM=M-1\nA=M\nD=M\n@3\nM=D");
                                machine_code.push(line_asm_code);
                            }
                            "1" => {
                                line_asm_code = format!("@SP\nM=M-1\nA=M\nD=M\n@4\nM=D");
                                machine_code.push(line_asm_code);
                            }
                            _ => {}
                        },
                        "temp" => {
                            // Temp is going to be differennt implementation than others.
                            // As temp is accessed differently by adding 5 to the index.
                            line_asm_code = format!(
                                "@SP\nM=M-1\nA=M\nD=M\n@13\nM=D\n@5\nD=A\n@{}\nD=A+D\n@14\nM=D\n@13\nD=M\n@14\nA=M\nM=D",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        _ => {
                            println!("This should not be printed in console. POP!{}", command);
                        }
                    }
                }
            }
            VmCommand::Function(command) => {
                let splitted_command: Vec<&str> = command.split(" ").collect();
                if splitted_command.len() < 2 {
                    let function_name = splitted_command[0];
                    println!("RETURN??? {}", function_name);
                } else {
                    let function_name = splitted_command[0];
                    let signature = splitted_command[1];
                    println!(
                        "Alas! function not implemented, yet. {} {}",
                        function_name, signature
                    );
                }
                line_asm_code = format!("// Waiting for implementation of function call.");
                machine_code.push(line_asm_code);
            }
            VmCommand::Branching(command) => {
                // Branching commands are two words each line.
                // First is pre_op second the post_op is the lable to goto.
                let splitted_command: Vec<&str> = command.split(" ").collect();
                let pre_op = splitted_command[0];
                let post_op = splitted_command[1];

                match pre_op {
                    "goto" => {
                        // Unconditional jump with 0;JMP
                        line_asm_code = format!("@{}\n0;JMP", post_op);
                        machine_code.push(line_asm_code);
                    }
                    "if-goto" => {
                        // Stack's topmost value is popped
                        // If the value is NOT ZERO Jump to label
                        // If value is zero 
                        line_asm_code = format!("@SP\nM=M-1\nA=M\nD=M\n@{}\nD;JNE", post_op);
                        machine_code.push(line_asm_code);
                    }
                    "label" => {
                        line_asm_code = format!("({})", post_op);
                        machine_code.push(line_asm_code);
                    }
                    _ => {}
                }
            }
        }
    }
    return machine_code;
}
