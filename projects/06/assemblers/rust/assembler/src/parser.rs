#[derive(Debug, Clone)]
pub enum Instruction {
    AInstruction(String),
    CInstruction {
        dest: Option<String>,
        comp: String,
        jump: Option<String>,
    },
    Label(String),
}

pub fn parse_lines(source: &str) -> Vec<Instruction>{

}