// --- Component Module Declarations ---

// Declare sub-modules for each component file
pub mod navigation;
pub mod hero;
pub mod about;
pub mod projects;
pub mod skills;
pub mod contact;

// Export the public component functions for easy access
pub use navigation::Navigation;
pub use hero::Hero;
pub use about::About;
pub use projects::Projects;
pub use skills::Skills;
pub use contact::Contact;
