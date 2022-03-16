#![allow(unused_imports)]
#![allow(dead_code)]

mod types;
mod parser;
mod instruction;
mod evaluator;

use std::fs;
use std::fmt;
use std::any::{Any};

use crate::types::*;
use crate::parser::*;
use crate::instruction::*;
use crate::evaluator::*;

fn print_instructions(instructions: &Vec<Instruction>) {
    instructions.iter()
        .for_each(|instruction| println!("{}", instruction.to_string()))
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
