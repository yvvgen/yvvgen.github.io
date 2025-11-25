// --- Component Module Declarations ---

// Declare sub-modules for each component file
pub mod contact;
pub mod experience;
pub mod hero;
pub mod navigation;
pub mod projects;
pub mod skills;
pub mod about;
pub mod loading;

// Export the public component functions for easy access
pub use contact::Contact;
pub use experience::Experience;
pub use hero::Hero;
pub use navigation::Navigation;
pub use projects::Projects;
pub use skills::Skills;
pub use about::About;
