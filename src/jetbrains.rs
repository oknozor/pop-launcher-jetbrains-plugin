use std::path::PathBuf;
use std::cmp::Ordering;
use anyhow::anyhow;
use crate::{Application, ProjectEntry};

#[derive(Debug, PartialEq)]
pub struct IdeConfigPath {
    path: PathBuf,
    ide: Ide,
    version: u32,
}

impl IdeConfigPath {
    fn new(path: PathBuf, ide: Ide, version: u32) -> Self {
        Self {
            path,
            ide,
            version,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Ide {
    IntelliJ,
    Clion,
    Rider,
    WebStorm,
    PhpStorm,
    Datagrip,
}

impl Ide {
    fn as_str(&self) -> &str {
        match self {
            Ide::IntelliJ => "IntelliJ",
            Ide::Clion => "CLion",
            Ide::Rider => "Rider",
            Ide::WebStorm => "WebStorm",
            Ide::PhpStorm => "PhpStorm",
            Ide::Datagrip => "Datagrip"
        }
    }

    fn icon(&self) -> &str {
        match self {
            Ide::IntelliJ => "intellij-idea",
            Ide::Clion => "clion",
            Ide::Rider => "rider",
            Ide::WebStorm => "webstorm",
            Ide::PhpStorm => "phpstorm",
            Ide::Datagrip => "datagrip",
        }
    }
}

impl IdeConfigPath {
    pub(crate) fn get_entries(self) -> anyhow::Result<Vec<ProjectEntry>> {
        let trusted_projects = self.path.join("options/trusted-paths.xml");
        if trusted_projects.exists() {
            let trusted_projects = std::fs::read_to_string(trusted_projects)?;
            let trusted_projects: Application = serde_xml_rs::from_str(&trusted_projects)?;
            let home = dirs::home_dir().expect("$HOME not found");

            let projects = trusted_projects.component.option.map.entries
                .into_iter()
                .map(|project| {
                    let path = project.key.replace("$USER_HOME$", home.to_string_lossy().as_ref());
                    let path = PathBuf::from(path);
                    let name = path.file_name().expect("No filename")
                        .to_string_lossy()
                        .to_string();
                    let icon = self.ide.icon().to_string();

                    ProjectEntry {
                        path,
                        name,
                        icon,
                    }
                }).collect();

            Ok(projects)
        } else {
            Err(anyhow!("trusted-paths.xml not found for {:?}", self.ide.as_str()))
        }
    }
}

impl TryFrom<PathBuf> for IdeConfigPath {
    type Error = &'static str;

    fn try_from(path: PathBuf) -> anyhow::Result<Self, Self::Error> {
        if path.is_dir() {
            let filename = path.file_name()
                .expect("Ide config dir should have a filename")
                .to_string_lossy();

            let config_path = if filename.starts_with("CLion") {
                let ide = Ide::Clion;
                ide.parse_version(&path)
                    .map(|version| IdeConfigPath::new(path, ide, version))
            } else if filename.starts_with("IntelliJ") {
                let ide = Ide::IntelliJ;
                ide.parse_version(&path)
                    .map(|version| IdeConfigPath::new(path, ide, version))
            } else if filename.starts_with("Rider") {
                let ide = Ide::Rider;
                ide.parse_version(&path)
                    .map(|version| IdeConfigPath::new(path, ide, version))
            } else if filename.starts_with("WebStorm") {
                let ide = Ide::WebStorm;
                ide.parse_version(&path)
                    .map(|version| IdeConfigPath::new(path, ide, version))
            } else if filename.starts_with("PhpStorm") {
                let ide = Ide::PhpStorm;
                ide.parse_version(&path)
                    .map(|version| IdeConfigPath::new(path, ide, version))
            } else if filename.starts_with("Datagrip") {
                let ide = Ide::Datagrip;
                ide.parse_version(&path)
                    .map(|version| IdeConfigPath::new(path, ide, version))
            } else {
                None
            };

            config_path.ok_or("Invalid config")
        } else {
            Err("Not a directory")
        }
    }
}


impl Eq for IdeConfigPath {}

impl Ord for IdeConfigPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd<Self> for IdeConfigPath {
    fn partial_cmp(&self, other: &IdeConfigPath) -> Option<Ordering> {
        if self.ide == other.ide {
            Some(self.version.cmp(&other.version))
        } else {
            self.ide.partial_cmp(&other.ide)
        }
    }
}

impl Ide {
    fn parse_version(&self, path: &PathBuf) -> Option<u32> {
        let filename = path.file_name().expect("Should have a filename")
            .to_string_lossy();

        let version = filename.strip_prefix(self.as_str())
            .expect("Got the wrong ide kind")
            .replace('.', "");

        version.parse::<u32>().ok()
    }
}
