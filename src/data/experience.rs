use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExperienceItem {
    pub title: String,
    pub company: String,
    pub date_range: String,
    pub responsibilities: Vec<String>,
    // Added tags for the badges
    pub tags: Vec<String>,
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
                title: "Fullstack AI Developper".to_string(),
                company: "Groupe AVT".to_string(),
                date_range: "Janvier 2025 à maintenant".to_string(),
                responsibilities: vec![
                    "Développement back d’une application de mise en relation apprentis/CFA/entreprises.".to_string(),
                    "Conception d’une partie intelligence : système de recommandation, génération automatique de CV.".to_string(),
                ],
                tags: vec!["PHP/Symfony".to_string(), "API Platform".to_string(), "Node.js".to_string(), "Docker".to_string(), "AI".to_string()],
                position: TimelinePosition::Right,
            },
            ExperienceItem {
                title: "Année Sabbatique".to_string(),
                company: "Apprentissage de la vie".to_string(),
                date_range: "2024".to_string(),
                responsibilities: vec![
                    "Apprentissage de techniques d'agroforesterie.".to_string(),
                    "Fabrication de fromage, etc.".to_string(),
                ],
                tags: vec!["Agroforesterie".to_string(), "Autonomie".to_string()],
                position: TimelinePosition::Left,
            },
            ExperienceItem {
                title: "Stagiaire Data Analyst".to_string(),
                company: "Toyota Motor Europe".to_string(),
                date_range: "Avril à Septembre 2023".to_string(),
                responsibilities: vec![
                    "Caractérisation des scènes de conduite à partir des données récupérées lors de trajets en voiture".to_string(),
                    "Evaluation statistique de la performance du régulateur de vitesse adaptatif".to_string(),
                ],
                tags: vec!["Python".to_string(), "Statistics".to_string(), "ADAS".to_string()],
                position: TimelinePosition::Right,
            },
            ExperienceItem {
                title: "Diplôme d'Ingénieur Généraliste".to_string(),
                company: "École Centrale de Nantes".to_string(),
                date_range: "2023".to_string(),
                responsibilities: vec![
                    "Spécialisation en mathématiques appliquées et en traitement de signal.".to_string(),
                ],
                tags: vec!["Maths".to_string(), "Signal Processing".to_string()],
                position: TimelinePosition::Left,
            },
            ExperienceItem {
                title: "Assistant Data Scientist".to_string(),
                company: "SNCF Connect & Tech".to_string(),
                date_range: "Avril à Août 2022".to_string(),
                responsibilities: vec![
                    "Migration et amélioration du modèle de recommandation de trajets".to_string(),
                    "Documentation de l’état de l’art (Deep Cross Network)".to_string(),
                ],
                tags: vec!["Data Science".to_string(), "Cloud".to_string(), "Recommender Systems".to_string()],
                position: TimelinePosition::Right,
            },
            ExperienceItem {
                title: "Stagiaire Data Scientist".to_string(),
                company: "Adeo".to_string(),
                date_range: "Mai à Août 2021".to_string(),
                responsibilities: vec![
                    "Développement et validation d’un POC de détection de fraude".to_string(),
                ],
                tags: vec!["Machine Learning".to_string(), "Gradient Boosting".to_string(), "Agile".to_string()],
                position: TimelinePosition::Right,
            },
            ExperienceItem {
                title: "Entrée à l'École Centrale de Nantes".to_string(),
                company: "École Centrale de Nantes".to_string(),
                date_range: "2019 - 2023".to_string(),
                responsibilities: vec![
                    "Spécialisation en mathématiques appliquées.".to_string(),
                ],
                tags: vec!["Engineering".to_string()],
                position: TimelinePosition::Left,
            },
            ExperienceItem {
                title: "Classes Préparatoires (MPSI-MP*)".to_string(),
                company: "Lycée Faidherbe, Lille".to_string(),
                date_range: "2016 - 2019".to_string(),
                responsibilities: vec![
                    "Option Informatique.".to_string(),
                ],
                tags: vec!["Maths".to_string(), "Physics".to_string(), "CS".to_string()],
                position: TimelinePosition::Left,
            },
        ]
    }
}
