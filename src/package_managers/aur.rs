use super::pacman::SearchResult;
use crate::process::CommandExecutor;
use crate::security::{is_valid_package_name, sanitize_search_query};
use crate::utils::SystemCapabilities;

pub struct AurManager;

impl AurManager {
    /// Sistemde uygun AUR yardımcısı olup olmadığını kontrol et
    pub fn helper() -> Option<&'static str> {
        SystemCapabilities::detect().preferred_aur_helper()
    }

    /// AUR üzerinde güvenli arama yap
    pub async fn search(query: &str) -> Result<Vec<SearchResult>, std::io::Error> {
        let helper = match Self::helper() {
            Some(h) => h,
            None => return Ok(Vec::new()),
        };

        let clean = sanitize_search_query(query);
        if clean.is_empty() {
            return Ok(Vec::new());
        }

        // paru -Ssa -- <query> veya yay -Ssa -- <query>
        let (success, stdout, _) = CommandExecutor::run_captured(helper, &["-Ssa", "--", &clean]).await?;
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
                None => ("aur".to_string(), repo_and_name.to_string()),
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
