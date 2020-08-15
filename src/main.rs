use brainfuck::program::Program;
use brainfuck::Interpreter;
use std::convert::TryFrom;

fn main() {
    let path = std::env::args().nth(1).expect("Missing filename");
    let input = std::fs::read_to_string(path).expect("Missing input");
    let program = Program::try_from(input.as_bytes()).expect("Malformed program");

    Interpreter::new().run(&program);
}
