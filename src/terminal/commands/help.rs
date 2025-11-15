use crate::terminal::views::help::HelpView;
use crate::terminal::command_result::CommandResult;
use crate::terminal::commands::definition::Command;

pub struct HelpCommand;

impl Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Show available commands"
    }

    fn execute(&self, _args: &[&str]) -> CommandResult {
        CommandResult::ChangeView(Box::new(HelpView))
    }
}
