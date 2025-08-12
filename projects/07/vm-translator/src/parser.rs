use crate::code_writer::{self, VmCommand};

pub fn parse_lines(source: String) -> Vec<VmCommand> {
    // read the source files.
    // Strip away comments & empty lines.
    let mut categorized_commands= Vec::new();
    for line in source.lines() {
        let stripped = strip_comment(&line);
        if stripped.is_empty() {
            continue;
        } else {
           categorized_commands.push(code_writer::categorize_commands(stripped));
        }
    }

    return categorized_commands;
}

fn strip_comment(line: &str) -> &str {
    match line.find("//") {
        Some(index) => &line[..index],
        None => line,
    }
}
