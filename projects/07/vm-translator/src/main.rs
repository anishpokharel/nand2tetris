/* VM Translator is basically a program that reads the VM commands,
  one command at a time, and translates each command into Hack Instructions.
*/
mod code_writer;
mod parser;

use std::env;
use std::fs::File;
use std::io::{Read, Write};

use crate::code_writer::generate_machine_code;

fn main() {
    // Read the file name from the arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: translator <file-name.vm>");
        return;
    }
    let input_file_name: &String = &args[1];
    // Open that file up.
    let message = format!(
        "Failed to open input file {} please check file path and permissions.",
        args[1]
    );
    let mut input_file = File::open(input_file_name).expect(&message);
    let mut contents = String::new();
    input_file
        .read_to_string(&mut contents)
        .expect("Failed to read input file.");
    // Now the parser should be called with the contents of the file.
    let first_parse = parser::parse_lines(contents);
    let machine_code: Vec<String>;
    machine_code = generate_machine_code(first_parse);
    let output_file_name = input_file_name.replace(".vm", ".asm");
    let mut output_file = File::create(output_file_name).expect("Failed to open output file.");
    for code in machine_code {
        println!("{}", code);
        writeln!(output_file, "{}", code).expect("Failed to write to output file.");
    }
}
