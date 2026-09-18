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
                "shopping-cart-symbolic",
                "shopping-cart",
                "edit-download",
                "package-x-generic",
                "package",
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
                "system-file-manager",
                "folder",
                "document-open",
            ],
            "evince" | "org.gnome.Evince" => &[
                "x-office-document",
                "document-viewer",
                "application-pdf",
            ],
            "keepassxc" | "org.keepassxc.KeePassXC" => &[
                "dialog-password",
                "security-high",
                "system-lock-screen",
            ],
            "firefox" => &["firefox", "internet-web-browser"],
            "brave-browser" | "brave" => &[
                "internet-web-browser",
                "web-browser",
                "applications-internet",
            ],
            "thunderbird" | "org.mozilla.thunderbird" => &[
                "mail-client",
                "internet-mail",
                "applications-internet",
            ],
            "telegram" | "telegram-desktop" | "org.telegram.desktop" => &[
                "internet-chat",
                "applications-internet",
            ],
            "code" | "visual-studio-code" | "vscodium" => &[
                "applications-development",
                "text-editor",
                "utilities-terminal",
            ],
            "neovim" | "nvim" => &[
                "utilities-terminal",
                "text-editor",
                "applications-development",
            ],
            "git" | "docker" => &[
                "applications-development",
                "utilities-terminal",
            ],
            "steam" | "lutris" | "heroic" | "bottles" => &[
                "com.usebottles.bottles",
                "heroic",
                "applications-games",
                "input-gaming",
            ],
            "gimp" | "inkscape" | "org.inkscape.Inkscape" => &[
                "applications-graphics",
                "image-x-generic",
                "applications-multimedia",
            ],
            "obs-studio" | "com.obsproject.Studio" => &[
                "com.obsproject.Studio",
                "applications-multimedia",
                "video-x-generic",
            ],
            "blender" => &[
                "applications-graphics",
                "applications-multimedia",
            ],
            "gnome-system-monitor" | "org.gnome.SystemMonitor" => &[
                "utilities-system-monitor",
                "applications-system",
            ],
            "gnome-disk-utility" | "org.gnome.DiskUtility" => &[
                "drive-harddisk",
                "applications-system",
            ],
            "btop" => &[
                "utilities-system-monitor",
                "utilities-terminal",
            ],
            "timeshift" => &[
                "document-revert",
                "applications-system",
                "system-software-update",
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
        }
    }
}
