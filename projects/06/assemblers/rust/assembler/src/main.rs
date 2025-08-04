mod assembler;

use std::env;
use std::fs::File;
use std::io::prelude::*;
// Main function, the entry point of the Rust assembler.
// The assembler takes .asm files as input and produces .hack file as output.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: hack_assembler <input.asm>");
        std::process::exit(1);
    } else {
        let input_file = &args[1];
        let mut file = File::open(input_file)
            .expect("Failed to open input file, please check the file path and permissions");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read input file. Please ensure the file is not empty and is readable.");
        // Here you would typically read the input file, parse it, and generate the .hack output.
        // For now, we just print the input file name.
        println!("Input file: {}", contents);
        match assembler::assemble(&contents) {
            Ok(machine_code) => {
                // Write the machine code to a .hack file
                let output_file = input_file.replace(".asm", ".hack");
                let mut output = File::create(output_file)
                    .expect("Failed to create output file. Please check the file path and permissions.");
                for line in machine_code {
                    writeln!(output, "{}", line)
                        .expect("Failed to write to output file. Please ensure the file is writable.");
                }
                println!("Machine code written successfully.");
            }
            Err(e) => {
                eprintln!("Error during assembly: {}", e);
            }
        }

    }
}
