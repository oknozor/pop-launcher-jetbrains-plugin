use crate::config::trusted_project::Application;
use crate::jetbrains::Ide;
use anyhow::Result;
use fork::{daemon, Fork};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use jetbrains::IdeConfigPath;
use pop_launcher_toolkit::launcher::{IconSource, PluginResponse, PluginSearchResult};
use pop_launcher_toolkit::plugin_trait::tracing::error;
use pop_launcher_toolkit::plugin_trait::{async_trait, PluginExt};
use std::borrow::Cow;
use std::io;
use std::os::unix::prelude::CommandExt;
use std::path::PathBuf;
use std::process::Command;

mod config;
mod jetbrains;

pub struct JetBrainsProjects {
    projects: Vec<ProjectEntry>,
    fuzzy_matcher: SkimMatcherV2,
}

#[derive(Debug)]
struct ProjectEntry {
    path: PathBuf,
    name: String,
    ide: Ide,
}

impl ProjectEntry {
    fn exec(&self) -> io::Error {
        error!("CMD {:?} {:?}", self.ide.bin(), self.path);

        let mut cmd = Command::new(self.ide.bin());
        cmd.arg(self.path.to_string_lossy().as_ref());

        #[cfg(feature = "sway")]
        cmd.env("_JAVA_AWT_WM_NONREPARENTING", "1");

        cmd.exec()
    }
}

impl JetBrainsProjects {
    fn sort_match(&mut self, query: &str) {
        self.projects.sort_by(|a, b| {
            let score_a = self.fuzzy_matcher.fuzzy_match(&a.name, query).unwrap_or(-1);
            let score_b = self.fuzzy_matcher.fuzzy_match(&b.name, query).unwrap_or(-1);

            score_b.cmp(&score_a)
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
                    let icon = entry.ide.icon().to_string();
                    self.respond_with(PluginResponse::Append(PluginSearchResult {
                        id: id as u32,
                        name: entry.name.clone(),
                        description: entry.path.to_string_lossy().to_string(),
                        keywords: None,
                        icon: Some(IconSource::Name(Cow::Owned(icon))),
                        exec: None,
                        window: None,
                    }))
                    .await;
                }
                self.respond_with(PluginResponse::Finished).await;
            }

            None | Some(_) => self.respond_with(PluginResponse::Finished).await,
        }
    }

    async fn activate(&mut self, id: u32) {
        self.respond_with(PluginResponse::Close).await;
        match self.projects.get(id as usize) {
            None => {
                error!("Entry not found at index {id}");
            }
            Some(project) => {
                error!("Activating {:?}", project);
                if let Ok(Fork::Child) = daemon(true, false) {
                    let _ = project.exec();
                }

                std::process::exit(0);
            }
        }
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

    let projects: Vec<ProjectEntry> = configs
        .into_iter()
        .filter_map(|ide| ide.get_entries().ok())
        .flatten()
        .collect();

    let mut plugin = JetBrainsProjects {
        projects,
        fuzzy_matcher: SkimMatcherV2::default().ignore_case().use_cache(true),
    };

    plugin.run().await;

    Ok(())
}
