/// Paket adı güvenliği doğrulayıcısı
///
/// Linux/Arch Linux paket adları sadece alfanümerik karakterler ile '.', '_', '+', '-', '@' içerebilir.
/// Herhangi bir shell metakarakteri (;, &, |, $, `, >, <, boşluk) içeremez.
pub fn is_valid_package_name(name: &str) -> bool {
    let name = name.trim();
    if name.is_empty() || name.len() > 128 {
        return false;
    }

    // İlk karakter alfanümerik olmalıdır
    let first = name.chars().next().unwrap();
    if !first.is_ascii_alphanumeric() {
        return false;
    }

    name.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '+' || c == '-' || c == '@'
    })
}

/// Uygulamanın root (UID 0) olarak çalıştırılıp çalıştırılmadığını kontrol eder.
pub fn is_running_as_root() -> bool {
    rustix::process::geteuid().as_raw() == 0
}

/// Sistem için hayati öneme sahip kritik çekirdek paketler.
/// Bu paketlerin kaldırılması sistemin açılmamasına veya çalışamaz hale gelmesine yol açar.
pub fn is_critical_system_package(name: &str) -> bool {
    const CRITICAL_PACKAGES: &[&str] = &[
        "linux",
        "linux-lts",
        "linux-zen",
        "glibc",
        "systemd",
        "systemd-libs",
        "pacman",
        "coreutils",
        "bash",
        "filesystem",
        "sudo",
        "polkit",
        "archlinux-keyring",
        "shadow",
        "util-linux",
    ];

    let name = name.trim().to_lowercase();
    CRITICAL_PACKAGES.contains(&name.as_str())
}

/// Arama girdisini güvenli hale getirir (Shell karakterleri ve aşırı uzun dizgileri temizler)
pub fn sanitize_search_query(query: &str) -> String {
    let trimmed = query.trim();
    let sanitized: String = trimmed
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.' || *c == '+' || *c == ' ')
        .take(64)
        .collect();
    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_package_names() {
        assert!(is_valid_package_name("firefox"));
        assert!(is_valid_package_name("visual-studio-code-bin"));
        assert!(is_valid_package_name("python-numpy"));
        assert!(is_valid_package_name("gcc-libs_1.2"));
        assert!(is_valid_package_name("lib32-glibc"));
    }

    #[test]
    fn test_invalid_package_names() {
        assert!(!is_valid_package_name("firefox; rm -rf /"));
        assert!(!is_valid_package_name("vlc && sudo reboot"));
        assert!(!is_valid_package_name("$(whoami)"));
        assert!(!is_valid_package_name("`id`"));
        assert!(!is_valid_package_name("foo bar"));
        assert!(!is_valid_package_name(""));
        assert!(!is_valid_package_name("-invalid-start"));
    }

    #[test]
    fn test_critical_packages() {
        assert!(is_critical_system_package("glibc"));
        assert!(is_critical_system_package("systemd"));
        assert!(is_critical_system_package("pacman"));
        assert!(is_critical_system_package("linux"));
        assert!(!is_critical_system_package("vlc"));
        assert!(!is_critical_system_package("firefox"));
    }

    #[test]
    fn test_sanitize_search_query() {
        assert_eq!(sanitize_search_query("firefox; reboot"), "firefox reboot");
        assert_eq!(sanitize_search_query("vlc $(whoami)"), "vlc whoami");
        assert_eq!(sanitize_search_query("  code-bin  "), "code-bin");
    }
}
