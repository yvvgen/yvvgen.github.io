use crate::components::help::HelpView;
use crate::data::terminal::CommandResult;
use crate::utils::terminal::Command;

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
