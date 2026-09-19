use crate::models::{AppCategory, AppItem, PackageSource};
use crate::package_managers::{AurManager, PacmanManager};
use crate::services::CatalogService;
use crate::state::{CartState, InstalledState};
use crate::ui::widgets::AppCard;
use gtk4::prelude::*;
use gtk4::{Box, Button, CheckButton, Label, Orientation, ScrolledWindow, Spinner, Align};
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CatalogPage;

impl CatalogPage {
    pub fn build<F>(
        cart_state: CartState,
        installed_state: InstalledState,
        on_detail_clicked: F,
    ) -> (ScrolledWindow, Rc<dyn Fn(Option<AppCategory>, &str)>)
    where
        F: Fn(AppItem) + Clone + 'static,
    {
        let root = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(16)
            .margin_start(24)
            .margin_end(24)
            .margin_top(20)
            .margin_bottom(32)
            .build();

        let header_row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();

        let title_label = Label::builder()
            .label("Uygulama Kataloğu")
            .halign(Align::Start)
            .css_classes(["title-1"])
            .hexpand(true)
            .build();
        header_row.append(&title_label);

        let count_label = Label::builder()
            .label("")
            .halign(Align::End)
            .css_classes(["dim-label"])
            .build();
        header_row.append(&count_label);
        root.append(&header_row);

        let current_category: Rc<RefCell<Option<AppCategory>>> = Rc::new(RefCell::new(None));
        let current_query: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

        let apps_container = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .build();

        let empty_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(16)
            .margin_top(32)
            .margin_bottom(32)
            .visible(false)
            .build();

        let status_page = adw::StatusPage::builder()
            .icon_name("system-search-symbolic")
            .title("Katalogda Bulunamadı")
            .description("Bu uygulama yerel katalogda yok. Resmi depolarda ve AUR'da canlı aramak ister misiniz?")
            .build();
        empty_box.append(&status_page);

        let live_search_btn = Button::builder()
            .label("Tüm Depolarda Canlı Ara (Pacman ve AUR)")
            .icon_name("system-search-symbolic")
            .css_classes(["suggested-action", "pill"])
            .halign(Align::Center)
            .build();
        empty_box.append(&live_search_btn);

        let live_spinner = Spinner::builder()
            .spinning(true)
            .width_request(24)
            .height_request(24)
            .halign(Align::Center)
            .visible(false)
            .build();
        empty_box.append(&live_spinner);

        let refresh_list = {
            let apps_container = apps_container.clone();
            let empty_box = empty_box.clone();
            let count_label = count_label.clone();
            let cart_state = cart_state.clone();
            let installed_state = installed_state.clone();
            let on_detail_clicked = on_detail_clicked.clone();
            let current_category = current_category.clone();
            let current_query = current_query.clone();

            Rc::new(move |new_cat: Option<AppCategory>, new_query: &str| {
                *current_category.borrow_mut() = new_cat;
                *current_query.borrow_mut() = new_query.to_string();

                while let Some(child) = apps_container.first_child() {
                    apps_container.remove(&child);
                }

                let q = current_query.borrow().trim().to_lowercase();
                let cat = *current_category.borrow();

                let all = CatalogService::get_all_apps();
                let filtered: Vec<AppItem> = all
                    .into_iter()
                    .filter(|app| {
                        let matches_cat = match cat {
                            Some(c) => app.category == c,
                            None => true,
                        };
                        let matches_q = if q.is_empty() {
                            true
                        } else {
                            app.name.to_lowercase().contains(&q)
                                || app.package_name.to_lowercase().contains(&q)
                                || app.description.to_lowercase().contains(&q)
                                || app.tags.iter().any(|t| t.to_lowercase().contains(&q))
                        };
                        matches_cat && matches_q
                    })
                    .collect();

                let total = filtered.len();
                count_label.set_label(&format!("{} uygulama listelendi", total));

                if total == 0 && !q.is_empty() {
                    empty_box.set_visible(true);
                    apps_container.set_visible(false);
                } else {
                    empty_box.set_visible(false);
                    apps_container.set_visible(true);
                    for item in filtered {
                        let card = AppCard::new(item, cart_state.clone(), installed_state.clone(), on_detail_clicked.clone());
                        apps_container.append(&card);
                    }
                }
            })
        };

        let current_query_for_live = current_query.clone();
        let apps_container_for_live = apps_container.clone();
        let empty_box_for_live = empty_box.clone();
        let live_search_btn_clone = live_search_btn.clone();
        let live_spinner_clone = live_spinner.clone();
        let cart_state_for_live = cart_state.clone();
        let installed_state_for_live = installed_state.clone();
        let on_detail_for_live = on_detail_clicked.clone();
        let count_label_for_live = count_label.clone();

        live_search_btn.connect_clicked(move |_| {
            let q = current_query_for_live.borrow().clone();
            if q.is_empty() {
                return;
            }

            live_search_btn_clone.set_sensitive(false);
            live_spinner_clone.set_visible(true);

            let container = apps_container_for_live.clone();
            let empty_b = empty_box_for_live.clone();
            let btn = live_search_btn_clone.clone();
            let spin = live_spinner_clone.clone();
            let cart = cart_state_for_live.clone();
            let inst = installed_state_for_live.clone();
            let on_detail = on_detail_for_live.clone();
            let count_lbl = count_label_for_live.clone();

            glib::spawn_future_local(async move {
                let mut found_items = Vec::new();

                if let Ok(pacman_results) = PacmanManager::search(&q).await {
                    for r in pacman_results.into_iter().take(20) {
                        found_items.push(AppItem {
                            id: format!("official.{}", r.name),
                            name: r.name.clone(),
                            package_name: r.name.clone(),
                            description: r.description,
                            category: AppCategory::System,
                            source: PackageSource::Official,
                            icon: "application-x-executable".into(),
                            homepage: None,
                            license: None,
                            tags: vec!["repo".into(), r.repo],
                        });
                    }
                }

                if let Ok(aur_results) = AurManager::search(&q).await {
                    for r in aur_results.into_iter().take(15) {
                        found_items.push(AppItem {
                            id: format!("aur.{}", r.name),
                            name: r.name.clone(),
                            package_name: r.name.clone(),
                            description: r.description,
                            category: AppCategory::System,
                            source: PackageSource::Aur,
                            icon: "application-x-executable".into(),
                            homepage: None,
                            license: None,
                            tags: vec!["aur".into()],
                        });
                    }
                }

                spin.set_visible(false);
                btn.set_sensitive(true);

                if !found_items.is_empty() {
                    empty_b.set_visible(false);
                    container.set_visible(true);
                    count_lbl.set_label(&format!("Canlı depolardan {} sonuç bulundu", found_items.len()));

                    while let Some(child) = container.first_child() {
                        container.remove(&child);
                    }

                    for item in found_items {
                        let card = AppCard::new(item, cart.clone(), inst.clone(), on_detail.clone());
                        container.append(&card);
                    }
                } else {
                    count_lbl.set_label("Depolarda eşleşen paket bulunamadı");
                }
            });
        });

        let cat_bar = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        let all_btn = CheckButton::builder()
            .label("Tümü")
            .active(true)
            .build();
        all_btn.add_css_class("selection-mode");

        let refresh_all = refresh_list.clone();
        let query_clone = current_query.clone();
        all_btn.connect_toggled(move |btn| {
            if btn.is_active() {
                let q = query_clone.borrow().clone();
                refresh_all(None, &q);
            }
        });
        cat_bar.append(&all_btn);

        let mut previous_btn = all_btn.clone();
        for cat in AppCategory::all() {
            let cat_btn = CheckButton::builder()
                .label(cat.title())
                .group(&previous_btn)
                .build();
            cat_btn.add_css_class("selection-mode");

            let refresh_cat = refresh_list.clone();
            let c = *cat;
            let query_clone = current_query.clone();
            cat_btn.connect_toggled(move |btn| {
                if btn.is_active() {
                    let q = query_clone.borrow().clone();
                    refresh_cat(Some(c), &q);
                }
            });

            cat_bar.append(&cat_btn);
            previous_btn = cat_btn;
        }

        let cat_scroller = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Never)
            .child(&cat_bar)
            .build();
        root.append(&cat_scroller);
        root.append(&empty_box);
        root.append(&apps_container);

        refresh_list(None, "");

        let scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&root)
            .build();

        (scroll, refresh_list)
    }
}
