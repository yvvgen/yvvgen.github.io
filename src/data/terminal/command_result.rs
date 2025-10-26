use crate::utils::terminal::TerminalView;

#[derive(Clone)]
pub enum CommandResult {
    Output(Vec<String>),
    ChangeView(Box<dyn TerminalView>),
    Clear,
    Error(String),
}
