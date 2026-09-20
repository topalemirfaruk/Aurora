use super::pacman::{PacmanManager, SearchResult};
use crate::process::CommandExecutor;
use crate::security::{is_valid_package_name, sanitize_search_query, url_encode};
use crate::utils::SystemCapabilities;

use serde::Deserialize;

#[derive(Deserialize)]
struct AurRpcResponse {
    results: Option<Vec<AurRpcPackage>>,
}

#[derive(Deserialize)]
struct AurRpcPackage {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
}

pub struct AurManager;

impl AurManager {
    /// Sistemde uygun AUR yardımcısı olup olmadığını kontrol et
    pub fn helper() -> Option<&'static str> {
        SystemCapabilities::detect().preferred_aur_helper()
    }

    /// AUR üzerinde güvenli arama yap (Yerel yardımcı veya resmi AUR RPC v5)
    pub async fn search(query: &str) -> Result<Vec<SearchResult>, std::io::Error> {
        let clean = sanitize_search_query(query);
        if clean.is_empty() {
            return Ok(Vec::new());
        }

        if let Some(helper) = Self::helper() {
            if let Ok((true, stdout, _)) = CommandExecutor::run_captured(helper, &["-Ssa", "--", &clean]).await {
                if !stdout.is_empty() {
                    let results = PacmanManager::parse_search_output(&stdout, "aur");
                    if !results.is_empty() {
                        return Ok(results);
                    }
                }
            }
        }

        Self::search_rpc(&clean).await
    }

    async fn search_rpc(query: &str) -> Result<Vec<SearchResult>, std::io::Error> {
        let encoded_query = url_encode(query);
        let url = format!("https://aur.archlinux.org/rpc/v5/search/{}", encoded_query);
        let (success, stdout, _) = CommandExecutor::run_captured(
            "curl",
            &["-sSL", "--proto", "=https", "--tlsv1.2", "--max-time", "8", &url],
        ).await?;

        if !success || stdout.is_empty() {
            return Ok(Vec::new());
        }

        if let Ok(resp) = serde_json::from_str::<AurRpcResponse>(&stdout) {
            let items = resp.results.unwrap_or_default();
            let results = items
                .into_iter()
                .map(|item| SearchResult {
                    repo: "aur".to_string(),
                    name: item.name,
                    version: item.version.unwrap_or_default(),
                    description: item.description.unwrap_or_default(),
                    is_installed: false,
                })
                .collect();
            return Ok(results);
        }

        Ok(Vec::new())
    }

    /// PKGBUILD içeriğini güvenli biçimde çek
    pub async fn get_pkgbuild(package_name: &str) -> Result<String, std::io::Error> {
        if !is_valid_package_name(package_name) {
            return Ok("Geçersiz veya güvensiz paket adı.".to_string());
        }

        let helper = Self::helper().unwrap_or("curl");
        if helper == "paru" {
            let (success, stdout, _) = CommandExecutor::run_captured("paru", &["-Gp", "--", package_name]).await?;
            if success && !stdout.is_empty() {
                return Ok(stdout);
            }
        } else if helper == "yay" {
            let (success, stdout, _) = CommandExecutor::run_captured("yay", &["-Gp", "--", package_name]).await?;
            if success && !stdout.is_empty() {
                return Ok(stdout);
            }
        }

        // Güvenli curl: Zorunlu HTTPS (--proto =https), max 10 saniye timeout (--max-time 10), TLS 1.2+
        let url = format!("https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h={}", package_name);
        let (success, stdout, stderr) = CommandExecutor::run_captured(
            "curl",
            &["-sSL", "--proto", "=https", "--tlsv1.2", "--max-time", "10", &url],
        ).await?;

        if success && !stdout.is_empty() && !stdout.contains("404 Not Found") {
            Ok(stdout)
        } else {
            Ok(format!("PKGBUILD içeriği alınamadı: {}", stderr))
        }
    }
}
