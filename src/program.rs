use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

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
