use crate::data::skills_data::{Skill, SkillCategory};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TimelinePosition {
    Left,
    Right,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExperienceItem {
    pub title: String,
    pub company: String,
    pub date_range: String,
    pub responsibilities: Vec<String>,
    pub skills: Vec<Skill>,
    pub position: TimelinePosition,
}

impl ExperienceItem {
    pub fn get_all() -> Vec<Self> {
        vec![

            ExperienceItem {
                title: "Fullstack AI Developer".into(),
                company: "Groupe AVT".into(),
                date_range: "Janvier 2025 – Maintenant".into(),
                responsibilities: vec![
                    "Développement back d’une application de mise en relation apprentis/CFA/entreprises.".into(),
                    "Conception d’une partie intelligence : recommandation, génération automatique de CV/LM.".into(),
                ],
                skills: vec![
                    Skill { name: "PHP/Symfony".into(), category: SkillCategory::ProgrammingLanguage },
                    Skill { name: "FastAPI".into(), category: SkillCategory::PythonLibrary },
                    Skill { name: "PostreSQL".into(), category: SkillCategory::ToolsAndMethods },
                ],
                position: TimelinePosition::Right,
            },
            ExperienceItem {
                title: "Voyages".into(),
                company: "Vie".into(),
                date_range: "2024".into(),
                responsibilities: vec![
                    "Spécialisation fabrication et dégustation de fromage".into(),
                ],
                skills: vec![
                ],
                position: TimelinePosition::Left,
            },
            ExperienceItem {
                title: "Diplôme d'Ingénieur Généraliste".into(),
                company: "École Centrale de Nantes".into(),
                date_range: "2023".into(),
                responsibilities: vec![
                    "Spécialisation en mathématiques appliquées et traitement du signal.".into(),
                ],
                skills: vec![
                    Skill { name: "Maths".into(), category: SkillCategory::MachineLearningMath },
                    Skill { name: "Signal Processing".into(), category: SkillCategory::MachineLearningMath },
                ],
                position: TimelinePosition::Left,
            },
            ExperienceItem {
                title: "Stagiaire Data Analyst".into(),
                company: "Toyota Motor Europe".into(),
                date_range: "Avril – Septembre 2023".into(),
                responsibilities: vec![
                    "Caractérisation des scènes de conduite à partir de données embarquées.".into(),
                    "Évaluation statistique du régulateur de vitesse adaptatif (ADAS).".into(),
                ],
                skills: vec![
                    Skill { name: "Python".into(), category: SkillCategory::ProgrammingLanguage },
                    Skill { name: "Statistics".into(), category: SkillCategory::MachineLearningMath },
                    Skill { name: "ADAS".into(), category: SkillCategory::MachineLearningMath },
                ],
                position: TimelinePosition::Right,
            },

            ExperienceItem {
                title: "Assistant Data Scientist".into(),
                company: "SNCF Connect & Tech".into(),
                date_range: "Avril – Août 2022".into(),
                responsibilities: vec![
                    "Migration et amélioration du modèle de recommandation de trajets.".into(),
                    "Documentation de l’état de l’art : Deep Cross Network, Factorization Machine.".into(),
                    "Calculs statistiques à la demande (abandon d’option, taux de remplissage…).".into(),
                    "Participation au Hackathon : alerte de réservation intelligente.".into(),
                ],
                skills: vec![
                    Skill { name: "PySpark".into(), category: SkillCategory::ProgrammingLanguage },
                    Skill { name: "AWS".into(), category: SkillCategory::ToolsAndMethods },
                    Skill { name: "Machine Learning".into(), category: SkillCategory::MachineLearningMath },
                ],
                position: TimelinePosition::Right,
            },

            ExperienceItem {
                title: "Stagiaire Data Scientist".into(),
                company: "Adeo".into(),
                date_range: "Mai – Août 2021".into(),
                responsibilities: vec![
                    "Développement et validation d’un POC de détection de fraude en caisse.".into(),
                    "Utilisation d’algorithmes supervisés (Gradient Boosting) et calibrage des seuils statistiques.".into(),
                ],
                skills: vec![
                    Skill { name: "Gradient Boosting".into(), category: SkillCategory::MachineLearningMath },
                    Skill { name: "Python".into(), category: SkillCategory::ProgrammingLanguage },
                    Skill { name: "Agile".into(), category: SkillCategory::ToolsAndMethods },
                ],
                position: TimelinePosition::Right,
            },

            ExperienceItem {
                title: "Entrée École Ingénieur Généraliste".into(),
                company: "École Centrale de Nantes".into(),
                date_range: "2019".into(),
                responsibilities: vec![
                    "Spécialisation en mathématiques appliquées et traitement du signal.".into(),
                ],
                skills: vec![
                    Skill { name: "Maths".into(), category: SkillCategory::MachineLearningMath },
                    Skill { name: "Signal Processing".into(), category: SkillCategory::MachineLearningMath },
                ],
                position: TimelinePosition::Left,
            },

            ExperienceItem {
                title: "Classes Préparatoires (MPSI–MP*)".into(),
                company: "Lycée Faidherbe, Lille".into(),
                date_range: "2016 – 2019".into(),
                responsibilities: vec![
                    "Option Informatique, filière scientifique renforcée.".into(),
                ],
                skills: vec![
                    Skill { name: "Maths".into(), category: SkillCategory::MachineLearningMath },
                    Skill { name: "Physics".into(), category: SkillCategory::MachineLearningMath },
                    Skill { name: "CS".into(), category: SkillCategory::ProgrammingLanguage },
                ],
                position: TimelinePosition::Left,
            },

        ]
    }
}
