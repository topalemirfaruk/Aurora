#[cfg(test)]
mod tests {
    use aurora::models::PackageSource;
    use aurora::package_managers::flatpak::FlatpakApp;
    use aurora::package_managers::pacman::{InstalledPackage, PackageUpdate, SearchResult};

    #[test]
    fn test_package_source_labels_and_classes() {
        assert_eq!(PackageSource::Official.badge_label(), "OFFICIAL");
        assert_eq!(PackageSource::Official.css_class(), "badge-official");

        assert_eq!(PackageSource::Aur.badge_label(), "AUR");
        assert_eq!(PackageSource::Aur.css_class(), "badge-aur");

        assert_eq!(PackageSource::Flatpak.badge_label(), "FLATPAK");
        assert_eq!(PackageSource::Flatpak.css_class(), "badge-flatpak");
    }

    #[test]
    fn test_installed_package_model() {
        let pkg = InstalledPackage {
            name: "firefox".to_string(),
            version: "135.0-1".to_string(),
            source: PackageSource::Official,
        };
        assert_eq!(pkg.name, "firefox");
        assert_eq!(pkg.version, "135.0-1");
        assert_eq!(pkg.source, PackageSource::Official);

        let aur_pkg = InstalledPackage {
            name: "visual-studio-code-bin".to_string(),
            version: "1.97.0-1".to_string(),
            source: PackageSource::Aur,
        };
        assert_eq!(aur_pkg.source, PackageSource::Aur);
    }

    #[test]
    fn test_flatpak_app_model() {
        let app = FlatpakApp {
            name: "VLC".to_string(),
            app_id: "org.videolan.VLC".to_string(),
            version: "3.0.23".to_string(),
        };
        assert_eq!(app.name, "VLC");
        assert_eq!(app.app_id, "org.videolan.VLC");
        assert_eq!(app.version, "3.0.23");
    }

    #[test]
    fn test_search_result_model() {
        let res = SearchResult {
            repo: "flathub".to_string(),
            name: "org.mozilla.firefox".to_string(),
            version: "135.0".to_string(),
            description: "Fast, Private & Safe Web Browser".to_string(),
            is_installed: false,
        };
        assert_eq!(res.repo, "flathub");
        assert_eq!(res.name, "org.mozilla.firefox");
        assert!(!res.is_installed);
    }

    #[test]
    fn test_package_update_struct() {
        let upd = PackageUpdate {
            name: "systemd".to_string(),
            old_version: "256.4-1".to_string(),
            new_version: "256.5-1".to_string(),
        };
        assert_eq!(upd.name, "systemd");
        assert_ne!(upd.old_version, upd.new_version);
    }
}
