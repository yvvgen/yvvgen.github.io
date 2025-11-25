use serde::{Deserialize, Serialize};

pub struct ProfileData {
    pub name: &'static str,
    pub interest: &'static str,
}

// Define the constant profile data
pub const PROFILE: ProfileData = ProfileData {
    name: "yvvgen",
    interest: "luddism",
};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub identity: Identity,
    pub contact: ContactInfo,
    pub languages: Vec<LanguageSkill>,
    pub education: Vec<EducationItem>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub title: String,
    pub location: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ContactInfo {
    pub phone: String,
    pub email: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct LanguageSkill {
    pub language: String,
    pub level: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct EducationItem {
    pub institution: String,
    pub degree: String,
    pub date_range: String,
}

impl Profile {
    pub fn get() -> Self {
        Self {
            identity: Identity {
                name: "Nessim Kerkeni".into(),
                title: "Ingénieur Data — Data Science & Data Engineering".into(),
                location: "Nantes".into(),
            },
            contact: ContactInfo {
                phone: "+33 6 81 39 08 13".into(),
                email: "kerkeni.nessim@gmail.com".into(),
            },
            languages: vec![
                LanguageSkill {
                    language: "Anglais".into(),
                    level: "Full professional proficiency (TOEIC 955)".into(),
                },
                LanguageSkill {
                    language: "Japonais".into(),
                    level: "Niveau N5".into(),
                },
            ],
            education: vec![
                EducationItem {
                    institution: "École Centrale de Nantes".into(),
                    degree:
                        "Ingénieur généraliste, spécialisation mathématiques & traitement du signal"
                            .into(),
                    date_range: "2019–2023".into(),
                },
                EducationItem {
                    institution: "Lycée Faidherbe (Lille)".into(),
                    degree: "CPGE MPSI–MP* option Informatique".into(),
                    date_range: "2016–2019".into(),
                },
            ],
        }
    }
}

