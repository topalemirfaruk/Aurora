use which::which;

#[derive(Debug, Clone)]
pub struct SystemCapabilities {
    pub has_pacman: bool,
    pub has_paru: bool,
    pub has_yay: bool,
    pub has_flatpak: bool,
}

impl SystemCapabilities {
    pub fn detect() -> Self {
        Self {
            has_pacman: which("pacman").is_ok(),
            has_paru: which("paru").is_ok(),
            has_yay: which("yay").is_ok(),
            has_flatpak: which("flatpak").is_ok(),
        }
    }

    pub fn preferred_aur_helper(&self) -> Option<&'static str> {
        if self.has_paru {
            Some("paru")
        } else if self.has_yay {
            Some("yay")
        } else {
            None
        }
    }
}
