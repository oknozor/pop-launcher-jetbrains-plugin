use std::borrow::Cow;
use std::cmp::Ordering;
use std::path::PathBuf;
use crate::config::trusted_project::{Application, Project};
use anyhow::Result;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use pop_launcher_toolkit::launcher::{IconSource, Indice, PluginResponse, PluginSearchResult};
use pop_launcher_toolkit::plugin_trait::{PluginExt, async_trait};
use jetbrains::IdeConfigPath;

mod config;
mod jetbrains;

pub struct JetBrainsProjects {
    projects: Vec<ProjectEntry>,
    fuzzy_matcher: SkimMatcherV2,
}

impl JetBrainsProjects {
    fn sort_match(&mut self, query: &str) {
        self.projects
            .sort_by(|a, b| {
                let a = self.fuzzy_matcher.fuzzy_match(&a.name, query);
                let b = self.fuzzy_matcher.fuzzy_match(&b.name, query);
                if let (Some(a), Some(b)) = (a, b) {
                    a.cmp(&b)
                } else {
                    Ordering::Equal
                }
            });
    }
}

#[async_trait]
impl PluginExt for JetBrainsProjects {
    fn name(&self) -> &str {
        "jetbrains"
    }

    async fn search(&mut self, query: &str) {
        match query.split_once(' ') {
            Some(("idea", query)) => {
                self.sort_match(query);
                for (id, entry) in self.projects.iter().enumerate().take(8) {
                    self.respond_with(PluginResponse::Append(PluginSearchResult {
                        id: id as u32,
                        name: entry.name.clone(),
                        description: entry.path.to_string_lossy().to_string(),
                        keywords: None,
                        icon: Some(IconSource::Name(Cow::Owned(entry.icon.to_string()))),
                        exec: None,
                        window: None,
                    })).await;
                };
                self.respond_with(PluginResponse::Finished).await;
            }

            None | Some(_) => self.respond_with(PluginResponse::Finished).await,
        }
    }

    async fn activate(&mut self, id: Indice) {
        todo!()
    }
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let conf = dirs::config_dir()
        .expect("Failed to get config dir")
        .join("JetBrains");

    let mut configs = vec![];

    for entry in conf.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if let Ok(config) = IdeConfigPath::try_from(path) {
            configs.push(config);
        }
    }

    configs.sort_by(|a, b| b.cmp(a));
    configs.dedup_by(|a, b| a < b);

    let projects: Vec<ProjectEntry> = configs.into_iter()
        .filter_map(|ide| ide.get_entries().ok())
        .flatten()
        .collect();

    let mut plugin = JetBrainsProjects {
        projects,
        fuzzy_matcher: SkimMatcherV2::default(),
    };

    plugin.run().await;


    Ok(())
}

#[derive(Debug)]
struct ProjectEntry {
    path: PathBuf,
    name: String,
    icon: String,
}