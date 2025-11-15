pub mod registry;
pub mod definition;
pub mod help;


use registry::CommandRegistry;
use help::HelpCommand;
// Import other commands here as they are created

pub fn create_command_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();
    registry.register(HelpCommand);
    // Register other commands here
    registry
}