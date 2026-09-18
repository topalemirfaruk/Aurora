#[cfg(test)]
mod tests {
    use aurora::models::{AppCategory, AppItem, PackageSource};
    use aurora::state::CartState;
    use std::cell::Cell;
    use std::rc::Rc;

    fn mock_app(id: &str, pkg: &str, source: PackageSource) -> AppItem {
        AppItem {
            id: id.into(),
            name: id.into(),
            package_name: pkg.into(),
            description: "Test app".into(),
            category: AppCategory::Essentials,
            source,
            icon: "application-x-executable".into(),
            homepage: None,
            license: None,
            tags: vec!["test".into()],
        }
    }

    #[test]
    fn test_cart_add_remove_toggle() {
        let cart = CartState::new();
        let app1 = mock_app("app1", "pkg1", PackageSource::Official);
        let app2 = mock_app("app2", "pkg2", PackageSource::Aur);

        assert_eq!(cart.count(), 0);
        assert!(!cart.contains("pkg1"));

        // Ekleme
        cart.add(app1.clone());
        assert_eq!(cart.count(), 1);
        assert!(cart.contains("pkg1"));

        // Aynı paketi tekrar ekleme (idempotent olmalı)
        cart.add(app1.clone());
        assert_eq!(cart.count(), 1);

        // İkinci paketi ekleme
        cart.add(app2.clone());
        assert_eq!(cart.count(), 2);
        assert!(cart.contains("pkg2"));

        // Toggle ile app2 çıkarma
        cart.toggle(app2.clone());
        assert_eq!(cart.count(), 1);
        assert!(!cart.contains("pkg2"));

        // Temizleme
        cart.clear();
        assert_eq!(cart.count(), 0);
    }

    #[test]
    fn test_cart_listener_notification() {
        let cart = CartState::new();
        let app = mock_app("app", "pkg", PackageSource::Official);

        let notified_count = Rc::new(Cell::new(0));
        let notified_count_clone = notified_count.clone();

        cart.on_change(move |count| {
            notified_count_clone.set(count);
        });

        cart.add(app.clone());
        assert_eq!(notified_count.get(), 1);

        cart.remove("pkg");
        assert_eq!(notified_count.get(), 0);
    }
}
