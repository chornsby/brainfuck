use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use crate::command::Command;

mod command;

/// A list of bytes parsed into a Brainfuck program
///
/// To simplify looping logic in the interpreter we are caching the positions
/// of all BeginLoop and EndLoop pairs.
pub struct Program {
    commands: Vec<Command>,
    loop_map: HashMap<usize, usize>,
}

impl TryFrom<&[u8]> for Program {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let commands: Vec<Command> = value
            .iter()
            .filter_map(|&byte| byte.try_into().ok())
            .collect();

        let mut loop_map: HashMap<usize, usize> = HashMap::new();
        let mut stack: Vec<usize> = vec![];

        for (index, command) in commands.iter().enumerate() {
            match command {
                Command::BeginLoop => stack.push(index),
                Command::EndLoop => {
                    let start = stack.pop().ok_or("Unmatched ]")?;
                    let end = index;

                    loop_map.insert(start, end);
                    loop_map.insert(end, start);
                }
                _ => {}
            }
        }

        if stack.is_empty() {
            Ok(Self { commands, loop_map })
        } else {
            Err("Unmatched [")
        }
    }
}

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
