use std::collections::HashMap;
use std::convert::TryFrom;

use crate::command::Command;

/// A list of bytes parsed into a Brainfuck program
///
/// To simplify looping logic in the interpreter we are caching the positions
/// of all BeginLoop and EndLoop pairs.
pub struct Program {
    pub commands: Vec<Command>,
    pub loop_map: HashMap<usize, usize>,
}

impl TryFrom<&[u8]> for Program {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut commands: Vec<Command> = vec![];
        let mut loop_stack: Vec<usize> = vec![];
        let mut loop_map: HashMap<usize, usize> = Default::default();

        for (index, command) in value
            .iter()
            .flat_map(|&byte| Command::try_from(byte).ok())
            .enumerate()
        {
            match command {
                Command::BeginLoop => loop_stack.push(index),
                Command::EndLoop => {
                    let start = loop_stack.pop().ok_or("Unmatched ]")?;
                    let end = index;

                    loop_map.insert(start, end);
                    loop_map.insert(end, start);
                }
                _ => {}
            }

            commands.push(command);
        }

        if loop_stack.is_empty() {
            Ok(Self { commands, loop_map })
        } else {
            Err("Unmatched [")
        }
    }
}
