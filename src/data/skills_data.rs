use serde::{Deserialize, Serialize};
use yew::macros::classes;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SkillCategory {
    ProgrammingLanguage,
    ToolsAndMethods,
    PythonLibrary,
    MachineLearningMath,
    SoftSkill,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub category: SkillCategory,
}

impl Skill {
    pub fn get_all() -> Vec<Self> {
        vec![
            // Programming Languages
            Skill {
                name: "Python".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "PySpark".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "Matlab".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "SQL".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "Bash".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "Fish".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "Java".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "C++".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "LaTeX".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "HTML/CSS".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "JavaScript".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "Go".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "Rust".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "React".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "Angular".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            Skill {
                name: "PHP".into(),
                category: SkillCategory::ProgrammingLanguage,
            },
            // Tools & Methods
            Skill {
                name: "Linux".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            Skill {
                name: "Git".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            Skill {
                name: "CI/CD".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            Skill {
                name: "Docker".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            Skill {
                name: "Podman".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            Skill {
                name: "Hadoop (Hive, Spark, HDFS)".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            Skill {
                name: "AWS (SageMaker, S3, Glue)".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            Skill {
                name: "Airflow".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            Skill {
                name: "Agile Development".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            Skill {
                name: "Data-Driven Development".into(),
                category: SkillCategory::ToolsAndMethods,
            },
            // Python Libraries
            Skill {
                name: "NumPy".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "Pandas".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "Matplotlib".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "Plotly".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "TensorFlow".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "Keras".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "PyTorch".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "Scikit-learn".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "Librosa".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "Django".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "Flask".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "FastAPI".into(),
                category: SkillCategory::PythonLibrary,
            },
            Skill {
                name: "Gunicorn".into(),
                category: SkillCategory::PythonLibrary,
            },
            // ML & Math
            Skill {
                name: "Classification".into(),
                category: SkillCategory::MachineLearningMath,
            },
            Skill {
                name: "Regression".into(),
                category: SkillCategory::MachineLearningMath,
            },
            Skill {
                name: "Clustering".into(),
                category: SkillCategory::MachineLearningMath,
            },
            Skill {
                name: "Tree Learning".into(),
                category: SkillCategory::MachineLearningMath,
            },
            Skill {
                name: "Neural Networks (Autoencoder, CNN, GNN)".into(),
                category: SkillCategory::MachineLearningMath,
            },
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
            Skill {
                name: "Signal Denoising".into(),
                category: SkillCategory::MachineLearningMath,
            },
            Skill {
                name: "Dictionary Learning".into(),
                category: SkillCategory::MachineLearningMath,
            },
            Skill {
                name: "Tokenization".into(),
                category: SkillCategory::MachineLearningMath,
            },
        ]
    }

    pub fn to_badge_html(&self) -> Html {
        let class = self.category.badge_class();
        let icon = self.category.icon();

        html! {
            <span class={classes!(class, "inline-flex", "items-center", "gap-1", "text-xs")}>
                { format!("{}::{}", icon, &self.name) }
            </span>
        }
    }

    pub fn list_to_badges(skills: &[Skill]) -> Html {
        html! {
            <div class="flex flex-wrap gap-2">{ for skills.iter().map(|s| s.to_badge_html()) }</div>
        }
    }
}

impl SkillCategory {
    /// DaisyUI badge class depending on type of skill
    pub fn badge_class(&self) -> &'static str {
        match self {
            SkillCategory::ProgrammingLanguage => {
                "badge badge-soft-primary badge-synthwave text-primary glow-primary"
            }
            SkillCategory::ToolsAndMethods => {
                "badge badge-soft-secondary badge-synthwave text-secondary glow-secondary"
            }
            SkillCategory::PythonLibrary => {
                "badge badge-soft-accent badge-synthwave text-accent glow-accent"
            }
            SkillCategory::MachineLearningMath => {
                "badge badge-soft-primary badge-synthwave text-primary glow-primary"
            }
            SkillCategory::SoftSkill => {
                "badge badge-soft-secondary badge-synthwave text-secondary glow-subtle"
            }
        }
    }

    /// Icon depending on skill category
    pub fn icon(&self) -> &'static str {
        match self {
            SkillCategory::ProgrammingLanguage => "λ",
            SkillCategory::ToolsAndMethods => "⚙",
            SkillCategory::PythonLibrary => "🐍",
            SkillCategory::MachineLearningMath => "∑",
            SkillCategory::SoftSkill => "★",
        }
    }
}
