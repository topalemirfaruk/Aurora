use crate::models::PackageSource;
use crate::process::CommandExecutor;
use crate::security::{is_valid_package_name, sanitize_search_query};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub source: PackageSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageUpdate {
    pub name: String,
    pub old_version: String,
    pub new_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub is_installed: bool,
}

pub struct PacmanManager;

impl PacmanManager {
    /// Sistemde kurulu tüm paketleri (Adı ve Sürümü) listele
    pub async fn list_installed() -> Result<Vec<InstalledPackage>, std::io::Error> {
        let (success, stdout, _) = CommandExecutor::run_captured("pacman", &["-Q"]).await?;
        if !success {
            return Ok(Vec::new());
        }

        let mut list = Vec::new();
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                list.push(InstalledPackage {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    source: PackageSource::Official,
                });
            }
        }
        Ok(list)
    }

    /// Belirli bir paketin kurulu olup olmadığını kontrol et
    pub async fn is_installed(package_name: &str) -> bool {
        if !is_valid_package_name(package_name) {
            return false;
        }

        let (success, _, _) = CommandExecutor::run_captured("pacman", &["-Q", "--", package_name]).await.unwrap_or((false, String::new(), String::new()));
        success
    }

    /// Resmi depolarda paket ara (`pacman -Ss <query>`)
    pub async fn search(query: &str) -> Result<Vec<SearchResult>, std::io::Error> {
        let clean = sanitize_search_query(query);
        if clean.is_empty() {
            return Ok(Vec::new());
        }

        let (success, stdout, _) = CommandExecutor::run_captured("pacman", &["-Ss", "--", &clean]).await?;
        if !success {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();
        let mut lines = stdout.lines();

        while let Some(header_line) = lines.next() {
            let header_parts: Vec<&str> = header_line.split_whitespace().collect();
            if header_parts.is_empty() {
                continue;
            }

            let repo_and_name = header_parts[0];
            let version = header_parts.get(1).unwrap_or(&"").to_string();
            let is_installed = header_line.contains("[installed]");

            let (repo, name) = match repo_and_name.split_once('/') {
                Some((r, n)) => (r.to_string(), n.to_string()),
                None => ("repo".to_string(), repo_and_name.to_string()),
            };

            let description = lines.next().unwrap_or("").trim().to_string();

            results.push(SearchResult {
                repo,
                name,
                version,
                description,
                is_installed,
            });
        }

        Ok(results)
    }

    /// Paket bilgilerini al (`pacman -Si <package>`)
    pub async fn get_info(package_name: &str) -> Result<String, std::io::Error> {
        if !is_valid_package_name(package_name) {
            return Ok("Geçersiz paket adı.".to_string());
        }

        let (success, stdout, stderr) = CommandExecutor::run_captured("pacman", &["-Si", "--", package_name]).await?;
        if success {
            Ok(stdout)
        } else {
            Ok(format!("Paket bilgisi bulunamadı: {}", stderr))
        }
    }

    pub async fn check_updates() -> Result<Vec<PackageUpdate>, std::io::Error> {
        let has_checkupdates = which::which("checkupdates").is_ok();
        let (success, stdout, _) = if has_checkupdates {
            CommandExecutor::run_captured("checkupdates", &["--nocolor"]).await?
        } else {
            CommandExecutor::run_captured("pacman", &["-Qu"]).await?
        };

        if !success || stdout.is_empty() {
            return Ok(Vec::new());
        }

        let mut updates = Vec::new();
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[2] == "->" {
                updates.push(PackageUpdate {
                    name: parts[0].to_string(),
                    old_version: parts[1].to_string(),
                    new_version: parts[3].to_string(),
                });
            } else if parts.len() >= 2 {
                let new_ver = if parts.len() >= 3 {
                    parts[2].to_string()
                } else {
                    parts[1].to_string()
                };
                updates.push(PackageUpdate {
                    name: parts[0].to_string(),
                    old_version: parts[1].to_string(),
                    new_version: new_ver,
                });
            }
        }
        Ok(updates)
    }
}
