use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Edit { path: PathBuf, force: bool },
    Write { path: Option<PathBuf> },
    Quit { force: bool },
    WriteQuit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandParseError {
    Empty,
    MissingPath,
    Unknown,
}

pub fn parse_command(input: &str) -> Result<Command, CommandParseError> {
    let s = input.trim();
    if s.is_empty() {
        return Err(CommandParseError::Empty);
    }

    let mut parts = s.split_whitespace();
    let head = parts.next().ok_or(CommandParseError::Empty)?;

    match head {
        "wq" => Ok(Command::WriteQuit),
        "q" => Ok(Command::Quit { force: false }),
        "q!" => Ok(Command::Quit { force: true }),
        "w" => {
            let path = parts.next().map(PathBuf::from);
            Ok(Command::Write { path })
        }
        "e" | "e!" => {
            let force = head == "e!";
            let path = parts.next().ok_or(CommandParseError::MissingPath)?;
            Ok(Command::Edit {
                path: PathBuf::from(path),
                force,
            })
        }
        _ => Err(CommandParseError::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic() {
        assert_eq!(parse_command("w").unwrap(), Command::Write { path: None });
        assert_eq!(
            parse_command("w a.txt").unwrap(),
            Command::Write {
                path: Some(PathBuf::from("a.txt"))
            }
        );
        assert_eq!(
            parse_command("e! a.txt").unwrap(),
            Command::Edit {
                path: PathBuf::from("a.txt"),
                force: true
            }
        );
        assert_eq!(parse_command("q!").unwrap(), Command::Quit { force: true });
        assert_eq!(parse_command("wq").unwrap(), Command::WriteQuit);
    }
}
