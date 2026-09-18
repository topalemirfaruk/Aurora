use crate::models::AppCategory;
use gtk4::gdk::Display;
use gtk4::IconTheme;

pub struct IconResolver;

impl IconResolver {
    /// İkonun sistemde var olup olmadığını kontrol eder, yoksa en uygun profesyonel yedeği döndürür.
    pub fn resolve(requested_icon: &str, category: AppCategory) -> String {
        let display = match Display::default() {
            Some(d) => d,
            None => return Self::fallback_for_category(category).to_string(),
        };

        let theme = IconTheme::for_display(&display);

        // 1. İstenen ikon doğrudan sistem temasında var mı?
        if theme.has_icon(requested_icon) {
            return requested_icon.to_string();
        }

        // 2. Bilinen alternatif eşleşmeler (Alias listesi)
        let aliases = Self::get_aliases(requested_icon);
        for alias in aliases {
            if theme.has_icon(alias) {
                return alias.to_string();
            }
        }

        // 3. Kategoriye özel garantili XDG sistem simgesi
        let cat_fallback = Self::fallback_for_category(category);
        if theme.has_icon(cat_fallback) {
            return cat_fallback.to_string();
        }

        // 4. Son çare
        "application-x-executable".to_string()
    }

    /// Verilen aday ikonlar arasında sistemde ilk bulunanı döndürür, hiçbiri yoksa fallback'i döner.
    pub fn first_available(candidates: &[&'static str], fallback: &'static str) -> &'static str {
        if let Some(display) = Display::default() {
            let theme = IconTheme::for_display(&display);
            for name in candidates {
                if theme.has_icon(name) {
                    return name;
                }
            }
        }
        fallback
    }

    /// Kurulum Sepeti / Kuyruk için garantili sistem ikonu
    pub fn cart_icon() -> &'static str {
        Self::first_available(
            &[
                "aurora-empty-cart",
                "package-x-generic",
                "folder-download-symbolic",
                "shopping-cart",
            ],
            "package-x-generic",
        )
    }

    /// Tema değiştirici için garantili gece/gündüz ikonu
    pub fn theme_icon() -> &'static str {
        Self::theme_icon_for(true)
    }

    /// Tema durumuna göre uygun ikon (Koyu moddaysa açık temaya geçiş için güneşi, Açık moddaysa ayı gösterir)
    pub fn theme_icon_for(is_dark: bool) -> &'static str {
        if is_dark {
            Self::first_available(
                &[
                    "weather-clear-symbolic",
                    "display-brightness-symbolic",
                    "weather-clear",
                ],
                "display-brightness-symbolic",
            )
        } else {
            Self::first_available(
                &[
                    "night-light-symbolic",
                    "weather-clear-night-symbolic",
                    "weather-clear-night",
                ],
                "night-light-symbolic",
            )
        }
    }

    fn get_aliases(icon: &str) -> &'static [&'static str] {
        match icon {
            "nautilus" | "org.gnome.Nautilus" => &[
                "org.gnome.Nautilus",
                "system-file-manager",
                "folder",
                "document-open",
            ],
            "dolphin" | "org.kde.dolphin" => &[
                "org.kde.dolphin",
                "system-file-manager",
                "folder",
            ],
            "thunar" | "org.xfce.thunar" => &[
                "org.xfce.thunar",
                "system-file-manager",
                "folder",
            ],
            "evince" | "org.gnome.Evince" => &[
                "org.gnome.Evince",
                "document-viewer",
                "x-office-document",
                "application-pdf",
            ],
            "okular" | "org.kde.okular" => &[
                "okular",
                "org.kde.okular",
                "document-viewer",
            ],
            "keepassxc" | "org.keepassxc.KeePassXC" => &[
                "keepassxc",
                "org.keepassxc.KeePassXC",
                "keepassxc-unlocked",
                "dialog-password",
                "security-high",
            ],
            "bitwarden" => &[
                "bitwarden",
                "dialog-password",
                "security-high",
            ],
            "firefox" => &["firefox", "internet-web-browser"],
            "brave-browser" | "brave" | "brave-bin" | "com.brave.Browser" => &[
                "brave-desktop",
                "brave-browser",
                "brave",
                "com.brave.Browser",
                "internet-web-browser",
            ],
            "google-chrome" | "google-chrome-stable" => &[
                "google-chrome",
                "google-chrome-stable",
                "chromium",
                "internet-web-browser",
            ],
            "chromium" | "org.chromium.Chromium" => &[
                "chromium",
                "org.chromium.Chromium",
                "internet-web-browser",
            ],
            "librewolf" | "librewolf-bin" => &[
                "librewolf",
                "firefox",
                "internet-web-browser",
            ],
            "tor-browser" | "tor-browser-bin" => &[
                "tor-browser",
                "internet-web-browser",
            ],
            "thunderbird" | "org.mozilla.thunderbird" => &[
                "thunderbird",
                "org.mozilla.thunderbird",
                "mail-client",
                "internet-mail",
            ],
            "telegram" | "telegram-desktop" | "org.telegram.desktop" => &[
                "org.telegram.desktop",
                "telegram-desktop",
                "telegram",
                "internet-chat",
            ],
            "discord" | "com.discordapp.Discord" => &[
                "discord",
                "com.discordapp.Discord",
                "internet-chat",
            ],
            "vesktop" | "vesktop-bin" => &[
                "vesktop",
                "discord",
                "internet-chat",
            ],
            "slack" | "slack-desktop" => &[
                "slack",
                "slack-desktop",
                "internet-chat",
            ],
            "signal" | "signal-desktop" => &[
                "signal-desktop",
                "signal",
                "internet-chat",
            ],
            "spotify" | "com.spotify.Client" => &[
                "spotify",
                "spotify-client",
                "com.spotify.Client",
                "audio-player",
            ],
            "vlc" | "org.videolan.VLC" => &[
                "vlc",
                "org.videolan.VLC",
                "video-player",
            ],
            "mpv" => &[
                "mpv",
                "video-player",
                "applications-multimedia",
            ],
            "code" | "visual-studio-code" | "vscodium" | "com.visualstudio.code" => &[
                "com.visualstudio.code.oss",
                "com.visualstudio.code",
                "code",
                "visual-studio-code",
                "vscodium",
                "applications-development",
                "text-editor",
            ],
            "sublime-text" | "sublime-text-4" => &[
                "sublime-text",
                "sublime-text-4",
                "text-editor",
                "applications-development",
            ],
            "neovim" | "nvim" => &[
                "nvim",
                "neovim",
                "utilities-terminal",
                "text-editor",
            ],
            "gitkraken" => &[
                "gitkraken",
                "applications-development",
            ],
            "dbeaver" => &[
                "dbeaver",
                "dbeaver-ce",
                "applications-development",
            ],
            "postman" => &[
                "postman",
                "applications-development",
            ],
            "docker" => &[
                "docker",
                "applications-development",
            ],
            "steam" => &[
                "steam",
                "com.valvesoftware.Steam",
                "applications-games",
            ],
            "lutris" | "net.lutris.Lutris" => &[
                "lutris",
                "net.lutris.Lutris",
                "applications-games",
            ],
            "heroic" | "heroic-games-launcher" => &[
                "heroic",
                "com.heroicgameslauncher.hgl",
                "applications-games",
            ],
            "bottles" | "com.usebottles.bottles" => &[
                "com.usebottles.bottles",
                "bottles",
                "applications-games",
            ],
            "retroarch" => &[
                "retroarch",
                "applications-games",
            ],
            "gimp" | "org.gimp.GIMP" => &[
                "org.gimp.GIMP",
                "gimp",
                "applications-graphics",
            ],
            "krita" | "org.kde.krita" => &[
                "krita",
                "org.kde.krita",
                "applications-graphics",
            ],
            "inkscape" | "org.inkscape.Inkscape" => &[
                "org.inkscape.Inkscape",
                "inkscape",
                "applications-graphics",
            ],
            "obs-studio" | "com.obsproject.Studio" => &[
                "com.obsproject.Studio",
                "obs-studio",
                "obs",
                "applications-multimedia",
            ],
            "blender" | "org.blender.Blender" => &[
                "blender",
                "org.blender.Blender",
                "applications-graphics",
            ],
            "kdenlive" | "org.kde.kdenlive" => &[
                "kdenlive",
                "org.kde.kdenlive",
                "applications-multimedia",
            ],
            "audacity" => &[
                "audacity",
                "applications-multimedia",
            ],
            "handbrake" | "fr.handbrake.ghb" => &[
                "fr.handbrake.ghb",
                "handbrake",
                "applications-multimedia",
            ],
            "alacritty" | "Alacritty" => &[
                "Alacritty",
                "alacritty",
                "utilities-terminal",
            ],
            "kitty" => &[
                "kitty",
                "utilities-terminal",
            ],
            "fastfetch" => &[
                "fastfetch",
                "utilities-terminal",
            ],
            "btop" => &[
                "btop",
                "utilities-system-monitor",
            ],
            "htop" => &[
                "htop",
                "utilities-system-monitor",
            ],
            "timeshift" => &[
                "timeshift",
                "timeshift-gtk",
                "document-revert",
            ],
            "gparted" => &[
                "gparted",
                "drive-harddisk",
            ],
            "flatseal" | "com.github.tchx84.Flatseal" => &[
                "com.github.tchx84.Flatseal",
                "flatseal",
                "preferences-system",
            ],
            // Sürücüler & Donanım
            "vulkan" | "vulkan-radeon" | "vulkan-intel" => &[
                "vulkan",
                "video-display",
                "preferences-desktop-display",
            ],
            "nvidia" | "nvidia-open" | "nvidia-utils" => &[
                "nvidia",
                "nvidia-settings",
                "video-display",
            ],
            "mesa" => &[
                "mesa",
                "video-display",
                "applications-system",
            ],
            "pipewire" | "pipewire-pulse" => &[
                "audio-card",
                "multimedia-volume-control",
                "preferences-desktop-sound",
            ],
            "bluez" | "bluetooth" => &[
                "bluetooth",
                "preferences-system-bluetooth",
                "network-wireless",
            ],
            "cups" | "printer" => &[
                "printer",
                "cups",
                "document-print",
            ],
            // Kodekler
            "ffmpeg" => &[
                "ffmpeg",
                "video-x-generic",
                "applications-multimedia",
            ],
            "gstreamer" | "gst-plugins" => &[
                "gstreamer-properties",
                "media-optical-audio",
                "applications-multimedia",
            ],
            "vaapi" | "intel-media-driver" | "libva" => &[
                "video-display",
                "media-playback-start",
                "applications-multimedia",
            ],
            // Çalışma Zamanları
            "wine" | "winetricks" => &[
                "wine",
                "wine-staging",
                "wine-winecfg",
                "system-run",
            ],
            "gamemode" | "gamescope" => &[
                "speedometer",
                "applications-games",
                "system-run",
            ],
            "java" | "openjdk" => &[
                "java",
                "openjdk",
                "applications-development",
            ],
            "nodejs" => &[
                "nodejs",
                "node",
                "applications-development",
            ],
            "python" => &[
                "python",
                "python3",
                "applications-development",
            ],
            "dotnet" => &[
                "dotnet",
                "applications-development",
            ],
            _ => &[],
        }
    }

    fn fallback_for_category(category: AppCategory) -> &'static str {
        match category {
            AppCategory::Essentials => "package-x-generic",
            AppCategory::Internet => "internet-web-browser",
            AppCategory::Development => "applications-development",
            AppCategory::Gaming => "applications-games",
            AppCategory::Multimedia => "applications-multimedia",
            AppCategory::System => "applications-system",
            AppCategory::Drivers => "video-display",
            AppCategory::Codecs => "media-optical-audio",
            AppCategory::Runtimes => "system-run",
        }
    }
}
