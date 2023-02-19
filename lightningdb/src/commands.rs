pub enum CommandType {
    GET,
    SET,
    DEL,
    SAVE,
}

pub struct Command {
    pub command: CommandType,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(full_command: &str) -> Result<Command, &'static str> {
        let mut parts = full_command.trim().splitn(2, ' ');
        let command_str = parts.next().ok_or("Empty command string")?;
        let command = Command::parse_command(command_str)?;
        let args = parts
            .next()
            .map(|s| s.split_whitespace().map(|w| w.to_string()).collect())
            .unwrap_or(Vec::new());
        Ok(Command { command, args })
    }

    fn parse_command(command: &str) -> Result<CommandType, &'static str> {
        match command.trim().to_uppercase().as_str() {
            "GET" => Ok(CommandType::GET),
            "SET" => Ok(CommandType::SET),
            "DEL" => Ok(CommandType::DEL),
            "SAVE" => Ok(CommandType::SAVE),
            _ => Err("Unknown command"),
        }
    }
}
