use crate::utils::terminal::{CommandRegistry, TerminalView};
use std::rc::Rc;

#[derive(Clone)]
pub struct HistoryEntry {
    pub command: String,
    pub output: Vec<String>,
}

#[derive(Clone)]
pub struct TerminalState {
    pub current_view: Option<Box<dyn TerminalView>>,
    pub history: Vec<HistoryEntry>,
    pub command_buffer: String,
    pub registry: Rc<CommandRegistry>,
}

impl PartialEq for TerminalState {
    fn eq(&self, other: &Self) -> bool {
        self.current_view == other.current_view
            && self.history.len() == other.history.len()
            && self.command_buffer == other.command_buffer
    }
}
