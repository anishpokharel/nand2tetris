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
use std::path::Path;

use crate::code_writer::generate_machine_code;

fn main() {
    // Read the file name from the arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: translator <file-name.vm> or <directory-path>");
        exiting_sequence(true);
        return;
    }
    let argument: &String = &args[1];
    let regex = Regex::new(r"\.[^/\\]+$").unwrap();

    if regex.is_match(&argument) {
        // Argument contains file with extension.
        if argument.ends_with(".vm") {
            println!("Single file detected.");
            process_single_file(argument);
        } else {
            if argument.ends_with("..") {
                println!("Directory detected. Scanning...");
                translate_directory(argument);
            } else {
                println!("The file extension should end in .vm");
                exiting_sequence(true);
                return;
            }
        }
    } else {
        println!("Directory detected. Scanning...");
        // Implement directory way of handling things.
        translate_directory(argument);
    }
}
/// Just prints the exit message. 
/// ```
/// is_force_exit: bool
/// ```
/// Where bool is indication if it's a forced exit because of errors. 
fn exiting_sequence(is_force_exit: bool) {
    if is_force_exit {
        println!("Please fix before re-running. \nExiting.. Bye.");
        panic!("Exiting due to error in sequence.");
    } else {
        println!("Thank you for using the translator.. Bye.");
    }
}

fn translate_directory(directory_url: &String) {
    // Scan the given directory for files ending in .vm
    let mut files: Vec<String> = Vec::new();

    match fs::read_dir(directory_url) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(et) => {
                        let file_name = et.file_name().into_string().unwrap_or_default();
                        if file_name.ends_with(".vm") {
                            println!("Found .vm file -> '{}'", file_name);
                            files.push(file_name);
                        }
                    }
                    Err(err) => {
                        println!("Error occured while reading entry error-> {}", err);
                        exiting_sequence(true);
                    }
                }
            }
        }
        Err(err) => {
            println!(
                "Error occured reading directory '{}' error -> {}",
                directory_url, err
            );
            exiting_sequence(true);
            return;
        }
    }

    if files.is_empty() {
        println!(
            "No .vm files detected in the given directory '{}'",
            directory_url
        );
        exiting_sequence(true);
        return;
    } else {
        // Process each .vm file following the same algo.
        let mut output_file_name: String = "WRONG_FILENAME".to_string();
        let mut combined_machine_code: Vec<String> = Vec::new();
        for file in files {
            let full_file_path;
            if directory_url.eq(".") {
                full_file_path = format!("{}", file);
                output_file_name = format!("{}.asm", get_output_file_name(directory_url));
            } else if directory_url.ends_with("..") {
                full_file_path = format!("{}{}{}", directory_url, "/", file);
                output_file_name = format!("..{}{}.asm", "/", get_output_file_name(directory_url));
            } else if directory_url.ends_with(".") {
                let mut formated_url = directory_url.as_str().to_string();
                formated_url.pop();
                full_file_path = format!("{}{}", formated_url, file);
                output_file_name = format!(
                    "{}{}.asm",
                    formated_url,
                    get_output_file_name(directory_url)
                );
            } else if !directory_url.ends_with("/") {
                full_file_path = format!("{}{}{}", directory_url, "/", file);
                output_file_name = format!("{}.asm", get_output_file_name(directory_url));
            } else {
                full_file_path = format!("{}{}", directory_url, file);
                output_file_name = format!(
                    "{}{}.asm",
                    directory_url,
                    get_output_file_name(directory_url)
                );
            }
            combined_machine_code.extend(handle_multiple_files(&full_file_path));
        }
        // A lot of fixings needed here.
        // This project is going to take a while, while I learn RUST.
        let mut output_file =
            File::create(output_file_name.clone()).expect("Failed to open output file.");
        for code in combined_machine_code {
            writeln!(output_file, "{}", code).expect("Failed to write to output file.");
        }
        println!("{} Processed...", output_file_name);
        exiting_sequence(false);
    }
}

fn handle_multiple_files(file_path: &str) -> Vec<String> {
    let message = format!(
        "Failed to open input file {} please check file path and permissions.",
        &file_path
    );

    let mut input_file = File::open(file_path).expect(&message);
    let mut contents = String::new();
    input_file
        .read_to_string(&mut contents)
        .expect("Failed to read input file.");
    let parsed_commands = parser::parse_lines(contents);
    let mut machine_code: Vec<String> = Vec::new();
    let file_comment = format!("// Following is conversion of {}", file_path);
    machine_code.push(file_comment);

    machine_code.extend(generate_machine_code(
        parsed_commands,
        file_path.replace(".vm", "").replace("/", ""),
    ));
    return machine_code;
}

fn process_single_file(file_name: &String) {
    println!("{} processing file.", file_name);
    let argument = file_name;

    let mut contents = String::new();
    match File::open(argument.clone()) {
        Ok(read_file) => {
            let mut input_file = read_file;
            input_file
                .read_to_string(&mut contents)
                // Here instead of using arguments.clone display the actual name of the file in question.
                .expect(format!("Failed to read contents... {}", argument.clone()).as_str());
            let parsed_commands = parser::parse_lines(contents);
            let machine_code: Vec<String>;
            machine_code = generate_machine_code(
                parsed_commands,
                argument.replace(".vm", "").replace("/", ""),
            );
            let output_file_name = argument.replace(".vm", ".asm");
            let mut output_file =
                File::create(output_file_name.clone()).expect("Failed to open output file.");
            for code in machine_code {
                writeln!(output_file, "{}", code).expect("Failed to write to output file.");
            }
            println!("{} processed.", output_file_name);
            exiting_sequence(false);
        }
        Err(err) => {
            println!("Error reading file. Does it exists? Error-> {}", err);
            exiting_sequence(true);
        }
    }
}

fn get_output_file_name(directory_to_process: &String) -> String {
    let parts: Vec<&str> = directory_to_process.split("/").collect();
    let current_directory = env::current_dir().unwrap();
    if directory_to_process.eq(".") {
        let dir_name = current_directory
            .file_name()
            .and_then(|closure| closure.to_str())
            .unwrap_or("");
        return dir_name.to_string();
    } else if directory_to_process.eq("..") {
        // Handle this situation as well.
        let parent_directory = current_directory.parent().unwrap_or(Path::new(""));
        let parent_name = parent_directory
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        return parent_name.to_string();
    } else if directory_to_process.ends_with(".") {
        return parts[parts.len() - 2].to_string();
    } else if directory_to_process.ends_with("/") {
        return parts[parts.len() - 2].to_string();
    } else {
        if parts[parts.len() - 1].eq("") {
            return format!("{}", parts[parts.len() - 2]);
        }
        return format!("{}/{}", directory_to_process, parts[parts.len() - 1]);
    }
}
