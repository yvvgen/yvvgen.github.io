// Best practice: Define a struct for structured data
pub struct ProfileData {
    pub name: &'static str,
    pub interest: &'static str,
}

// Define the constant profile data
pub const PROFILE: ProfileData = ProfileData {
    name: "yvvgen",
    interest: "luddism",
};