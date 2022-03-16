use std::fmt;

#[derive(Debug)]
pub enum Instruction {
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

