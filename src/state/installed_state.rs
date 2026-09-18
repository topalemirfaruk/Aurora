use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use crate::process::CommandExecutor;

#[derive(Clone)]
pub struct InstalledState {
    installed_pkgs: Rc<RefCell<HashSet<String>>>,
    listeners: Rc<RefCell<Vec<Box<dyn Fn() + 'static>>>>,
}

impl Default for InstalledState {
    fn default() -> Self {
        Self::new()
    }
}

impl InstalledState {
    pub fn new() -> Self {
        let state = Self {
            installed_pkgs: Rc::new(RefCell::new(HashSet::new())),
            listeners: Rc::new(RefCell::new(Vec::new())),
        };
        state.refresh_background();
        state
    }

    /// Verilen paketin sistemde (Pacman, AUR veya Flatpak) kurulu olup olmadığını döner (O(1))
    pub fn is_installed(&self, package_name: &str) -> bool {
        let set = self.installed_pkgs.borrow();
        if set.contains(package_name) {
            return true;
        }

        // Bazı AUR paketleri "-bin" veya "-git" son eki ile kurulabilir veya tersi (örn: brave-bin -> brave)
        if let Some(base) = package_name.strip_suffix("-bin") {
            if set.contains(base) {
                return true;
            }
        }
        if let Some(base) = package_name.strip_suffix("-git") {
            if set.contains(base) {
                return true;
            }
        }

        let bin_variant = format!("{}-bin", package_name);
        if set.contains(&bin_variant) {
            return true;
        }
        let git_variant = format!("{}-git", package_name);
        if set.contains(&git_variant) {
            return true;
        }

        // Yaygın paket adı varyasyonları
        match package_name {
            "brave-bin" | "brave" => set.contains("brave") || set.contains("brave-bin"),
            "visual-studio-code-bin" | "code" | "code-oss" => {
                set.contains("visual-studio-code-bin")
                    || set.contains("code")
                    || set.contains("code-oss")
                    || set.contains("com.visualstudio.code")
            }
            "google-chrome" => set.contains("google-chrome") || set.contains("google-chrome-stable") || set.contains("com.google.Chrome"),
            "spotify" => set.contains("spotify") || set.contains("spotify-launcher") || set.contains("com.spotify.Client"),
            "discord" => set.contains("discord") || set.contains("com.discordapp.Discord"),
            "steam" => set.contains("steam") || set.contains("com.valvesoftware.Steam"),
            "telegram-desktop" => set.contains("telegram-desktop") || set.contains("org.telegram.desktop"),
            "obs-studio" => set.contains("obs-studio") || set.contains("com.obsproject.Studio"),
            "gimp" => set.contains("gimp") || set.contains("org.gimp.GIMP"),
            "vlc" => set.contains("vlc") || set.contains("org.videolan.VLC"),
            "keepassxc" => set.contains("keepassxc") || set.contains("org.keepassxc.KeePassXC"),
            "libreoffice-fresh" | "libreoffice-still" | "libreoffice" => {
                set.contains("libreoffice-fresh") || set.contains("libreoffice-still") || set.contains("org.libreoffice.LibreOffice")
            }
            _ => false,
        }
    }

    /// Bir AppItem'ın ID veya paket adına göre kurulu olup olmadığını döner
    pub fn is_installed_item(&self, item: &crate::models::AppItem) -> bool {
        let set = self.installed_pkgs.borrow();
        if set.contains(&item.id) {
            return true;
        }
        self.is_installed(&item.package_name)
    }

    /// Arka planda asenkron olarak kurulu paketleri yükler ve dinleyicileri uyarır
    pub fn refresh_background(&self) {
        let this = self.clone();
        glib::spawn_future_local(async move {
            let mut set = HashSet::new();

            // 1. Pacman / AUR paketleri (pacman -Q)
            if let Ok((success, stdout, _)) = CommandExecutor::run_captured("pacman", &["-Q"]).await {
                if success {
                    for line in stdout.lines() {
                        if let Some(pkg) = line.split_whitespace().next() {
                            set.insert(pkg.to_string());
                        }
                    }
                }
            }

            // 2. Flatpak paketleri
            if let Ok((success, stdout, _)) = CommandExecutor::run_captured("flatpak", &["list", "--app", "--columns=application"]).await {
                if success {
                    for line in stdout.lines() {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            set.insert(trimmed.to_string());
                        }
                    }
                }
            }

            *this.installed_pkgs.borrow_mut() = set;

            // Dinleyicileri tetikle
            for cb in this.listeners.borrow().iter() {
                cb();
            }
        });
    }

    pub fn on_change<F: Fn() + 'static>(&self, callback: F) {
        self.listeners.borrow_mut().push(Box::new(callback));
    }
}
