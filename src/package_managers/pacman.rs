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
    /// Sistemde kurulu tüm paketleri (Adı, Sürümü ve Kaynağı) listele
    pub async fn list_installed() -> Result<Vec<InstalledPackage>, std::io::Error> {
        let foreign_pkgs: std::collections::HashSet<String> = match CommandExecutor::run_captured("pacman", &["-Qm"]).await {
            Ok((true, stdout, _)) => stdout
                .lines()
                .filter_map(|l| l.split_whitespace().next().map(|s| s.to_string()))
                .collect(),
            _ => std::collections::HashSet::new(),
        };

        let (success, stdout, _) = CommandExecutor::run_captured("pacman", &["-Q"]).await?;
        if !success {
            return Ok(Vec::new());
        }

        let mut list = Vec::new();
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let is_foreign = foreign_pkgs.contains(&name);
                list.push(InstalledPackage {
                    name,
                    version: parts[1].to_string(),
                    source: if is_foreign {
                        PackageSource::Aur
                    } else {
                        PackageSource::Official
                    },
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

    /// checkupdates veya pacman -Qu çıktısını PackageUpdate vektörüne ayrıştırır
    pub fn parse_updates_output(stdout: &str) -> Vec<PackageUpdate> {
        let mut updates = Vec::new();
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(arrow_idx) = parts.iter().position(|&p| p == "->") {
                if arrow_idx > 0 && arrow_idx + 1 < parts.len() {
                    let old_ver = parts[arrow_idx - 1].to_string();
                    let new_ver = parts[arrow_idx + 1].to_string();
                    // Gerçek bir güncelleme olması için eski ve yeni sürüm birbirinden farklı olmalıdır
                    if old_ver != new_ver {
                        updates.push(PackageUpdate {
                            name: parts[0].to_string(),
                            old_version: old_ver,
                            new_version: new_ver,
                        });
                    }
                }
            }
        }
        updates
    }

    pub async fn check_updates() -> Result<Vec<PackageUpdate>, std::io::Error> {
        let has_checkupdates = which::which("checkupdates").is_ok();
        let (has_updates, stdout) = if has_checkupdates {
            let (status, stdout, stderr) =
                CommandExecutor::run_captured_with_status("checkupdates", &["--nocolor"]).await?;
            // checkupdates:
            // Standart Arch: 0 = güncelleme yok, 2 = güncellemeler mevcut, 1 = hata.
            // Bazı dağıtımlarda (örn. CachyOS / yamalı betikler): 0 veya 2 dönebilir.
            let has_upd = match status.code() {
                Some(0) | Some(2) => !stdout.trim().is_empty(),
                Some(code) => {
                    tracing::warn!("checkupdates çıkış kodu {}: {}", code, stderr.trim());
                    !stdout.trim().is_empty()
                }
                None => !stdout.trim().is_empty(),
            };
            (has_upd, stdout)
        } else {
            let (status, stdout, stderr) =
                CommandExecutor::run_captured_with_status("pacman", &["-Qu"]).await?;
            // pacman -Qu: 0 = güncellemeler mevcut, 1 = güncelleme yok
            let has_upd = match status.code() {
                Some(0) => !stdout.trim().is_empty(),
                Some(1) => false,
                Some(code) => {
                    tracing::warn!("pacman -Qu çıkış kodu {}: {}", code, stderr.trim());
                    !stdout.trim().is_empty()
                }
                None => !stdout.trim().is_empty(),
            };
            (has_upd, stdout)
        };

        if !has_updates || stdout.is_empty() {
            return Ok(Vec::new());
        }

        Ok(Self::parse_updates_output(&stdout))
    }
}
