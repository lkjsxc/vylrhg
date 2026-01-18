#[derive(Debug, Clone)]
pub enum Command {
    Help,
    Input(String),
}

pub fn parse_line(line: &str) -> Option<Command> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed == "help" || trimmed == "?" {
        return Some(Command::Help);
    }
    Some(Command::Input(trimmed.to_string()))
}

pub fn help_text() -> &'static str {
    "commands:\n  help | ?\n  markup:<markup>\n  asm:<program>\n  tab:new <title>"
}
