#[cfg(test)]
mod tests {
    use aurora::state::SettingsState;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn test_settings_state_defaults_and_mutation() {
        let state = SettingsState::new();
        let initial = state.get();

        assert!(initial.require_summary_confirmation);

        // Değer değiştirme
        state.set_dark_theme(false);
        assert!(!state.get().dark_theme);

        state.set_dark_theme(true);
        assert!(state.get().dark_theme);

        state.set_aur_helper("yay");
        assert_eq!(state.get().aur_helper, "yay");

        state.set_aur_helper("paru");
        assert_eq!(state.get().aur_helper, "paru");
    }

    #[test]
    fn test_settings_listeners() {
        let state = SettingsState::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();

        state.on_change(move |_data| {
            called_clone.set(true);
        });

        state.set_require_pkgbuild_review(false);
        assert!(called.get());
        assert!(!state.get().require_pkgbuild_review);
    }

    #[test]
    fn test_installed_state_basic() {
        let state = aurora::state::InstalledState::new();
        assert!(!state.is_installed("package-that-does-not-exist-ever-12345"));
    }

    #[test]
    fn test_package_update_model() {
        let update = aurora::package_managers::pacman::PackageUpdate {
            name: "linux".to_string(),
            old_version: "6.8.1.arch1-1".to_string(),
            new_version: "6.8.2.arch1-1".to_string(),
        };
        assert_eq!(update.name, "linux");
        assert_eq!(update.old_version, "6.8.1.arch1-1");
        assert_eq!(update.new_version, "6.8.2.arch1-1");
    }

    #[test]
    fn test_config_constants() {
        use aurora::config::{APP_CONTRIBUTING_URL, APP_DEV_STATUS, APP_GITHUB_URL, APP_ID, APP_ISSUES_URL, APP_NAME};
        assert_eq!(APP_ID, "org.aurora.ApplicationCenter");
        assert_eq!(APP_NAME, "Aurora");
        assert!(APP_GITHUB_URL.starts_with("https://github.com/"));
        assert!(APP_ISSUES_URL.ends_with("/issues"));
        assert!(APP_CONTRIBUTING_URL.ends_with("/CONTRIBUTING.md"));
        assert!(APP_DEV_STATUS.contains("Aktif Geliştirme"));
    }
}
