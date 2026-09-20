#[cfg(test)]
mod tests {
    use aurora::models::{AppCategory, PackageSource};
    use aurora::services::CatalogService;

    #[test]
    fn test_catalog_not_empty() {
        let apps = CatalogService::get_all_apps();
        assert!(!apps.is_empty(), "Katalog en az bir uygulama içermelidir");
        assert!(apps.len() >= 10, "Katalogda zengin bir başlangıç seti olmalıdır");
    }

    #[test]
    fn test_catalog_categories_coverage() {
        let apps = CatalogService::get_all_apps();
        for cat in AppCategory::all() {
            let matches: Vec<_> = apps.iter().filter(|a| a.category == *cat).collect();
            assert!(!matches.is_empty(), "{:?} kategorisinde en az bir uygulama bulunmalıdır", cat);
        }
    }

    #[test]
    fn test_search_functionality() {
        let results = CatalogService::search("firefox");
        assert!(!results.is_empty(), "Firefox araması sonuç döndürmelidir");
        assert_eq!(results[0].package_name, "firefox");

        let aur_results = CatalogService::search("code");
        assert!(aur_results.iter().any(|a| a.source == PackageSource::Aur));
    }

    #[test]
    fn test_package_bundles_validity() {
        let bundles = CatalogService::get_package_bundles();
        assert_eq!(bundles.len(), 4, "4 adet paket seti bulunmalıdır");

        let all_apps = CatalogService::get_all_apps();

        for bundle in bundles {
            assert!(!bundle.id.is_empty());
            assert!(!bundle.title.is_empty());
            assert!(!bundle.description.is_empty());
            assert!(!bundle.icon.is_empty());
            assert!(!bundle.package_names.is_empty());

            // Paketteki her uygulamanın katalogda kayıtlı olduğunu doğrula
            for pkg_name in bundle.package_names {
                let found = all_apps.iter().any(|app| &app.package_name == pkg_name);
                assert!(found, "Paket setindeki '{}' paketi ana katalogda mevcut olmalıdır", pkg_name);
            }
        }
    }

    #[test]
    fn test_nvidia_dkms_drivers_in_catalog() {
        let all_apps = CatalogService::get_all_apps();
        let has_nvidia_dkms = all_apps.iter().any(|a| a.package_name == "nvidia-dkms");
        let has_nvidia_legacy = all_apps.iter().any(|a| a.package_name == "nvidia-470xx-dkms");
        let has_nvidia_settings = all_apps.iter().any(|a| a.package_name == "nvidia-settings");

        assert!(has_nvidia_dkms, "nvidia-dkms katalogda yer almalıdır");
        assert!(has_nvidia_legacy, "nvidia-470xx-dkms katalogda yer almalıdır");
        assert!(has_nvidia_settings, "nvidia-settings katalogda yer almalıdır");
    }
}
