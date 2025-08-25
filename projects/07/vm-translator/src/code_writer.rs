use std::fmt::{self};

use crate::exiting_sequence;

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
        vm_command = VmCommand::Function(command.to_string());
    } else {
        // VM Branching commands are, goto, if-goto and label
        // Anything else should be invalid VM Command.
        vm_command = VmCommand::Branching(command.to_string());
    }
    return vm_command;
}

pub fn generate_machine_code(
    vm_command: Vec<VmCommand>,
    output_file_name: String,
    input_file_name: String,
    include_bootstrap_code: bool,
) -> Vec<String> {
    let mut machine_code = Vec::new();
    let f_name = input_file_name.as_str();

    if include_bootstrap_code {
        let initiliaze_stack_base = "@256\nD=A\n@SP\nM=D".to_string();
        machine_code.push(initiliaze_stack_base);
        // Call Sys.init in the bootstrap code.
        machine_code.extend(call_sys_init(0, output_file_name.clone()));
    }

    for (i, command) in vm_command.iter().enumerate() {
        let mut line_asm_code: String;
        match command {
            VmCommand::AirthLogic(command) => {
                match command.as_str() {
                    "add" => {
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
                            "This should not be printed in console. Investigate! {} Arithmetic.",
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
                            line_asm_code = format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1", value);
                            machine_code.push(line_asm_code);
                        }
                        "this" => {
                            line_asm_code = format!(
                                "@{}\nD=A\n@3\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "that" => {
                            line_asm_code = format!(
                                "@{}\nD=A\n@4\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        "pointer" => {
                            match value.as_str() {
                                "0" => {
                                    line_asm_code = format!(
                                        // this is RAM[3]
                                        "@3\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1"
                                    );
                                    machine_code.push(line_asm_code);
                                }
                                "1" => {
                                    line_asm_code = format!("@4\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
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
                                "@5\nD=A\n@{}\nD=D+A\nA=D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                                value
                            );
                            machine_code.push(line_asm_code);
                        }
                        _ => {
                            println!("This should not be printed in console. PUSH!{}", command);
                        }
                    }
                } else {
                    match segment.as_str() {
                        "argument" => {
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
                            // Can't pop a constant.
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
                            // A temp is accessed differently by adding 5 to the index.
                            line_asm_code = format!(
                                "@SP\nM=M-1\nA=M\nD=M\n@13\nM=D\n@5\nD=A\n@{}\nD=D+A\n@14\nM=D\n@13\nD=M\n@14\nA=M\nM=D",
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
                let function_type = splitted_command[0];
                match function_type {
                    "function" => {
                        let function_name = splitted_command[1];
                        let mut n_vars: u32 = splitted_command[2].parse().unwrap();
                        line_asm_code = format!("({}.{})", output_file_name, function_name);
                        machine_code.push(line_asm_code);

                        while n_vars > 0 {
                            n_vars -= 1;
                            // Push zeros to the stack.
                            line_asm_code = format!("@0\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1");
                            machine_code.push(line_asm_code);
                        }
                    }
                    "call" => {
                        let function_name = splitted_command[1];

                        let n_args: u32 = splitted_command[2].parse().unwrap();

                        // push returnaddress
                        line_asm_code = format!(
                            "@RET_{}.{}.{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1",
                            output_file_name, function_name, i
                        );
                        machine_code.push(line_asm_code);

                        // push LCL
                        line_asm_code = format!("@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
                        machine_code.push(line_asm_code);

                        // push ARG
                        line_asm_code = format!("@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
                        machine_code.push(line_asm_code);

                        // push THIS
                        line_asm_code = format!("@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
                        machine_code.push(line_asm_code);

                        // push THAT
                        line_asm_code = format!("@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
                        machine_code.push(line_asm_code);

                        // reposition ARG = SP-5-n_args
                        line_asm_code =
                            format!("@{}\nD=A\n@SP\nD=M-D\n@5\nD=D-A\n@ARG\nM=D", n_args);
                        machine_code.push(line_asm_code);

                        // reposition LCL = SP
                        line_asm_code = format!("@SP\nD=M\n@LCL\nM=D");
                        machine_code.push(line_asm_code);

                        // then goto the calee..
                        line_asm_code = format!("@{}.{}\n0;JMP", output_file_name, function_name);
                        machine_code.push(line_asm_code);

                        // INJECT (returnAddress) label into the code.
                        line_asm_code =
                            format!("(RET_{}.{}.{})", output_file_name, function_name, i);
                        machine_code.push(line_asm_code);
                    }
                    "return" => {
                        // frame = LCL ;; frame is a temporary variable.
                        line_asm_code = format!("@LCL\nD=M\n@13\nM=D"); // 13 is temporary variable frame.
                        machine_code.push(line_asm_code);

                        // returnAddress = *(frame - 5) ;; puts the return address in temporary variable.
                        line_asm_code = format!("@13\nD=M\n@5\nD=D-A\nA=D\nD=M\n@14\nM=D"); // 14 is temporary return address.
                        machine_code.push(line_asm_code);

                        // *ARG = pop()
                        line_asm_code = format!("@SP\nM=M-1\nA=M\nD=M\n@ARG\nA=M\nM=D");
                        machine_code.push(line_asm_code);

                        // SP = ARG + 1
                        line_asm_code = format!("@ARG\nD=M\n@SP\nM=D+1");
                        machine_code.push(line_asm_code);

                        // THAT = frame - 1
                        line_asm_code = format!("@13\nD=M\n@1\nD=D-A\nA=D\nD=M\n@THAT\nM=D");
                        machine_code.push(line_asm_code);

                        // THIS = frame - 2
                        line_asm_code = format!("@13\nD=M\n@2\nD=D-A\nA=D\nD=M\n@THIS\nM=D");
                        machine_code.push(line_asm_code);

                        // ARG = frame - 3
                        line_asm_code = format!("@13\nD=M\n@3\nD=D-A\nA=D\nD=M\n@ARG\nM=D");
                        machine_code.push(line_asm_code);

                        // LCL = frame - 4
                        line_asm_code = format!("@13\nD=M\n@4\nD=D-A\nA=D\nD=M\n@LCL\nM=D");
                        machine_code.push(line_asm_code);

                        // goto returnAddress
                        line_asm_code = format!("@14\nA=M\n0;JMP");
                        machine_code.push(line_asm_code);
                    }
                    _ => {
                        println!("Invalid input command. RETURN {}", command);
                    }
                }
            }
            VmCommand::Branching(command) => {
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
                        line_asm_code = format!("@SP\nM=M-1\nA=M\nD=M\n@{}\nD;JNE", post_op);
                        machine_code.push(line_asm_code);
                    }
                    "label" => {
                        line_asm_code = format!("({})", post_op);
                        machine_code.push(line_asm_code);
                    }
                    _ => {
                        println!("WRONG INPUT COMMAND FOUND, BRANCHING {}", command);
                        exiting_sequence(true);
                    }
                }
            }
        }
    }
    return machine_code;
}

fn call_sys_init(n_args: u32, file_name: String) -> Vec<String> {
    let mut machine_code = Vec::new();
    let mut line_asm_code: String;
    let function_name = "Sys.init";

    // push returnaddress
    line_asm_code = format!(
        "@RET_{}.{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1",
        file_name, function_name
    );
    machine_code.push(line_asm_code);

    // push LCL
    line_asm_code = format!("@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    machine_code.push(line_asm_code);

    // push ARG
    line_asm_code = format!("@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    machine_code.push(line_asm_code);

    // push THIS
    line_asm_code = format!("@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    machine_code.push(line_asm_code);

    // push THAT
    line_asm_code = format!("@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1");
    machine_code.push(line_asm_code);

    // reposition ARG = SP-5-n_args
    line_asm_code = format!("@{}\nD=A\n@SP\nD=M-D\n@5\nD=D-A\n@ARG\nM=D", n_args);
    machine_code.push(line_asm_code);

    // reposition LCL = SP
    line_asm_code = format!("@SP\nD=M\n@LCL\nM=D");
    machine_code.push(line_asm_code);

    // then goto the calee..
    line_asm_code = format!("@{}.{}\n0;JMP", file_name, function_name);
    machine_code.push(line_asm_code);

    // INJECT (returnAddress) label into the code.
    line_asm_code = format!("(RET_{}.{})", file_name, function_name);
    machine_code.push(line_asm_code);

    return machine_code;
}
