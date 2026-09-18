use crate::models::{AppCategory, AppItem, PackageSource};

pub struct CatalogService;

impl CatalogService {
    pub fn get_all_apps() -> Vec<AppItem> {
        vec![
            // --- Temel Uygulamalar (Essentials) ---
            AppItem {
                id: "org.gnome.Nautilus".into(),
                name: "Files (Nautilus)".into(),
                package_name: "nautilus".into(),
                description: "GNOME masaüstü için sade ve modern dosya yöneticisi.".into(),
                category: AppCategory::Essentials,
                source: PackageSource::Official,
                icon: "org.gnome.Nautilus".into(),
                homepage: Some("https://apps.gnome.org/Nautilus/".into()),
                license: Some("GPL-3.0-or-later".into()),
                tags: vec!["file-manager".into(), "gnome".into(), "files".into()],
            },
            AppItem {
                id: "org.videolan.VLC".into(),
                name: "VLC Media Player".into(),
                package_name: "vlc".into(),
                description: "Hemen her formatı oynatabilen evrensel açık kaynaklı medya oynatıcı.".into(),
                category: AppCategory::Essentials,
                source: PackageSource::Official,
                icon: "vlc".into(),
                homepage: Some("https://www.videolan.org/vlc/".into()),
                license: Some("GPL-2.0-only".into()),
                tags: vec!["video".into(), "audio".into(), "player".into()],
            },
            AppItem {
                id: "org.keepassxc.KeePassXC".into(),
                name: "KeePassXC".into(),
                package_name: "keepassxc".into(),
                description: "Güvenli, yerel ve çapraz platform şifre yöneticisi.".into(),
                category: AppCategory::Essentials,
                source: PackageSource::Official,
                icon: "org.keepassxc.KeePassXC".into(),
                homepage: Some("https://keepassxc.org/".into()),
                license: Some("GPL-2.0-only".into()),
                tags: vec!["password".into(), "security".into(), "privacy".into()],
            },
            AppItem {
                id: "org.gnome.Evince".into(),
                name: "Document Viewer (Evince)".into(),
                package_name: "evince".into(),
                description: "PDF, PostScript ve benzeri çoklu belge biçimleri için hafif okuyucu.".into(),
                category: AppCategory::Essentials,
                source: PackageSource::Official,
                icon: "org.gnome.Evince".into(),
                homepage: Some("https://apps.gnome.org/Evince/".into()),
                license: Some("GPL-2.0-or-later".into()),
                tags: vec!["pdf".into(), "reader".into(), "document".into()],
            },

            // --- İnternet & Ağ ---
            AppItem {
                id: "org.mozilla.firefox".into(),
                name: "Mozilla Firefox".into(),
                package_name: "firefox".into(),
                description: "Gizliliğe önem veren, hızlı ve modern açık kaynaklı web tarayıcısı.".into(),
                category: AppCategory::Internet,
                source: PackageSource::Official,
                icon: "firefox".into(),
                homepage: Some("https://www.mozilla.org/firefox/".into()),
                license: Some("MPL-2.0".into()),
                tags: vec!["browser".into(), "web".into(), "privacy".into()],
            },
            AppItem {
                id: "com.brave.Browser".into(),
                name: "Brave Browser".into(),
                package_name: "brave-bin".into(),
                description: "Dahili reklam engelleyici ve izleyici korumalı Chromium tabanlı tarayıcı.".into(),
                category: AppCategory::Internet,
                source: PackageSource::Aur,
                icon: "brave-browser".into(),
                homepage: Some("https://brave.com/".into()),
                license: Some("MPL-2.0".into()),
                tags: vec!["browser".into(), "adblock".into(), "chromium".into()],
            },
            AppItem {
                id: "org.mozilla.thunderbird".into(),
                name: "Thunderbird".into(),
                package_name: "thunderbird".into(),
                description: "E-posta, takvim ve kişi yönetimi sunan gelişmiş iletişim istemcisi.".into(),
                category: AppCategory::Internet,
                source: PackageSource::Official,
                icon: "thunderbird".into(),
                homepage: Some("https://www.thunderbird.net/".into()),
                license: Some("MPL-2.0".into()),
                tags: vec!["mail".into(), "email".into(), "calendar".into()],
            },
            AppItem {
                id: "org.telegram.desktop".into(),
                name: "Telegram Desktop".into(),
                package_name: "telegram-desktop".into(),
                description: "Hızlı, bulut tabanlı ve güvenli anlık mesajlaşma uygulaması.".into(),
                category: AppCategory::Internet,
                source: PackageSource::Official,
                icon: "telegram".into(),
                homepage: Some("https://desktop.telegram.org/".into()),
                license: Some("GPL-3.0-only".into()),
                tags: vec!["chat".into(), "messaging".into(), "social".into()],
            },

            // --- Geliştirme Araçları ---
            AppItem {
                id: "com.visualstudio.code".into(),
                name: "Visual Studio Code".into(),
                package_name: "visual-studio-code-bin".into(),
                description: "Gelişmiş kod düzenleyici, hata ayıklama ve geniş eklenti ekosistemi.".into(),
                category: AppCategory::Development,
                source: PackageSource::Aur,
                icon: "code".into(),
                homepage: Some("https://code.visualstudio.com/".into()),
                license: Some("Custom".into()),
                tags: vec!["ide".into(), "editor".into(), "coding".into()],
            },
            AppItem {
                id: "com.vscodium.codium".into(),
                name: "VSCodium".into(),
                package_name: "vscodium-bin".into(),
                description: "Telemetri ve tescilli ikili dosyalardan arındırılmış VS Code sürümü.".into(),
                category: AppCategory::Development,
                source: PackageSource::Aur,
                icon: "vscodium".into(),
                homepage: Some("https://vscodium.com/".into()),
                license: Some("MIT".into()),
                tags: vec!["editor".into(), "privacy".into(), "foss".into()],
            },
            AppItem {
                id: "io.neovim.nvim".into(),
                name: "Neovim".into(),
                package_name: "neovim".into(),
                description: "Genişletilebilir, modern ve son derece hızlı terminal metin editörü.".into(),
                category: AppCategory::Development,
                source: PackageSource::Official,
                icon: "nvim".into(),
                homepage: Some("https://neovim.io/".into()),
                license: Some("Apache-2.0".into()),
                tags: vec!["terminal".into(), "editor".into(), "vim".into()],
            },
            AppItem {
                id: "org.git_scm.Git".into(),
                name: "Git".into(),
                package_name: "git".into(),
                description: "Hızlı ve dağıtık versiyon kontrol sistemi.".into(),
                category: AppCategory::Development,
                source: PackageSource::Official,
                icon: "git".into(),
                homepage: Some("https://git-scm.com/".into()),
                license: Some("GPL-2.0-only".into()),
                tags: vec!["vcs".into(), "git".into(), "cli".into()],
            },
            AppItem {
                id: "com.docker.Docker".into(),
                name: "Docker".into(),
                package_name: "docker".into(),
                description: "Konteyner tabanlı uygulama geliştirme ve dağıtım platformu.".into(),
                category: AppCategory::Development,
                source: PackageSource::Official,
                icon: "docker".into(),
                homepage: Some("https://www.docker.com/".into()),
                license: Some("Apache-2.0".into()),
                tags: vec!["containers".into(), "devops".into(), "cloud".into()],
            },

            // --- Oyun & Eğlence ---
            AppItem {
                id: "com.valvesoftware.Steam".into(),
                name: "Steam".into(),
                package_name: "steam".into(),
                description: "Valve tarafından geliştirilen lider dijital oyun platformu ve kütüphanesi.".into(),
                category: AppCategory::Gaming,
                source: PackageSource::Official,
                icon: "steam".into(),
                homepage: Some("https://store.steampowered.com/".into()),
                license: Some("Proprietary".into()),
                tags: vec!["gaming".into(), "store".into(), "proton".into()],
            },
            AppItem {
                id: "net.lutris.Lutris".into(),
                name: "Lutris".into(),
                package_name: "lutris".into(),
                description: "Tüm platform oyunlarınızı tek çatı altında toplayan açık kaynaklı oyun yöneticisi.".into(),
                category: AppCategory::Gaming,
                source: PackageSource::Official,
                icon: "lutris".into(),
                homepage: Some("https://lutris.net/".into()),
                license: Some("GPL-3.0-or-later".into()),
                tags: vec!["games".into(), "wine".into(), "launcher".into()],
            },
            AppItem {
                id: "com.heroicgameslauncher.hgl".into(),
                name: "Heroic Games Launcher".into(),
                package_name: "heroic-games-launcher-bin".into(),
                description: "Epic Games, GOG ve Amazon Prime oyunları için native açık kaynaklı istemci.".into(),
                category: AppCategory::Gaming,
                source: PackageSource::Aur,
                icon: "heroic".into(),
                homepage: Some("https://heroicgameslauncher.com/".into()),
                license: Some("GPL-3.0-only".into()),
                tags: vec!["gog".into(), "epic".into(), "gaming".into()],
            },
            AppItem {
                id: "com.usebottles.bottles".into(),
                name: "Bottles".into(),
                package_name: "bottles".into(),
                description: "Linux üzerinde Windows yazılımlarını ve oyunlarını kolayca çalıştırma ortamı.".into(),
                category: AppCategory::Gaming,
                source: PackageSource::Official,
                icon: "com.usebottles.bottles".into(),
                homepage: Some("https://usebottles.com/".into()),
                license: Some("GPL-3.0-or-later".into()),
                tags: vec!["wine".into(), "windows".into(), "emulation".into()],
            },

            // --- Tasarım & Medya ---
            AppItem {
                id: "org.gimp.GIMP".into(),
                name: "GIMP".into(),
                package_name: "gimp".into(),
                description: "Profesyonel raster grafik ve fotoğraf düzenleme yazılımı.".into(),
                category: AppCategory::Multimedia,
                source: PackageSource::Official,
                icon: "gimp".into(),
                homepage: Some("https://www.gimp.org/".into()),
                license: Some("GPL-3.0-or-later".into()),
                tags: vec!["photo".into(), "image".into(), "design".into()],
            },
            AppItem {
                id: "org.inkscape.Inkscape".into(),
                name: "Inkscape".into(),
                package_name: "inkscape".into(),
                description: "Vektörel çizim, logo ve illüstrasyon geliştirme aracı.".into(),
                category: AppCategory::Multimedia,
                source: PackageSource::Official,
                icon: "org.inkscape.Inkscape".into(),
                homepage: Some("https://inkscape.org/".into()),
                license: Some("GPL-3.0-or-later".into()),
                tags: vec!["vector".into(), "svg".into(), "graphics".into()],
            },
            AppItem {
                id: "com.obsproject.Studio".into(),
                name: "OBS Studio".into(),
                package_name: "obs-studio".into(),
                description: "Ekran kaydı ve canlı yayın için endüstri standardı açık kaynaklı stüdyo.".into(),
                category: AppCategory::Multimedia,
                source: PackageSource::Official,
                icon: "com.obsproject.Studio".into(),
                homepage: Some("https://obsproject.com/".into()),
                license: Some("GPL-2.0-or-later".into()),
                tags: vec!["stream".into(), "recording".into(), "video".into()],
            },
            AppItem {
                id: "org.blender.Blender".into(),
                name: "Blender".into(),
                package_name: "blender".into(),
                description: "3D modelleme, render alma, animasyon ve görsel efekt paketi.".into(),
                category: AppCategory::Multimedia,
                source: PackageSource::Official,
                icon: "blender".into(),
                homepage: Some("https://www.blender.org/".into()),
                license: Some("GPL-3.0-or-later".into()),
                tags: vec!["3d".into(), "animation".into(), "render".into()],
            },

            // --- Sistem & Donanım ---
            AppItem {
                id: "org.gnome.SystemMonitor".into(),
                name: "System Monitor".into(),
                package_name: "gnome-system-monitor".into(),
                description: "Sistem kaynaklarını, işlemci ve bellek kullanımını grafiklerle izleme.".into(),
                category: AppCategory::System,
                source: PackageSource::Official,
                icon: "org.gnome.SystemMonitor".into(),
                homepage: Some("https://apps.gnome.org/SystemMonitor/".into()),
                license: Some("GPL-2.0-or-later".into()),
                tags: vec!["monitor".into(), "cpu".into(), "ram".into()],
            },
            AppItem {
                id: "org.gnome.DiskUtility".into(),
                name: "Disks".into(),
                package_name: "gnome-disk-utility".into(),
                description: "Diskleri yönetme, bölümleme ve S.M.A.R.T. sağlık kontrolleri yapma.".into(),
                category: AppCategory::System,
                source: PackageSource::Official,
                icon: "org.gnome.DiskUtility".into(),
                homepage: Some("https://apps.gnome.org/DiskUtility/".into()),
                license: Some("GPL-2.0-or-later".into()),
                tags: vec!["disk".into(), "storage".into(), "partition".into()],
            },
            AppItem {
                id: "com.github.aristocratos.btop".into(),
                name: "btop".into(),
                package_name: "btop".into(),
                description: "Görsel olarak zengin, hızlı ve modern terminal kaynak monitörü.".into(),
                category: AppCategory::System,
                source: PackageSource::Official,
                icon: "utilities-terminal".into(),
                homepage: Some("https://github.com/aristocratos/btop".into()),
                license: Some("Apache-2.0".into()),
                tags: vec!["tui".into(), "cli".into(), "performance".into()],
            },
            AppItem {
                id: "org.timeshift.Timeshift".into(),
                name: "Timeshift".into(),
                package_name: "timeshift".into(),
                description: "BTRFS veya RSYNC kullanarak sistem geri yükleme noktaları ve yedekleri oluşturma.".into(),
                category: AppCategory::System,
                source: PackageSource::Official,
                icon: "timeshift".into(),
                homepage: Some("https://github.com/linuxmint/timeshift".into()),
                license: Some("GPL-3.0-or-later".into()),
                tags: vec!["backup".into(), "restore".into(), "snapshot".into()],
            },
        ]
    }

    pub fn get_by_category(category: AppCategory) -> Vec<AppItem> {
        Self::get_all_apps()
            .into_iter()
            .filter(|app| app.category == category)
            .collect()
    }

    pub fn search(query: &str) -> Vec<AppItem> {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return Self::get_all_apps();
        }

        Self::get_all_apps()
            .into_iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&q)
                    || app.package_name.to_lowercase().contains(&q)
                    || app.description.to_lowercase().contains(&q)
                    || app.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }
}
