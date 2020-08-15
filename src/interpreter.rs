use crate::command::Command;
use crate::program::Program;

/// A Brainfuck program interpreter with its own mutable memory
pub struct Interpreter {
    memory: [u8; 30_000],
    memory_pointer: usize,
}

impl Interpreter {
    /// Initialises the interpreter with the classic 30 000 cells of memory
    pub fn new() -> Self {
        Self {
            memory: [0; 30_000],
            memory_pointer: 0,
        }
    }

    /// Runs the given valid Brainfuck program to completion
    pub fn run(&mut self, program: &Program) {
        let mut command_pointer = 0;

        while command_pointer < program.commands.len() {
            match program.commands[command_pointer] {
                Command::StepForward => self.memory_pointer += 1,
                Command::StepBackward => self.memory_pointer -= 1,
                Command::Increment => self.memory[self.memory_pointer] += 1,
                Command::Decrement => self.memory[self.memory_pointer] -= 1,
                Command::PrintOutput => print!("{}", self.memory[self.memory_pointer] as char),
                Command::ReadInput => todo!(),
                Command::BeginLoop => {
                    if self.memory[self.memory_pointer] == 0 {
                        command_pointer = program.loop_map[&command_pointer];
                    }
                }
                Command::EndLoop => {
                    if self.memory[self.memory_pointer] != 0 {
                        command_pointer = program.loop_map[&command_pointer];
                    }
                }
            }

            command_pointer += 1;
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn run_valid_program() {
        let input = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]";
        let program = Program::try_from(input.to_vec().as_slice()).unwrap();

        let mut interpreter = Interpreter::new();
        interpreter.run(&program);

        assert_eq!(interpreter.memory_pointer, 0);
        assert_eq!(interpreter.memory[0], 0);
        assert_eq!(interpreter.memory[1], 0);
        assert_eq!(interpreter.memory[2], 72);
        assert_eq!(interpreter.memory[3], 104);
        assert_eq!(interpreter.memory[4], 88);
        assert_eq!(interpreter.memory[5], 32);
        assert_eq!(interpreter.memory[6], 8);
        assert_eq!(interpreter.memory[7], 0);
        assert_eq!(interpreter.memory[8], 0);
        assert_eq!(interpreter.memory[9], 0);
    }
}
