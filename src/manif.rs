use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project
{
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BuildMode
{
    BatchBuild,
    SingleFile,
}

impl Default for BuildMode
{
    fn default() -> Self { BuildMode::BatchBuild }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OutputFormat
{
    PDF,
    DVI,
}

impl std::str::FromStr for OutputFormat
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "pdf" => Ok(OutputFormat::PDF),
            "dvi" => Ok(OutputFormat::DVI),
            _ => Err(format!("{} is not a valid output format", s)),
        }
    }
}

impl Default for OutputFormat
{
    fn default() -> Self { OutputFormat::PDF }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BuildCmd(String);

impl TryFrom<BuildCmd> for BuildMode
{
    type Error = Box<dyn std::error::Error>;

    fn try_from(bcmd: BuildCmd) -> Result<Self, Self::Error>
    {
        match bcmd.0.as_str()
        {
            "batch" => Ok(BuildMode::BatchBuild),
            "single" => Ok(BuildMode::SingleFile),
            _ => Err("Invalid build mode".into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Defaults
{
    pub target: String,
    #[serde(rename = "artifacts-dir")]
    pub artifacts_dir: String,
    #[serde(rename = "target-dir")]
    pub target_dir: String,
    pub build: Option<String>,
    pub main: Option<String>,
    #[serde(rename = "src-dir")]
    pub src_dir: Option<String>,
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
    pub fn get_build(&self) -> Result<BuildMode, ()>
    {
        let build = self.defaults.build.clone();
        if build.is_none()
        {
            return Ok(BuildMode::default());
        }
        else
        {
            let bcmd = BuildCmd(build.unwrap());
            let bmode = BuildMode::try_from(bcmd);
            if bmode.is_err()
            {
                return Err(());
            }
            else
            {
                return Ok(bmode.unwrap());
            }
        }
    }
    pub fn get_main(&self) -> Option<String> { return self.defaults.main.clone(); }
    pub fn get_src(&self) -> Option<String> { return self.defaults.src_dir.clone(); }
    pub fn check(&self) -> bool
    {
        let mut ok = true;
        if self.get_build().is_err()
        {
            ok = false;
        }
        else if self.get_main().is_some()
        {
            let main = self.defaults.main.clone().unwrap();
            if !std::path::Path::new(&main).exists()
            {
                ok = false;
            }
        }
        let _ = match self.get_src()
        {
            Some(src) =>
            {
                if !std::path::Path::new(&src).exists()
                {
                    ok = false;
                }
            }
            None =>
            {
                if !std::path::Path::new("src").exists()
                {
                    ok = false;
                }
            }
        };
        return ok;
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
                build: Some("batch".to_string()),
                main: None,
                src_dir: None,
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
