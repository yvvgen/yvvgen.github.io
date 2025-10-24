pub struct Skill {
    pub name: &'static str,
    pub level: &'static str,
}

pub const SKILLS: &[Skill] = &[
    Skill {
        name: "Rust",
        level: "Advanced",
    },
    Skill {
        name: "TypeScript",
        level: "Advanced",
    },
    Skill {
        name: "React",
        level: "Intermediate",
    },
];
