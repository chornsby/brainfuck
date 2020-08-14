use std::convert::TryFrom;

/// One of the eight commands of the Brainfuck language
#[derive(Debug, PartialEq)]
pub enum Command {
    StepForward,
    StepBackward,
    Increment,
    Decrement,
    PrintOutput,
    ReadInput,
    BeginLoop,
    EndLoop,
}

impl TryFrom<u8> for Command {
    type Error = &'static str;

    /// Tries to convert a byte into a known Command
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'>' => Ok(Self::StepForward),
            b'<' => Ok(Self::StepBackward),
            b'+' => Ok(Self::Increment),
            b'-' => Ok(Self::Decrement),
            b'.' => Ok(Self::PrintOutput),
            b',' => Ok(Self::ReadInput),
            b'[' => Ok(Self::BeginLoop),
            b']' => Ok(Self::EndLoop),
            _ => Err("Unknown command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bytes_to_known_commands() {
        let command_bytes = b"><+-.,[]";

        let expected = vec![
            Command::StepForward,
            Command::StepBackward,
            Command::Increment,
            Command::Decrement,
            Command::PrintOutput,
            Command::ReadInput,
            Command::BeginLoop,
            Command::EndLoop,
        ];
        let actual: Vec<Command> = command_bytes
            .iter()
            .flat_map(|&b| Command::try_from(b).ok())
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn ignore_non_command_bytes() {
        let command_bytes = b"><+-.,[]";

        for byte in 0..=255 {
            if command_bytes.contains(&byte) {
                continue;
            }

            assert!(
                Command::try_from(byte).is_err(),
                "non command byte {} ({}) was not ignored",
                byte as char,
                byte,
            );
        }
    }
}
