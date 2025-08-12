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
    let mut stack_pointer = 256;
    // Initialize Stack Pointer SP.
    let init_stack_pointer = format!("@{}\nD=A\n@SP\nM=D", stack_pointer);

    let mut machine_code = Vec::new();
    machine_code.push(init_stack_pointer); // Adding to machine code vector.
    for command in vm_command {
        let line_asm_code: String;
        println!("{}", command); // Delete this line when ready.
        match command {
            VmCommand::AirthLogic(command) => match command.as_str() {
                "add" => {
                    // Pop two values from the stack.
                    let x_value = format!("@{}\nD=M\n@x\nM=D", stack_pointer);
                    stack_pointer -= 1;
                    let y_value = format!("\n@{}\nD=M\n@y\nM=D", stack_pointer);
                    stack_pointer -= 1;
                    let added_value = format!("\n@x\nD=M\n@y\nA=M\nM=A\nD=D+M");
                    let result = format!("\n@{}\nM=D", stack_pointer);
                    stack_pointer += 1;
                    line_asm_code = format!("{}{}{}{}", x_value, y_value, added_value, result);
                    machine_code.push(line_asm_code);
                }
                "sub" => {
                    // This is my assumption as to how to handle the add & subs.
                    // This might not be how it's done and I have strong feeling.
                    let x_value = format!("@{}\nD=M\n@x\nM=D", stack_pointer);
                    stack_pointer -= 1;
                    let y_value = format!("\n@{}\nD=M\n@y\nM=D", stack_pointer);
                    stack_pointer -= 1;
                    let subtracted_value = format!("\n@x\nD=M\n@y\nA=M\nM=A\nD=D-M");
                    let result = format!("\n@{}\nM=D", stack_pointer);
                    stack_pointer += 1;
                    line_asm_code = format!("{}{}{}{}", x_value, y_value, subtracted_value, result);
                    machine_code.push(line_asm_code);
                }
                "neg" => {
                    let x_value = format!("@{}\nD=M\n@x\nM=D", stack_pointer);
                    stack_pointer -= 1;
                    let negative_value= format!("\n@x\nD=M\nD=-D");
                    let result = format!("\n@{}\nM=D", stack_pointer);
                    stack_pointer += 1;
                    line_asm_code = format!("{}{}{}", x_value, negative_value, result);
                    machine_code.push(line_asm_code);
                }
                "eq" => {
                    // This is utterly stupid. Wrong in many levels. 
                    let x_value = format!("@{}\nD=M\n@x\nM=D", stack_pointer);
                    stack_pointer -= 1;
                    let y_value = format!("\n@{}\nD=M\n@y\nM=D", stack_pointer);
                    stack_pointer -= 1;
                    let eq_compare = format!("\n@x\nD=M\n@y\nA=m\nM=A\nD=D==M");
                    line_asm_code = format!("{}{}{}", x_value, y_value, eq_compare);
                    machine_code.push(line_asm_code);

                }
                "gt" => {
                    stack_pointer -= 1;
                }
                "lt" => {
                    stack_pointer -= 1;
                }
                "and" => {
                    stack_pointer -= 2;
                }
                "or" => {
                    stack_pointer -= 2;
                }
                "not" => {
                    let x_value = format!("@{}\nD=M\n@x\nM=D", stack_pointer);
                    stack_pointer -= 1;
                    let not_value = format!("\n@x\nD=M\nD=!D");
                    let result = format!("\n@{}\nM=D", stack_pointer);
                    stack_pointer += 1;
                    line_asm_code = format!("{}{}{}",x_value, not_value, result);
                    machine_code.push(line_asm_code);
                }
                _ => {
                    println!("This should not be printed in console. Investigate!")
                }
            },
            VmCommand::PushPop {
                d_type,
                segment,
                value,
            } => {
                if d_type.eq("push") {
                    // For Push off the stack.
                    stack_pointer += 1;
                    line_asm_code = format!("@x\nD=M\n@{}\nM=D", stack_pointer);
                    machine_code.push(line_asm_code);
                    segment.contains("pat"); // Delete me
                } else {
                    // For Pop off the stack.
                    stack_pointer -= 1;
                    line_asm_code = format!("@{}\nD=M\n@x\nM=D", stack_pointer);
                    machine_code.push(line_asm_code);
                    value.contains("pat"); // Delete me
                }
            }
            VmCommand::Function(_command) => {}
            VmCommand::Branching(_command) => {}
        }
    }
    return machine_code;
}
