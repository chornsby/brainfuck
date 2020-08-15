use std::collections::HashMap;
use std::convert::TryFrom;

use crate::command::Command;

/// A list of bytes parsed into a Brainfuck program
///
/// To simplify looping logic in the interpreter we are caching the positions
/// of all BeginLoop and EndLoop pairs.
#[derive(Debug)]
pub struct Program {
    pub commands: Vec<Command>,
    pub loop_map: HashMap<usize, usize>,
}

impl TryFrom<&[u8]> for Program {
    type Error = &'static str;

    /// Tries to parse all commands and cache loop jump indices for a slice
    /// of bytes
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_program() {
        let input = b"[->+<]";
        let result = Program::try_from(input.iter().as_slice());
        let program = result.unwrap();

        assert_eq!(
            program.commands,
            vec![
                Command::BeginLoop,
                Command::Decrement,
                Command::StepForward,
                Command::Increment,
                Command::StepBackward,
                Command::EndLoop
            ]
        );
        assert_eq!(program.loop_map.len(), 2);
        assert_eq!(program.loop_map[&0], 5);
        assert_eq!(program.loop_map[&5], 0);
    }

    #[test]
    fn missing_end_loop() {
        let input = b"[";
        let result = Program::try_from(input.iter().as_slice());

        assert_eq!(result.unwrap_err(), "Unmatched [");
    }

    #[test]
    fn missing_begin_loop() {
        let input = b"]";
        let result = Program::try_from(input.iter().as_slice());

        assert_eq!(result.unwrap_err(), "Unmatched ]");
    }

    #[test]
    fn invalid_loop_nesting() {
        let input = b"[]][";
        let result = Program::try_from(input.iter().as_slice());

        assert_eq!(result.unwrap_err(), "Unmatched ]");
    }
}
