use crate::models::{AppCategory, AppItem};
use crate::services::CatalogService;
use crate::state::{CartState, InstalledState};
use crate::ui::widgets::AppCard;
use crate::utils::SystemCapabilities;
use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Grid, Label, Orientation, ScrolledWindow};

pub struct HomePage;

impl HomePage {
    pub fn build<F, C>(
        cart_state: CartState,
        installed_state: InstalledState,
        on_detail_clicked: F,
        on_category_clicked: C,
    ) -> ScrolledWindow
    where
        F: Fn(AppItem) + Clone + 'static,
        C: Fn(AppCategory) + Clone + 'static,
    {
        let container = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(24)
            .margin_start(24)
            .margin_end(24)
            .margin_top(24)
            .margin_bottom(32)
            .build();

        let hero_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .css_classes(["aurora-hero"])
            .build();

        let hero_title = Label::builder()
            .label("Aurora Uygulama Merkezi")
            .halign(Align::Start)
            .css_classes(["aurora-hero-title"])
            .build();

        let hero_sub = Label::builder()
            .label("Arch Linux için modern, hızlı ve güvenli uygulama keşif ve kurulum merkezi.")
            .halign(Align::Start)
            .css_classes(["aurora-hero-subtitle"])
            .build();

        hero_box.append(&hero_title);
        hero_box.append(&hero_sub);

        let sys = SystemCapabilities::detect();
        let sys_row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .margin_top(8)
            .build();

        let pacman_pill = Label::builder()
            .label(if sys.has_pacman { "Pacman Aktif" } else { "Pacman Yok" })
            .css_classes(["status-pill"])
            .build();
        sys_row.append(&pacman_pill);

        if let Some(helper) = sys.preferred_aur_helper() {
            let aur_pill = Label::builder()
                .label(format!("AUR: {}", helper))
                .css_classes(["status-pill"])
                .build();
            sys_row.append(&aur_pill);
        }

        if sys.has_flatpak {
            let flatpak_pill = Label::builder()
                .label("Flatpak Mevcut")
                .css_classes(["status-pill"])
                .build();
            sys_row.append(&flatpak_pill);
        }

        hero_box.append(&sys_row);
        container.append(&hero_box);

        let cat_section_title = Label::builder()
            .label("Kategorilere Göz At")
            .halign(Align::Start)
            .css_classes(["title-2"])
            .build();
        container.append(&cat_section_title);

        let cat_flow_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();

        for cat in AppCategory::all() {
            let cat_btn = Button::builder()
                .css_classes(["card", "flat"])
                .build();

            let btn_content = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(8)
                .margin_start(12)
                .margin_end(12)
                .margin_top(10)
                .margin_bottom(10)
                .build();

            let icon = gtk4::Image::from_icon_name(cat.icon_name());
            let title = Label::new(Some(cat.title()));

            btn_content.append(&icon);
            btn_content.append(&title);
            cat_btn.set_child(Some(&btn_content));

            let on_cat = on_category_clicked.clone();
            let c = *cat;
            cat_btn.connect_clicked(move |_| {
                on_cat(c);
            });

            cat_flow_box.append(&cat_btn);
        }

        let cat_scroller = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Never)
            .child(&cat_flow_box)
            .build();
        container.append(&cat_scroller);

        let bundles_title_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .build();

        let bundles_title = Label::builder()
            .label("Hızlı Kurulum Paketleri")
            .halign(Align::Start)
            .css_classes(["title-2"])
            .build();

        let bundles_sub = Label::builder()
            .label("Temel multimedya, oyun uyumluluk, ses ve geliştirici ortamlarını tek tıkla topluca kurun.")
            .halign(Align::Start)
            .css_classes(["dim-label"])
            .build();

        bundles_title_box.append(&bundles_title);
        bundles_title_box.append(&bundles_sub);
        container.append(&bundles_title_box);

        let bundles_grid = Grid::builder()
            .column_spacing(16)
            .row_spacing(16)
            .column_homogeneous(true)
            .build();

        let bundles = CatalogService::get_package_bundles();
        for (i, bundle) in bundles.into_iter().enumerate() {
            let bundle_card = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(12)
                .css_classes(["aurora-card"])
                .build();

            let header_row = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(12)
                .valign(Align::Center)
                .build();

            let icon_img = gtk4::Image::builder()
                .icon_name(bundle.icon)
                .pixel_size(24)
                .build();
            let icon_box = Box::builder()
                .css_classes(["aurora-icon-frame"])
                .build();
            icon_box.append(&icon_img);
            header_row.append(&icon_box);

            let title_col = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(2)
                .hexpand(true)
                .build();

            let title_lbl = Label::builder()
                .label(bundle.title)
                .halign(Align::Start)
                .css_classes(["heading"])
                .build();
            title_col.append(&title_lbl);

            let count_badge = Label::builder()
                .label(format!("{} Paket", bundle.package_names.len()))
                .css_classes(["status-pill"])
                .halign(Align::Start)
                .build();
            title_col.append(&count_badge);

            header_row.append(&title_col);
            bundle_card.append(&header_row);

            let desc_lbl = Label::builder()
                .label(bundle.description)
                .halign(Align::Start)
                .wrap(true)
                .css_classes(["caption", "dim-label"])
                .build();
            bundle_card.append(&desc_lbl);

            let pkgs_summary = Label::builder()
                .label(format!("Paketler: {}", bundle.package_names.join(", ")))
                .halign(Align::Start)
                .wrap(true)
                .css_classes(["caption"])
                .build();
            bundle_card.append(&pkgs_summary);

            let btn = Button::builder()
                .css_classes(["suggested-action", "pill"])
                .margin_top(4)
                .build();

            let bundle_pkgs: Vec<String> = bundle.package_names.iter().map(|s| s.to_string()).collect();

            let update_btn = {
                let cart_state = cart_state.clone();
                let btn = btn.clone();
                let pkgs = bundle_pkgs.clone();
                let total_pkgs = pkgs.len();
                move || {
                    let in_cart_count = pkgs.iter().filter(|pkg| cart_state.contains(pkg)).count();
                    if in_cart_count == total_pkgs {
                        btn.set_label("Tümü Sepette");
                        btn.remove_css_class("suggested-action");
                        btn.add_css_class("flat");
                        btn.set_tooltip_text(Some("Bu paketteki tüm uygulamalar sepete eklenmiş"));
                    } else if in_cart_count > 0 {
                        btn.set_label(&format!("Kalanı Ekle ({}/{})", total_pkgs - in_cart_count, total_pkgs));
                        btn.add_css_class("suggested-action");
                        btn.remove_css_class("flat");
                        btn.set_tooltip_text(Some("Henüz sepette olmayan paketleri ekle"));
                    } else {
                        btn.set_label(&format!("Sepete Ekle ({} Paket)", total_pkgs));
                        btn.add_css_class("suggested-action");
                        btn.remove_css_class("flat");
                        btn.set_tooltip_text(Some("Paketteki tüm uygulamaları sepete ekle"));
                    }
                }
            };

            update_btn();

            {
                let update_btn_clone = update_btn.clone();
                cart_state.on_change(move |_| {
                    update_btn_clone();
                });
            }

            {
                let cart_state = cart_state.clone();
                let pkgs = bundle_pkgs.clone();
                btn.connect_clicked(move |_| {
                    let all_apps = CatalogService::get_all_apps();
                    for pkg in &pkgs {
                        if !cart_state.contains(pkg) {
                            if let Some(app) = all_apps.iter().find(|a| &a.package_name == pkg) {
                                cart_state.add(app.clone());
                            }
                        }
                    }
                });
            }

            bundle_card.append(&btn);

            let col = (i % 2) as i32;
            let row = (i / 2) as i32;
            bundles_grid.attach(&bundle_card, col, row, 1, 1);
        }

        container.append(&bundles_grid);

        let featured_title = Label::builder()
            .label("Öne Çıkan Uygulamalar")
            .halign(Align::Start)
            .css_classes(["title-2"])
            .build();
        container.append(&featured_title);

        let apps_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .build();

        let all_apps = CatalogService::get_all_apps();
        for app in all_apps.into_iter().take(6) {
            let card = AppCard::new(app, cart_state.clone(), installed_state.clone(), on_detail_clicked.clone());
            apps_box.append(&card);
        }

        container.append(&apps_box);

        ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&container)
            .build()
    }
}
