use crate::package_managers::PacmanManager;
use crate::ui::widgets::UninstallDialog;
use gtk4::prelude::*;
use gtk4::{
    Box, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, SearchEntry, Spinner, Align
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct InstalledPage;

impl InstalledPage {
    pub fn build(
        installed_state: crate::state::InstalledState,
        settings_state: crate::state::SettingsState,
    ) -> ScrolledWindow {
        let root = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(16)
            .margin_start(24)
            .margin_end(24)
            .margin_top(20)
            .margin_bottom(32)
            .build();

        let header = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();

        let title = Label::builder()
            .label("Kurulu Uygulamalar & Paket Kaldırma")
            .halign(Align::Start)
            .css_classes(["title-1"])
            .hexpand(true)
            .build();
        header.append(&title);

        let count_badge = Label::builder()
            .label("Taranıyor...")
            .css_classes(["status-pill"])
            .build();
        header.append(&count_badge);

        let refresh_btn = Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Kurulu paketleri ve güncellemeleri yeniden tara")
            .css_classes(["flat", "circular"])
            .build();
        header.append(&refresh_btn);
        root.append(&header);

        let updates_card = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(16)
            .css_classes(["aurora-card"])
            .build();

        let update_icon = gtk4::Image::from_icon_name("software-update-available-symbolic");
        update_icon.set_pixel_size(32);
        updates_card.append(&update_icon);

        let update_text_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .build();

        let update_title = Label::builder()
            .label("Sistem Güncelleme Denetimi")
            .halign(Align::Start)
            .css_classes(["heading"])
            .build();

        let update_subtitle = Label::builder()
            .label("Güncellemeler denetleniyor...")
            .halign(Align::Start)
            .css_classes(["caption", "dim-label"])
            .build();

        update_text_box.append(&update_title);
        update_text_box.append(&update_subtitle);
        updates_card.append(&update_text_box);

        let upgrade_btn = Button::builder()
            .label("Sistemi Güncelle")
            .icon_name("software-update-available-symbolic")
            .css_classes(["suggested-action", "pill"])
            .valign(Align::Center)
            .visible(false)
            .build();
        updates_card.append(&upgrade_btn);

        root.append(&updates_card);

        let search_entry = SearchEntry::builder()
            .placeholder_text("Kurulu paketleri filtrele veya kaldırmak için ara...")
            .build();
        root.append(&search_entry);

        let spinner_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .valign(Align::Center)
            .halign(Align::Center)
            .margin_top(40)
            .margin_bottom(40)
            .build();

        let spinner = Spinner::builder()
            .spinning(true)
            .width_request(32)
            .height_request(32)
            .build();

        let loading_label = Label::builder()
            .label("Sistem paketleri taranıyor...")
            .css_classes(["dim-label"])
            .build();

        spinner_box.append(&spinner);
        spinner_box.append(&loading_label);
        root.append(&spinner_box);

        let list_box = ListBox::builder()
            .css_classes(["boxed-list"])
            .selection_mode(gtk4::SelectionMode::None)
            .visible(false)
            .build();
        root.append(&list_box);

        let all_packages = Rc::new(RefCell::new(Vec::<crate::package_managers::pacman::InstalledPackage>::new()));
        let load_data_ref: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));

        let filter_list = {
            let list_box = list_box.clone();
            let all_packages = all_packages.clone();
            let load_data_ref = load_data_ref.clone();
            let inst_for_filter = installed_state.clone();

            Rc::new(move |query: &str| {
                while let Some(child) = list_box.first_child() {
                    list_box.remove(&child);
                }

                let q = query.trim().to_lowercase();
                let pkgs = all_packages.borrow();
                let filtered = pkgs.iter().filter(|p| {
                    if q.is_empty() {
                        true
                    } else {
                        p.name.to_lowercase().contains(&q)
                    }
                });

                for pkg in filtered.take(100) {
                    let row = ListBoxRow::new();
                    let row_box = Box::builder()
                        .orientation(Orientation::Horizontal)
                        .spacing(12)
                        .margin_start(16)
                        .margin_end(16)
                        .margin_top(10)
                        .margin_bottom(10)
                        .build();

                    let icon = gtk4::Image::from_icon_name("application-x-executable");
                    row_box.append(&icon);

                    let text_box = Box::builder()
                        .orientation(Orientation::Vertical)
                        .spacing(2)
                        .hexpand(true)
                        .build();

                    let name_label = Label::builder()
                        .label(&pkg.name)
                        .halign(Align::Start)
                        .css_classes(["heading"])
                        .build();

                    let ver_label = Label::builder()
                        .label(&format!("Sürüm: {}", pkg.version))
                        .halign(Align::Start)
                        .css_classes(["caption", "dim-label"])
                        .build();

                    text_box.append(&name_label);
                    text_box.append(&ver_label);
                    row_box.append(&text_box);

                    let remove_btn = Button::builder()
                        .label("Kaldır")
                        .css_classes(["flat", "destructive-action"])
                        .tooltip_text("Bu paketi sistemden kaldır")
                        .build();

                    let pkg_name = pkg.name.clone();
                    let load_data_for_success = load_data_ref.clone();
                    let inst_refresh = inst_for_filter.clone();

                    remove_btn.connect_clicked(move |btn| {
                        if let Some(root_win) = btn.root().and_downcast::<gtk4::Window>() {
                            let load_fn = load_data_for_success.borrow().clone();
                            let inst = inst_refresh.clone();
                            UninstallDialog::show(&root_win, &pkg_name, move || {
                                if let Some(ref f) = load_fn {
                                    f();
                                }
                                inst.refresh_background();
                            });
                        }
                    });

                    row_box.append(&remove_btn);
                    row.set_child(Some(&row_box));
                    list_box.append(&row);
                }
            })
        };

        let update_sub_for_load = update_subtitle.clone();
        let upgrade_btn_for_load = upgrade_btn.clone();

        let load_data = {
            let spinner_box = spinner_box.clone();
            let list_box = list_box.clone();
            let count_badge = count_badge.clone();
            let all_packages = all_packages.clone();
            let filter_list = filter_list.clone();
            let search_entry = search_entry.clone();

            Rc::new(move || {
                spinner_box.set_visible(true);
                list_box.set_visible(false);
                count_badge.set_label("Taranıyor...");

                let spinner_box = spinner_box.clone();
                let list_box = list_box.clone();
                let count_badge = count_badge.clone();
                let all_packages = all_packages.clone();
                let filter_list = filter_list.clone();
                let search_entry = search_entry.clone();

                let update_sub_clone = update_sub_for_load.clone();
                let upgrade_btn_clone = upgrade_btn_for_load.clone();

                glib::spawn_future_local(async move {
                    if let Ok(pkgs) = PacmanManager::list_installed().await {
                        let total = pkgs.len();
                        *all_packages.borrow_mut() = pkgs;
                        count_badge.set_label(&format!("{} paket kurulu", total));
                        filter_list(&search_entry.text());
                    } else {
                        count_badge.set_label("Tarama başarısız");
                    }

                    spinner_box.set_visible(false);
                    list_box.set_visible(true);

                    match PacmanManager::check_updates().await {
                        Ok(updates) if !updates.is_empty() => {
                            update_sub_clone.set_label(&format!("{} adet sistem ve AUR paketi için güncelleme mevcut.", updates.len()));
                            upgrade_btn_clone.set_visible(true);
                        }
                        Ok(_) => {
                            update_sub_clone.set_label("Sisteminiz güncel. Bekleyen paket güncellemesi yok.");
                            upgrade_btn_clone.set_visible(false);
                        }
                        Err(_) => {
                            update_sub_clone.set_label("Güncelleme durumu kontrol edilemedi.");
                            upgrade_btn_clone.set_visible(false);
                        }
                    }
                });
            })
        };

        *load_data_ref.borrow_mut() = Some(load_data.clone());

        let load_data_for_upgrade = load_data_ref.clone();
        let settings_for_upgrade = settings_state.clone();
        let inst_for_upgrade = installed_state.clone();
        upgrade_btn.connect_clicked(move |btn| {
            if let Some(root_win) = btn.root().and_downcast::<gtk4::Window>() {
                let load_fn = load_data_for_upgrade.borrow().clone();
                let inst = inst_for_upgrade.clone();
                crate::ui::widgets::InstallDialog::show_system_upgrade(
                    &root_win,
                    settings_for_upgrade.clone(),
                    move || {
                        if let Some(ref f) = load_fn {
                            f();
                        }
                        inst.refresh_background();
                    },
                );
            }
        });

        let filter_for_search = filter_list.clone();
        search_entry.connect_search_changed(move |entry| {
            filter_for_search(&entry.text());
        });

        let load_for_btn = load_data.clone();
        let inst_for_refresh_btn = installed_state.clone();
        refresh_btn.connect_clicked(move |_| {
            load_for_btn();
            inst_for_refresh_btn.refresh_background();
        });

        load_data();

        ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&root)
            .build()
    }
}
