# Types
str: String -> Starts with " and ends with "

int: Integer -> Number

bool: Boolean -> TRUE | FALSE

# Instructions

## PUSHI
Push integer on top of stack
> pushi \<int>

## PUSHB
Push boolean on top of stack
> pushb \<bool>

## PUSHS
Push string on top of stack
> pushs \<str>

## JMP
Jump to specific instruction
> jmp \<int>

## CJMP
> Stack Requirement: \<bool>

Jumps to specific instruction if \<bool> on stack is true else it jumps to another specified instruction
> cjmp \<int> \<int>
