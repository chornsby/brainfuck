mod command;
mod interpreter;
mod program;

use std::convert::TryFrom;

use interpreter::Interpreter;
use program::Program;

fn main() {
    let path = std::env::args().nth(1).expect("Missing filename");
    let input = std::fs::read_to_string(path).expect("Missing input");
    let program = Program::try_from(input.as_bytes()).expect("Malformed program");

    Interpreter::new().run(&program);
}
