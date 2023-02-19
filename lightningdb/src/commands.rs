pub enum CommandType {
    AUTH,
    SAVE,
    STRINGSGET,
    STRINGSSET,
    STRINGSDEL,
    STRINGAPPEND,
    STRINGGETSET,
    STRINGGETDEL,
    STRINGSTRLEN,
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
            "GET" => Ok(CommandType::STRINGSGET),
            "SET" => Ok(CommandType::STRINGSSET),
            "DEL" => Ok(CommandType::STRINGSDEL),
            "SAVE" => Ok(CommandType::SAVE),
            "APPEND" => Ok(CommandType::STRINGAPPEND),
            "GETSET" => Ok(CommandType::STRINGGETSET),
            "GETDEL" => Ok(CommandType::STRINGGETDEL),
            "STRLEN" => Ok(CommandType::STRINGSTRLEN),
            "AUTH" => Ok(CommandType::AUTH),
            _ => Err("Unknown command"),
        }
    }
}
