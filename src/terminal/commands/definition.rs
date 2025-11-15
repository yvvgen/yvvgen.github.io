use crate::terminal::command_result::CommandResult;
use crate::terminal::state::TerminalState;
use yew::prelude::*;

/// Trait for all terminal commands
pub trait Command {
    fn name(&self) -> &str;
    fn aliases(&self) -> Vec<&str> {
        vec![]
    }
    fn description(&self) -> &str;
    fn execute(&self, args: &[&str]) -> CommandResult;
}

/// Trait for terminal views (full-screen components)
pub trait TerminalView {
    fn id(&self) -> &str;
    fn render(&self, state: UseStateHandle<TerminalState>) -> Html;
    fn clone_box(&self) -> Box<dyn TerminalView>;
}

impl Clone for Box<dyn TerminalView> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn TerminalView> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
