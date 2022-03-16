use std::fs;
use std::fmt;

#[derive(Debug)]
enum Instruction {
    PushInt(i32),
    PushBool(bool),
    PushString(String),
    Jump(u64, u64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    Void
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return fmt::Debug::fmt(self, f)
    }
}

fn parse_line(line: &str) -> Instruction {
    let mut splitted = line.split_ascii_whitespace();
    let instruction_name = String::from(splitted.next().expect("require instruction name"));

    return match instruction_name.as_str() {
        "pushi" => {
            let value = splitted.next().map(|value| value.parse::<i32>()).expect("require value param in pushi instruction").unwrap();
            Instruction::PushInt(value)
        },
        "pushb" => {
            let value = splitted.next().map(|value| value.parse::<bool>()).expect("require bool param in pushb instruction").unwrap();
            Instruction::PushBool(value)
        },
        "pushs" => {
            Instruction::PushString(
                splitted.collect::<Vec<&str>>()
                    .join(" ")
                    .strip_prefix("\"")
                    .expect("String needs to start with \"")
                    .strip_suffix("\"")
                    .expect("String needs to end with \"").to_string()
            )
        },
        "jmp" => {
            let if_true = splitted.next().map(|if_true| if_true.parse::<u64>()).expect("Require if_true param").unwrap();
            let if_false = splitted.next().map(|if_false| if_false.parse::<u64>()).expect("Require if_false param").unwrap();
            
            Instruction::Jump(if_true, if_false)
        },
        "add" => Instruction::Add,
        "sub" => Instruction::Subtract,
        "mul" => Instruction::Multiply,
        "div" => Instruction::Divide,
        "mod" => Instruction::Modulo,
        "eq" => Instruction::Equals,
        "neq" => Instruction::NotEquals,
        _ => Instruction::Void
    }
}

fn main() {
    let file_name = String::from("examples/example.rock");
    let file_content = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let lines = file_content.lines()
        .filter(|line| !line.is_empty());

    for line in lines {
        let instruction = parse_line(line);
        println!("{}", instruction.to_string())
    }
}
