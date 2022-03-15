use std::fs;
use std::fmt;

#[derive(Debug)]
enum Instruction {
    JMP(u64, u64),
    VOID
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return fmt::Debug::fmt(self, f)
    }
}

fn parse_line(line: String) -> Instruction {
    let mut splitted = line.split_ascii_whitespace();
    let instruction_name = String::from(splitted.next().expect("require instruction name"));

    return match instruction_name.as_str() {
        "jmp" => {
            let if_true = splitted.next().map(|if_true| if_true.parse::<u64>()).expect("Require if_true param").unwrap();
            let if_false = splitted.next().map(|if_false| if_false.parse::<u64>()).expect("Require if_false param").unwrap();
            
            Instruction::JMP(if_true, if_false)
        },
        _ => Instruction::VOID
    }
}

fn main() {
    let file_name = String::from("examples/example.rock");
    let file_content = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let lines = file_content.lines()
        .filter(|line| !line.is_empty());

    for line in lines {
        let instruction = parse_line(String::from(line));
        println!("{}", instruction.to_string())
    }
}
