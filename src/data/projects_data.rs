use crate::data::experience_data::TimelinePosition;
use crate::data::skills_data::{Skill, SkillCategory};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ProjectItem {
    pub title: String,
    pub date_range: String,
    pub description: Vec<String>,
    pub skills: Vec<Skill>,
    pub position: TimelinePosition,
}

impl ProjectItem {
    pub fn get_all() -> Vec<Self> {
        vec![
            ProjectItem {
                title: "FFT-based Audio Visualizer".into(),
                date_range: "2024 – 2025".into(),
                description: vec![
                    "Analyse audio temps réel avec FFT.".into(),
                    "Contrôle d’un ruban LED piloté via ESP32 en Rust.".into(),
                ],
                skills: vec![
                    Skill {
                        name: "Rust".into(),
                        category: SkillCategory::ProgrammingLanguage,
                    },
                    Skill {
                        name: "FFT".into(),
                        category: SkillCategory::MachineLearningMath,
                    },
                    Skill {
                        name: "Signal Processing".into(),
                        category: SkillCategory::MachineLearningMath,
                    },
                ],
                position: TimelinePosition::Right,
            },
            ProjectItem {
                title: "Système de Recommandation Académique".into(),
                date_range: "2023".into(),
                description: vec![
                    "Exploration des Factorization Machines et Deep Cross Networks.".into(),
                    "Implémentation d’un mini système de recommandation pédagogique.".into(),
                ],
                skills: vec![
                    Skill {
                        name: "Python".into(),
                        category: SkillCategory::ProgrammingLanguage,
                    },
                    Skill {
                        name: "Recommender Systems".into(),
                        category: SkillCategory::MachineLearningMath,
                    },
                ],
                position: TimelinePosition::Left,
            },
            ProjectItem {
                title: "Projet traitement d'image : Dénombrement et classification".into(),
                date_range: "2022".into(),
                description: vec![
                    "Dénombrement + Dénombrement via autoencodeur/ondelettes.".into(),
                    "Classification d’images avec CNN.".into(),
                ],
                skills: vec![
                    Skill {
                        name: "CNN".into(),
                        category: SkillCategory::MachineLearningMath,
                    },
                    Skill {
                        name: "Image Processing".into(),
                        category: SkillCategory::MachineLearningMath,
                    },
                    Skill {
                        name: "Python".into(),
                        category: SkillCategory::ProgrammingLanguage,
                    },
                ],
                position: TimelinePosition::Right,
            },
            ProjectItem {
                title: "Optimisation & Modèles Bayésiens".into(),
                date_range: "2021".into(),
                description: vec![
                    "Application de chaînes de Markov à un problème d’inférence.".into(),
                    "Étude mêlant optimisation convexe et non convexe.".into(),
                ],
                skills: vec![
                    Skill {
                        name: "Bayesian Statistics".into(),
                        category: SkillCategory::MachineLearningMath,
                    },
                    Skill {
                        name: "Markov Chains".into(),
                        category: SkillCategory::MachineLearningMath,
                    },
                    Skill {
                        name: "Optimization".into(),
                        category: SkillCategory::MachineLearningMath,
                    },
                ],
                position: TimelinePosition::Left,
            },
        ]
    }
}
