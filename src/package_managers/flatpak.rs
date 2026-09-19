use super::pacman::SearchResult;
use crate::process::CommandExecutor;
use crate::security::sanitize_search_query;
use crate::utils::SystemCapabilities;

#[derive(Debug, Clone)]
pub struct FlatpakApp {
    pub name: String,
    pub app_id: String,
    pub version: String,
}

pub struct FlatpakManager;

impl FlatpakManager {
    pub fn is_available() -> bool {
        SystemCapabilities::detect().has_flatpak
    }

    pub async fn list_installed() -> Result<Vec<FlatpakApp>, std::io::Error> {
        if !Self::is_available() {
            return Ok(Vec::new());
        }

        let (success, stdout, _) = CommandExecutor::run_captured("flatpak", &["list", "--app", "--columns=name,application,version"]).await?;
        if !success {
            return Ok(Vec::new());
        }

        let mut list = Vec::new();
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                list.push(FlatpakApp {
                    name: parts[0].trim().to_string(),
                    app_id: parts[1].trim().to_string(),
                    version: parts.get(2).unwrap_or(&"").trim().to_string(),
                });
            }
        }
        Ok(list)
    }

    pub async fn search(query: &str) -> Result<Vec<SearchResult>, std::io::Error> {
        if !Self::is_available() {
            return Ok(Vec::new());
        }

        let clean = sanitize_search_query(query);
        if clean.is_empty() {
            return Ok(Vec::new());
        }

        let (success, stdout, _) = CommandExecutor::run_captured(
            "flatpak",
            &["search", "--columns=application:f,name:f,description:f,version:f,remotes:f", &clean],
        ).await?;

        if !success {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                let app_id = parts[0].trim().to_string();
                let name = parts[1].trim().to_string();
                let description = parts[2].trim().to_string();
                let version = parts.get(3).unwrap_or(&"").trim().to_string();
                let repo = parts.get(4).unwrap_or(&"flathub").trim().to_string();

                if !app_id.is_empty() {
                    results.push(SearchResult {
                        repo: if repo.is_empty() { "flathub".to_string() } else { repo },
                        name: app_id,
                        version,
                        description: if !name.is_empty() {
                            format!("{} — {}", name, description)
                        } else {
                            description
                        },
                        is_installed: false,
                    });
                }
            }
        }
        Ok(results)
    }
}
