use crate::process::CommandExecutor;
use crate::security::{is_critical_system_package, is_valid_package_name};

#[derive(Debug, Clone)]
pub struct SystemHealthInfo {
    pub is_running: String,
    pub cache_size: String,
    pub orphaned_packages: Vec<String>,
    pub failed_services: Vec<String>,
    pub has_paccache: bool,
}

pub struct MaintenanceService;

impl MaintenanceService {
    pub async fn check_health() -> SystemHealthInfo {
        let has_paccache = which::which("paccache").is_ok();

        let is_running = match CommandExecutor::run_captured("systemctl", &["is-system-running"]).await {
            Ok((_, stdout, _)) => {
                let trimmed = stdout.trim();
                if trimmed.is_empty() {
                    "Bilinmiyor".to_string()
                } else {
                    trimmed.to_string()
                }
            }
            Err(_) => "Erişilemedi".to_string(),
        };

        let cache_size = match CommandExecutor::run_captured("du", &["-sh", "/var/cache/pacman/pkg"]).await {
            Ok((true, stdout, _)) => stdout
                .split_whitespace()
                .next()
                .unwrap_or("0 B")
                .to_string(),
            _ => "Hesaplanamadı".to_string(),
        };

        let orphaned_packages = match CommandExecutor::run_captured("pacman", &["-Qtdq"]).await {
            Ok((true, stdout, _)) => stdout
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|s| !s.is_empty() && is_valid_package_name(s))
                .collect(),
            _ => Vec::new(),
        };

        let failed_services = match CommandExecutor::run_captured("systemctl", &["--failed", "--no-legend", "--plain"]).await {
            Ok((true, stdout, _)) => stdout
                .lines()
                .filter_map(|l| {
                    let parts: Vec<&str> = l.split_whitespace().collect();
                    parts.first().map(|s| s.to_string())
                })
                .collect(),
            _ => Vec::new(),
        };

        SystemHealthInfo {
            is_running,
            cache_size,
            orphaned_packages,
            failed_services,
            has_paccache,
        }
    }

    pub fn build_cache_clean_command(uninstalled_only: bool) -> (String, Vec<String>) {
        if which::which("paccache").is_ok() {
            if uninstalled_only {
                ("paccache".to_string(), vec!["-ruk0".to_string()])
            } else {
                ("paccache".to_string(), vec!["-r".to_string()])
            }
        } else {
            ("sudo".to_string(), vec!["pacman".to_string(), "-Sc".to_string(), "--noconfirm".to_string()])
        }
    }

    pub fn build_orphan_clean_command(orphans: &[String]) -> Option<(String, Vec<String>)> {
        let safe_orphans: Vec<String> = orphans
            .iter()
            .filter(|pkg| is_valid_package_name(pkg) && !is_critical_system_package(pkg))
            .cloned()
            .collect();

        if safe_orphans.is_empty() {
            return None;
        }

        let mut args = vec!["pacman".to_string(), "-Rns".to_string(), "--noconfirm".to_string(), "--".to_string()];
        args.extend(safe_orphans);
        Some(("sudo".to_string(), args))
    }

    pub fn build_service_restart_command(service_name: &str) -> Option<(String, Vec<String>)> {
        let trimmed = service_name.trim();
        if trimmed.is_empty() {
            return None;
        }

        let pkg_part = trimmed.trim_end_matches(".service");
        if !is_valid_package_name(pkg_part) {
            return None;
        }

        Some(("sudo".to_string(), vec!["systemctl".to_string(), "restart".to_string(), "--".to_string(), trimmed.to_string()]))
    }
}
