use crate::process::CommandExecutor;
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
}
