use crate::terminal::command_result::CommandResult;
use crate::terminal::commands::definition::Command;
use std::collections::HashMap;
use std::rc::Rc;

pub struct CommandRegistry {
    commands: HashMap<String, Rc<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register<C: Command + 'static>(&mut self, command: C) {
        let command = Rc::new(command);
        self.commands
            .insert(command.name().to_string(), command.clone());

        for alias in command.aliases() {
            self.commands.insert(alias.to_string(), command.clone());
        }
    }

    pub fn execute(&self, cmd_line: &str) -> CommandResult {
        let parts: Vec<&str> = cmd_line.trim().split_whitespace().collect();

        if parts.is_empty() {
            return CommandResult::Output(vec![]);
        }

        let cmd_name = parts[0];
        let args = &parts[1..];

        match self.commands.get(cmd_name) {
            Some(command) => command.execute(args),
            None => CommandResult::Error(format!("Command not found: {}", cmd_name)),
        }
    }

    pub fn list_commands(&self) -> Vec<Rc<dyn Command>> {
        let mut commands: Vec<_> = self
            .commands
            .values()
            .filter(|cmd| {
                cmd.name()
                    == self
                        .commands
                        .get(cmd.name())
                        .map(|c| c.name())
                        .unwrap_or("")
            })
            .cloned()
            .collect();

        commands.sort_by_key(|cmd| cmd.name().to_string());
        commands
    }
}
