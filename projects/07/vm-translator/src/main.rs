/* VM Translator is basically a program that reads the VM commands,
  one command at a time, and translates each command into Hack Instructions.
*/
mod code_writer;
mod parser;

use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

use crate::code_writer::generate_machine_code;

fn main() {
    // Read the file name from the arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: translator <file-name.vm> or <directory>");
        return;
    }
    let argument: &String = &args[1];
    let regex = Regex::new(r"\.[^/\\]+$").unwrap();

    if regex.is_match(&argument) {
        // Argument contains file with extension.
        if argument.contains(".vm") {
            process_single_file(argument);
        } else {
            println!("The file extension should end in .vm");
        }
    } else {
        println!("Directory detected. Scanning...");
        // Implement directory way of handling things.
        translate_directory(argument);
    }
}

fn translate_directory(directory_url: &String) {
    // Scan the given directory for files ending in .vm
    let mut files: Vec<String> = Vec::new();

    for entry in fs::read_dir(directory_url).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap_or_default();
        if file_name.ends_with(".vm") {
            println!("Found .vm file -> {}", file_name);
            files.push(file_name);
        }
    }
    if files.is_empty() {
        println!(
            "No .vm files detected in the given directory {}",
            directory_url
        );
        return;
    } else {
        // Process each .vm file following the same algo.
        for file in files {
            let full_file_path;
            if directory_url.eq(".") {
                full_file_path = format!("{}", file);
            } else if directory_url.ends_with(".") {
                let mut formated_url = directory_url.as_str().to_string();
                formated_url.pop();
                full_file_path = format!("{}{}", formated_url, file);
            } else if !directory_url.ends_with("/") {
                full_file_path = format!("{}{}{}", directory_url, "/", file);
            } else {
                full_file_path = format!("{}{}", directory_url, file);
            }
            process_single_file(&full_file_path);
        }
    }
}

fn process_single_file(file_name: &String) {
    println!("{} processing file.", file_name);
    let argument = file_name;
    // single file selected
    let message = format!(
        "Failed to open input file {} please check file path and permissions.",
        argument.clone()
    );
    let mut input_file = File::open(argument.clone()).expect(&message);
    let mut contents = String::new();

    input_file
        .read_to_string(&mut contents)
        .expect("Failed to read input file.");
    // Now the parser should be called with the contents of the file.
    let first_parse = parser::parse_lines(contents);
    let machine_code: Vec<String>;
    machine_code = generate_machine_code(first_parse, argument.replace(".vm", "").replace("/", ""));
    let output_file_name = argument.replace(".vm", ".asm");
    let mut output_file =
        File::create(output_file_name.clone()).expect("Failed to open output file.");
    for code in machine_code {
        writeln!(output_file, "{}", code).expect("Failed to write to output file.");
    }
    println!("{} processed.", output_file_name)
}
