use crate::instruction::*;

fn parse_line(line: &str) -> Instruction {
    let mut splitted = line.split_ascii_whitespace();
    let instruction_name = String::from(splitted.next().expect("require instruction name"));

    return match instruction_name.as_str() {
        "pushi" => {
            Instruction::PushInt(
                splitted.next()
                    .map(|value| value.parse::<i32>())
                    .expect("require value param in pushi instruction")
                    .unwrap()
            )
        },
        "pushb" => {
            Instruction::PushBool(
                splitted.next()
                    .map(|value| value.parse::<bool>())
                    .expect("require bool param in pushb instruction")
                    .unwrap()
            )
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
            Instruction::Jump(
                splitted.next()
                    .map(|where_to_jump| where_to_jump.parse::<u64>())
                    .expect("Require where_to_jump param")
                    .unwrap()
            )
        },
        "cjmp" => {
            let if_true = splitted.next()
                .map(|if_true| if_true.parse::<u64>())
                .expect("Require if_true param")
                .unwrap();
            let if_false = splitted.next()
                .map(|if_false| if_false.parse::<u64>())
                .expect("Require if_false param")
                .unwrap();
        
            Instruction::ConditionalJump(if_true, if_false)
        }
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

pub fn parse_lines(lines: &Vec<&str>) -> Vec<Instruction> {
    lines.iter()
        .map(|line| parse_line(line))
        .collect()
}