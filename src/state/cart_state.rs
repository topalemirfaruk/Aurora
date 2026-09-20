use crate::models::AppItem;
use std::cell::RefCell;
use std::rc::Rc;

type CartListener = Box<dyn Fn(usize) + 'static>;

#[derive(Clone, Default)]
pub struct CartState {
    items: Rc<RefCell<Vec<AppItem>>>,
    listeners: Rc<RefCell<Vec<CartListener>>>,
}

impl CartState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&self, item: AppItem) {
        let mut items = self.items.borrow_mut();
        if !items.iter().any(|i| i.package_name == item.package_name) {
            items.push(item);
            let count = items.len();
            drop(items);
            self.notify(count);
        }
    }

    pub fn remove(&self, package_name: &str) {
        let mut items = self.items.borrow_mut();
        if let Some(pos) = items.iter().position(|i| i.package_name == package_name) {
            items.remove(pos);
            let count = items.len();
            drop(items);
            self.notify(count);
        }
    }

    pub fn toggle(&self, item: AppItem) {
        if self.contains(&item.package_name) {
            self.remove(&item.package_name);
        } else {
            self.add(item);
        }
    }

    pub fn contains(&self, package_name: &str) -> bool {
        self.items.borrow().iter().any(|i| i.package_name == package_name)
    }

    pub fn items(&self) -> Vec<AppItem> {
        self.items.borrow().clone()
    }

    pub fn count(&self) -> usize {
        self.items.borrow().len()
    }

    pub fn clear(&self) {
        let mut items = self.items.borrow_mut();
        items.clear();
        drop(items);
        self.notify(0);
    }

    pub fn export_list(&self) -> String {
        let items = self.items.borrow();
        items
            .iter()
            .map(|i| i.package_name.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn import_from_text(&self, text: &str) -> usize {
        let mut added = 0;
        let catalog_apps = crate::services::CatalogService::get_all_apps();

        for line in text.lines() {
            let pkg = line.trim();
            if pkg.is_empty() || pkg.starts_with('#') {
                continue;
            }
            if !crate::security::is_valid_package_name(pkg) {
                continue;
            }
            if self.contains(pkg) {
                continue;
            }

            let item = if let Some(cat_app) = catalog_apps.iter().find(|a| a.package_name == pkg) {
                cat_app.clone()
            } else {
                let icon_name = crate::utils::IconResolver::resolve(pkg, crate::models::AppCategory::System);
                AppItem {
                    id: format!("imported.{}", pkg),
                    name: pkg.to_string(),
                    package_name: pkg.to_string(),
                    description: format!("İçe aktarılan sistem paketi ({})", pkg),
                    category: crate::models::AppCategory::System,
                    source: crate::models::PackageSource::Official,
                    icon: icon_name,
                    homepage: None,
                    license: None,
                    tags: vec!["imported".into()],
                }
            };

            self.add(item);
            added += 1;
        }
        added
    }

    pub fn on_change<F: Fn(usize) + 'static>(&self, callback: F) {
        self.listeners.borrow_mut().push(Box::new(callback));
    }

    fn notify(&self, count: usize) {
        for listener in self.listeners.borrow().iter() {
            listener(count);
        }
    }
}
