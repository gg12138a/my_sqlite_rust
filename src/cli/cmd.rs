//! user command input from cli. such as [.exit], [.tables].
pub enum CliCommand {
    Exit,
}

pub fn parse_cmd(input: &str) -> Option<CliCommand> {
    match input {
        ".exit" => Some(CliCommand::Exit),
        _ => None,
    }
}
