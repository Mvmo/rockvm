#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs;
use std::fmt;
use std::any::{Any};

#[derive(Debug)]
enum Instruction {
    PushInt(i32),
    PushBool(bool),
    PushString(String),
    Jump(u64),
    ConditionalJump(u64, u64),
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

fn parse_lines(lines: &Vec<&str>) -> Vec<Instruction> {
    lines.iter()
        .map(|line| parse_line(line))
        .collect()
}

fn print_instructions(instructions: &Vec<Instruction>) {
    instructions.iter()
        .for_each(|instruction| println!("{}", instruction.to_string()))
}

type TypeName = &'static str;

trait RockType {
    fn new(name: String) -> Self;
    fn name(&self) -> &str;
}

struct PrimitiveType {
    name: String
}

impl RockType for PrimitiveType {
    fn new(name: String) -> Self {
        PrimitiveType { name }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

struct ComplexType {
    name: String,
    fields: Vec<Field>
}

impl RockType for ComplexType {
    fn new(name: String) -> Self {
        ComplexType { name: name, fields: Vec::new() }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

struct Field {
    name: String,
    type_name: TypeName
}

impl Field {
    fn new(name: String, type_name: TypeName) -> Self {
        Field { name, type_name }
    }
}

trait RockInstance<T: RockType> {
    fn _type(&self) -> &T;
}

struct PrimitiveInstance {
    _type: &'static PrimitiveType,
    value: dyn Any
}

/*const INT_PRIMITIVE: PrimitiveType = PrimitiveType::new("int".to_string());
const BOOL_PRIMITIVE: PrimitiveType = PrimitiveType::new("bool".to_string());
const STRING_PRIMITIVE: PrimitiveType = PrimitiveType::new("string".to_string());
*/

impl RockInstance<PrimitiveType> for PrimitiveInstance {
    fn _type(&self) -> &PrimitiveType {
        self._type 
    }
}

struct ComplexInstance {
    _type: &'static ComplexType,
    values: Vec<&'static dyn RockInstance<dyn Any>>
}

impl RockInstance<ComplexType> for ComplexInstance {
    fn _type(&self) -> &ComplexType {
        self._type
    }
}

fn evaluate_instructions(instructions: &Vec<Instruction>) {
    let mut index = 0;
    loop {
        if index >= instructions.len() {
            break
        }

        let instruction = instructions.get(index)
            .expect("Error while trying to access instruction");

        match &*instruction {
            Instruction::PushInt(i) => println!("push {}", i),
            Instruction::PushBool(b) => println!("push {}", b),
            Instruction::PushString(s) => println!("push {}", s),
            Instruction::Jump(where_to_jump) => {
                println!("jump {}", where_to_jump);
                index = *where_to_jump as usize
            }
            _ => {
                println!("none")
            }
        }

        index += 1
    }
}

fn main() {
    let file_name = String::from("examples/example.rock");
    let file_content = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let lines = file_content.lines()
        .filter(|line| !line.is_empty())
        .collect();

    let instructions = parse_lines(&lines);
    evaluate_instructions(&instructions);
}
