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
    fn test_parse_search_output_multiline_wrapping() {
        use aurora::package_managers::pacman::PacmanManager;

        let sample_output = "\
extra/libreoffice-fresh 26.8.0-2 [installed]
    LibreOffice is the free and open source personal productivity suite for
    Windows, Macintosh and GNU/Linux. This package contains the latest stable
    release.
core/linux 6.13.2.arch1-1 [kurulu]
    The Linux kernel and modules
aur/visual-studio-code-bin 1.97.0-1
    Visual Studio Code (vscode): Editor for building and debugging modern web
    and cloud applications, official binary version
";

        let results = PacmanManager::parse_search_output(sample_output, "repo");
        assert_eq!(results.len(), 3, "Tam olarak 3 paket bulunmalı, sahte kayıt olmamalı");

        // 1. Paket (3 satıra bölünmüş açıklama birleştirilmeli)
        assert_eq!(results[0].repo, "extra");
        assert_eq!(results[0].name, "libreoffice-fresh");
        assert_eq!(results[0].version, "26.8.0-2");
        assert!(results[0].is_installed);
        assert_eq!(
            results[0].description,
            "LibreOffice is the free and open source personal productivity suite for Windows, Macintosh and GNU/Linux. This package contains the latest stable release."
        );

        // 2. Paket (Türkçe [kurulu] etiketi)
        assert_eq!(results[1].repo, "core");
        assert_eq!(results[1].name, "linux");
        assert_eq!(results[1].version, "6.13.2.arch1-1");
        assert!(results[1].is_installed);
        assert_eq!(results[1].description, "The Linux kernel and modules");

        // 3. Paket (AUR ve 2 satıra sarılmış açıklama)
        assert_eq!(results[2].repo, "aur");
        assert_eq!(results[2].name, "visual-studio-code-bin");
        assert_eq!(results[2].version, "1.97.0-1");
        assert!(!results[2].is_installed);
        assert_eq!(
            results[2].description,
            "Visual Studio Code (vscode): Editor for building and debugging modern web and cloud applications, official binary version"
        );
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

    #[tokio::test]
    async fn test_aur_multi_word_search_rpc_url_encoding() {
        use aurora::package_managers::aur::AurManager;
        use aurora::security::url_encode;

        // Verify URL encoding of spaces and special chars
        let encoded = url_encode("visual studio");
        assert_eq!(encoded, "visual%20studio");

        // AurManager::search ile çok kelimeli arama (curl hatası 3 vermemeli)
        let res = AurManager::search("visual studio").await;
        assert!(res.is_ok(), "Multi-word AUR search must not fail");
        let results = res.unwrap();
        assert!(!results.is_empty(), "Multi-word query should return packages from AUR");
    }

    #[tokio::test]
    async fn test_command_executor_cancellation_kills_process() {
        use aurora::process::{CommandExecutor, ProcessMessage};
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        use std::time::{Duration, Instant};

        let (cancel_tx, cancel_rx) = tokio::sync::watch::channel(false);
        let cancelled_received = Arc::new(AtomicBool::new(false));
        let cr = cancelled_received.clone();

        let args = vec!["10".to_string()];
        let start = Instant::now();

        let handle = tokio::spawn(async move {
            CommandExecutor::run_streaming_cancellable("sleep", &args, cancel_rx, move |msg| {
                if let ProcessMessage::Finished(success, code) = msg {
                    if !success && code.is_none() {
                        cr.store(true, Ordering::SeqCst);
                    }
                }
            })
            .await
        });

        // 50ms sonra iptal sinyali gönder
        tokio::time::sleep(Duration::from_millis(50)).await;
        let _ = cancel_tx.send(true);

        // Süreç 10 saniye beklemeden hemen (< 2 saniye) öldürülmeli ve bitmeli
        let res = tokio::time::timeout(Duration::from_secs(2), handle).await;
        assert!(res.is_ok(), "Process should be killed promptly, not wait 10s");
        let run_res = res.unwrap().unwrap();
        assert!(run_res.is_ok());
        assert_eq!(run_res.unwrap(), false, "Cancelled process should return false");
        assert!(start.elapsed() < Duration::from_secs(2), "Killed within 2 seconds");
        assert!(cancelled_received.load(Ordering::SeqCst), "ProcessMessage::Finished with false and None code received");
    }

    #[test]
    fn test_build_helper_args_paru_and_yay() {
        use aurora::ui::widgets::InstallDialog;

        let pkgs = vec!["google-chrome".to_string(), "visual-studio-code-bin".to_string()];

        // Paru with skip_review = true
        let (cmd, args) = InstallDialog::build_helper_args("paru", &pkgs, true);
        assert_eq!(cmd, "paru");
        assert!(args.contains(&"--skipreview".to_string()));
        assert!(args.contains(&"--sudoflags".to_string()));
        assert!(args.contains(&"-A".to_string()));
        assert!(args.contains(&"--noconfirm".to_string()));
        assert!(args.contains(&"google-chrome".to_string()));
        assert!(args.contains(&"visual-studio-code-bin".to_string()));

        // Paru with skip_review = false
        let (cmd_no_skip, args_no_skip) = InstallDialog::build_helper_args("paru", &pkgs, false);
        assert_eq!(cmd_no_skip, "paru");
        assert!(!args_no_skip.contains(&"--skipreview".to_string()));
        assert!(args_no_skip.contains(&"--sudoflags".to_string()));

        // Yay with skip_review = true
        let (cmd_yay, args_yay) = InstallDialog::build_helper_args("yay", &pkgs, true);
        assert_eq!(cmd_yay, "yay");
        assert!(args_yay.contains(&"--answeredit".to_string()));
        assert!(args_yay.contains(&"None".to_string()));
        assert!(args_yay.contains(&"--answerclean".to_string()));

        // Yay with skip_review = false
        let (cmd_yay_no_skip, args_yay_no_skip) = InstallDialog::build_helper_args("yay", &pkgs, false);
        assert_eq!(cmd_yay_no_skip, "yay");
        assert!(!args_yay_no_skip.contains(&"--answeredit".to_string()));
        assert!(!args_yay_no_skip.contains(&"--answerclean".to_string()));
    }

    #[test]
    fn test_build_system_upgrade_args() {
        use aurora::ui::widgets::InstallDialog;

        let (cmd_paru, args_paru) = InstallDialog::build_system_upgrade_args("paru", true);
        assert_eq!(cmd_paru, "paru");
        assert!(args_paru.contains(&"-Syu".to_string()));
        assert!(args_paru.contains(&"--skipreview".to_string()));

        let (cmd_paru_rev, args_paru_rev) = InstallDialog::build_system_upgrade_args("paru", false);
        assert_eq!(cmd_paru_rev, "paru");
        assert!(!args_paru_rev.contains(&"--skipreview".to_string()));

        let (cmd_yay, args_yay) = InstallDialog::build_system_upgrade_args("yay", true);
        assert_eq!(cmd_yay, "yay");
        assert!(args_yay.contains(&"--answeredit".to_string()));

        let (cmd_yay_rev, args_yay_rev) = InstallDialog::build_system_upgrade_args("yay", false);
        assert_eq!(cmd_yay_rev, "yay");
        assert!(!args_yay_rev.contains(&"--answeredit".to_string()));

        let (cmd_pacman, args_pacman) = InstallDialog::build_system_upgrade_args("pacman", true);
        assert_eq!(cmd_pacman, "sudo");
        assert!(args_pacman.contains(&"pacman".to_string()));
        assert!(args_pacman.contains(&"-Syu".to_string()));
    }
}
