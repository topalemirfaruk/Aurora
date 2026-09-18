use super::app::PackageSource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstalledStatus {
    Installed { version: String },
    NotInstalled,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDetail {
    pub package_name: String,
    pub source: PackageSource,
    pub version: String,
    pub description: String,
    pub download_size: Option<String>,
    pub installed_size: Option<String>,
    pub url: Option<String>,
    pub license: Option<String>,
    pub dependencies: Vec<String>,
    pub status: InstalledStatus,
}
