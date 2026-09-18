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
}
