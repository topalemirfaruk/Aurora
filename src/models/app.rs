use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageSource {
    Official,
    Aur,
    Flatpak,
}

impl PackageSource {
    pub fn badge_label(&self) -> &'static str {
        match self {
            Self::Official => "OFFICIAL",
            Self::Aur => "AUR",
            Self::Flatpak => "FLATPAK",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Official => "badge-official",
            Self::Aur => "badge-aur",
            Self::Flatpak => "badge-flatpak",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppCategory {
    Essentials,
    Internet,
    Development,
    Gaming,
    Multimedia,
    System,
    Drivers,
    Codecs,
    Runtimes,
}

impl AppCategory {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Essentials => "Temel Uygulamalar",
            Self::Internet => "İnternet & Ağ",
            Self::Development => "Geliştirme Araçları",
            Self::Gaming => "Oyun & Eğlence",
            Self::Multimedia => "Tasarım & Medya",
            Self::System => "Sistem & Donanım",
            Self::Drivers => "Sürücüler & Donanım",
            Self::Codecs => "Medya Kodekleri",
            Self::Runtimes => "Çalışma Zamanları",
        }
    }

    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::Essentials => "user-home-symbolic",
            Self::Internet => "applications-internet-symbolic",
            Self::Development => "applications-development-symbolic",
            Self::Gaming => "applications-games-symbolic",
            Self::Multimedia => "applications-multimedia-symbolic",
            Self::System => "applications-system-symbolic",
            Self::Drivers => "video-display-symbolic",
            Self::Codecs => "media-optical-audio-symbolic",
            Self::Runtimes => "system-run-symbolic",
        }
    }

    pub fn all() -> &'static [AppCategory] {
        &[
            Self::Essentials,
            Self::Internet,
            Self::Development,
            Self::Gaming,
            Self::Multimedia,
            Self::System,
            Self::Drivers,
            Self::Codecs,
            Self::Runtimes,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppItem {
    pub id: String,
    pub name: String,
    pub package_name: String,
    pub description: String,
    pub category: AppCategory,
    pub source: PackageSource,
    pub icon: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub tags: Vec<String>,
}
