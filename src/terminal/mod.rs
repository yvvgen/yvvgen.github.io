pub mod component; // New main terminal component
pub mod screen;
pub mod state;
pub mod command_result;
pub mod commands;
pub mod views;
pub mod hooks;

#[derive(PartialEq)]
pub enum BootState {
    Booting,
    Ready,
}

pub enum CommandOutput {
    Text(String),
    Launch(String),
    None,
}