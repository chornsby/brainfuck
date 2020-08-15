use crate::command::Command;
use crate::program::Program;

pub struct Interpreter {
    memory: [u8; 30_000],
    memory_pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: [0; 30_000],
            memory_pointer: 0,
        }
    }

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
