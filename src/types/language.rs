use std::fmt;
use std::path::Path;
use std::str::FromStr;

// TODO: add comment pattern to reduce duplication in mutation exclusions?

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    FunC,
    Tact,
}

impl Language {
    pub fn all() -> Vec<Language> {
        vec![Language::FunC, Language::Tact]
    }

    pub fn from_path(path: &Path) -> Result<Self, String> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| String::from("missing file extension"))?;
        Language::from_str(extension)
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FunC => write!(f, "func"),
            Self::Tact => write!(f, "tact"),
        }
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "func" | "fc" => Ok(Self::FunC),
            "tact" => Ok(Self::Tact),
            _ => Err(format!("Unsupported language: {s}")),
        }
    }
}
