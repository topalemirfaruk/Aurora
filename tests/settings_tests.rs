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
}
