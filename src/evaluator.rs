use crate::instruction::{Instruction};

pub fn evaluate_instructions(instructions: &Vec<Instruction>) {
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