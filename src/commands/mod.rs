mod built_in;

pub use built_in::*;

use crate::utils::terminal::CommandRegistry;

/// Initialize and return a registry with all commands
pub fn create_command_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    // Register built-in commands
    registry.register(help::HelpCommand);
    // registry.register(clear::ClearCommand);
    // registry.register(exit::ExitCommand);
    // registry.register(ls::LsCommand);
    // registry.register(echo::EchoCommand);
    //
    // // Register custom commands
    // registry.register(about::AboutCommand);
    // registry.register(nca::NCACommand);

    registry
}
