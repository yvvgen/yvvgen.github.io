use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExperienceItem {
    pub title: String,
    pub company: String,
    pub date_range: String,
    pub responsibilities: Vec<String>,
    pub position: TimelinePosition,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TimelinePosition {
    Left,
    Right,
}

impl ExperienceItem {
    pub fn get_all() -> Vec<Self> {
        vec![
            ExperienceItem {
                title: "Senior Frontend Engineer".to_string(),
                company: "TechCorp".to_string(),
                date_range: "Jan 2022 – Present".to_string(),
                responsibilities: vec![
                    "Led development of a scalable e-commerce platform using Rust and WebAssembly."
                        .to_string(),
                    "Optimized application performance by 30% through efficient state management."
                        .to_string(),
                    "Mentored junior developers and conducted code reviews.".to_string(),
                ],
                position: TimelinePosition::Left,
            },
            ExperienceItem {
                title: "Software Developer".to_string(),
                company: "StartUp Inc.".to_string(),
                date_range: "Jun 2019 – Dec 2021".to_string(),
                responsibilities: vec![
                    "Developed and maintained RESTful APIs using Node.js and Express.".to_string(),
                    "Implemented responsive UI components with React and Tailwind CSS.".to_string(),
                    "Collaborated with cross-functional teams to deliver features on time."
                        .to_string(),
                ],
                position: TimelinePosition::Right,
            },
            ExperienceItem {
                title: "Junior Web Developer".to_string(),
                company: "Digital Solutions Agency".to_string(),
                date_range: "Jul 2017 – May 2019".to_string(),
                responsibilities: vec![
                    "Built custom WordPress themes and plugins for various clients.".to_string(),
                    "Ensured cross-browser compatibility and accessibility compliance.".to_string(),
                    "Assisted in database design and optimization.".to_string(),
                ],
                position: TimelinePosition::Left,
            },
        ]
    }
}
