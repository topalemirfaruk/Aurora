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

    #[test]
    fn test_parse_updates_output() {
        use aurora::package_managers::pacman::PacmanManager;

        let sample_output = "\
firefox 134.0-1 -> 135.0-1
glibc 2.40-2 -> 2.41-1
linux 6.13.1.arch1-1 -> 6.13.2.arch1-1
ignored-pkg 1.0-1 -> 1.0-1
bad-line-without-arrow
";
        let updates = PacmanManager::parse_updates_output(sample_output);
        assert_eq!(updates.len(), 3);
        assert_eq!(updates[0].name, "firefox");
        assert_eq!(updates[0].old_version, "134.0-1");
        assert_eq!(updates[0].new_version, "135.0-1");

        assert_eq!(updates[1].name, "glibc");
        assert_eq!(updates[1].old_version, "2.40-2");
        assert_eq!(updates[1].new_version, "2.41-1");

        assert_eq!(updates[2].name, "linux");
        assert_eq!(updates[2].old_version, "6.13.1.arch1-1");
        assert_eq!(updates[2].new_version, "6.13.2.arch1-1");
    }

    #[test]
    fn test_maintenance_command_builders() {
        use aurora::services::MaintenanceService;

        let (cmd, args) = MaintenanceService::build_cache_clean_command(false);
        assert!(!cmd.is_empty());
        assert!(!args.is_empty());

        let (cmd_u, args_u) = MaintenanceService::build_cache_clean_command(true);
        assert!(!cmd_u.is_empty());
        assert!(!args_u.is_empty());

        // Empty orphans should return None
        let empty_clean = MaintenanceService::build_orphan_clean_command(&[]);
        assert!(empty_clean.is_none());

        // Valid orphans
        let orphans = vec!["python-build".to_string(), "scdoc".to_string()];
        let orphan_cmd = MaintenanceService::build_orphan_clean_command(&orphans);
        assert!(orphan_cmd.is_some());
        let (ocmd, oargs) = orphan_cmd.unwrap();
        assert_eq!(ocmd, "sudo");
        assert!(oargs.contains(&"python-build".to_string()));

        // Critical package protection in orphan cleaner
        let critical_orphans = vec!["glibc".to_string(), "linux".to_string(), "systemd".to_string()];
        let blocked = MaintenanceService::build_orphan_clean_command(&critical_orphans);
        assert!(blocked.is_none(), "Critical system packages must never be targeted for orphan removal");

        // Service restart
        let s_cmd = MaintenanceService::build_service_restart_command("bluetooth.service");
        assert!(s_cmd.is_some());
        let (scmd, sargs) = s_cmd.unwrap();
        assert_eq!(scmd, "sudo");
        assert_eq!(sargs, vec!["systemctl", "restart", "--", "bluetooth.service"]);

        // Invalid service name with injection chars
        let bad_service = MaintenanceService::build_service_restart_command("bad;rm -rf /");
        assert!(bad_service.is_none());
    }

    #[tokio::test]
    async fn test_live_or_mock_check_updates_integration() {
        use aurora::package_managers::pacman::PacmanManager;
        let res = PacmanManager::check_updates().await;
        assert!(res.is_ok(), "check_updates must return Ok");
        let updates = res.unwrap();
        // Bu sistemde güncellemeler mevcut olduğundan listenin başarıyla dolduğunu teyit ediyoruz
        assert!(!updates.is_empty(), "Updates should be detected on this system");
        for u in updates.iter().take(5) {
            assert_ne!(u.old_version, u.new_version);
        }
    }
}
