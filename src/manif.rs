use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project
{
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Defaults
{
    pub target: String,
    #[serde(rename = "artifacts-dir")]
    pub artifacts_dir: String,
    #[serde(rename = "target-dir")]
    pub target_dir: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Manifest
{
    pub project: Project,
    pub defaults: Defaults,
}

impl Manifest
{
    pub fn new(file: &str) -> Self
    {
        let fullmanif: Manifest = toml::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
        return fullmanif;
    }
    pub fn write_out_template() -> Result<(), Box<dyn std::error::Error>>
    {
        let manif = toml::to_string_pretty(&Manifest {
            project: Project {
                name: "curseforge".to_string(),
                version: "0.1.0".to_string(),
                authors: vec!["Your Name".to_string()],
            },
            defaults: Defaults {
                target: "pdf".to_string(),
                artifacts_dir: "artifacts".to_string(),
                target_dir: "build".to_string(),
            },
        })?;
        std::fs::write("Curse.toml", manif)?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn read_test_manifest()
    {
        let manif = Manifest::new("Curse.toml");
        assert_eq!(manif.project.name, "curseforge");
        assert_eq!(manif.project.version, "0.1.0");
        assert_eq!(manif.defaults.target, "pdf");
    }
}
