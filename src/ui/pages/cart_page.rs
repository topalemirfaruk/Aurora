use crate::models::{AppItem, PackageSource};
use crate::state::CartState;
use crate::utils::SystemCapabilities;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow, Align};
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CartPage;

impl CartPage {
    pub fn build<I>(
        cart_state: CartState,
        settings_state: crate::state::SettingsState,
        on_install: I,
    ) -> (ScrolledWindow, Rc<dyn Fn()>)
    where
        I: Fn(Vec<AppItem>) + 'static,
    {
        let root = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(20)
            .margin_start(24)
            .margin_end(24)
            .margin_top(20)
            .margin_bottom(32)
            .build();

        // Başlık alanı
        let header = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();

        let title = Label::builder()
            .label("Kurulum Sepeti")
            .halign(Align::Start)
            .css_classes(["title-1"])
            .hexpand(true)
            .build();
        header.append(&title);

        let clear_btn = Button::builder()
            .label("Sepeti Temizle")
            .css_classes(["flat", "destructive-action"])
            .build();
        header.append(&clear_btn);
        root.append(&header);

        // Boş sepet sayfası
        let empty_state = adw::StatusPage::builder()
            .icon_name(crate::utils::IconResolver::cart_icon())
            .title("Sepetiniz Boş")
            .description("Katalogdan veya ana sayfadan kurmak istediğiniz uygulamaları seçip sepete ekleyebilirsiniz.")
            .build();

        let content_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(16)
            .build();

        // Özet ve Kurulum Eylem Kutusu
        let summary_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(16)
            .css_classes(["aurora-card"])
            .build();

        let summary_text_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .hexpand(true)
            .build();

        let summary_label = Label::builder()
            .label("Seçilen Paketler Özeti")
            .halign(Align::Start)
            .css_classes(["heading"])
            .build();

        let breakdown_label = Label::builder()
            .label("")
            .halign(Align::Start)
            .css_classes(["dim-label"])
            .build();

        summary_text_box.append(&summary_label);
        summary_text_box.append(&breakdown_label);
        summary_box.append(&summary_text_box);

        // Canlı Kurulumu Başlat Butonu
        let start_install_btn = Button::builder()
            .label("🚀 Kurulumu Başlat")
            .valign(Align::Center)
            .css_classes(["suggested-action", "pill"])
            .build();

        let cart_clone_for_install = cart_state.clone();
        let on_install = Rc::new(on_install);
        let on_install_clone = on_install.clone();
        start_install_btn.connect_clicked(move |_| {
            let items = cart_clone_for_install.items();
            if !items.is_empty() {
                on_install_clone(items);
            }
        });

        summary_box.append(&start_install_btn);
        content_box.append(&summary_box);

        // Komut Önizleme Kutusu
        let cmd_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .css_classes(["aurora-card"])
            .build();

        let cmd_header = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        let cmd_title = Label::builder()
            .label("Oluşturulan Kurulum Komutları")
            .halign(Align::Start)
            .css_classes(["heading"])
            .hexpand(true)
            .build();

        let copy_btn = Button::builder()
            .label("📋 Kopyala")
            .css_classes(["suggested-action"])
            .build();

        cmd_header.append(&cmd_title);
        cmd_header.append(&copy_btn);
        cmd_box.append(&cmd_header);

        let cmd_text_label = Label::builder()
            .label("")
            .selectable(true)
            .wrap(true)
            .halign(Align::Start)
            .css_classes(["monospace"])
            .margin_top(6)
            .build();

        cmd_box.append(&cmd_text_label);
        content_box.append(&cmd_box);

        // Paket Listesi Başlığı
        let list_title = Label::builder()
            .label("Sepetteki Uygulamalar")
            .halign(Align::Start)
            .css_classes(["title-3"])
            .margin_top(8)
            .build();
        content_box.append(&list_title);

        let items_container = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .build();
        content_box.append(&items_container);

        root.append(&empty_state);
        root.append(&content_box);

        // Kopyalama eylemi
        let current_commands = Rc::new(RefCell::new(String::new()));
        let current_commands_copy = current_commands.clone();
        let copy_btn_clone = copy_btn.clone();
        copy_btn.connect_clicked(move |_| {
            let text = current_commands_copy.borrow().clone();
            if let Some(display) = Display::default() {
                let clipboard = display.clipboard();
                clipboard.set_text(&text);
                copy_btn_clone.set_label("Kopyalandı! ✓");
                let btn = copy_btn_clone.clone();
                glib::timeout_add_local_once(std::time::Duration::from_millis(1500), move || {
                    btn.set_label("📋 Kopyala");
                });
            }
        });

        // Temizleme eylemi
        let cart_state_for_clear = cart_state.clone();
        clear_btn.connect_clicked(move |_| {
            cart_state_for_clear.clear();
        });

        // Yenileme fonksiyonu
        let refresh = {
            let cart_state = cart_state.clone();
            let empty_state = empty_state.clone();
            let content_box = content_box.clone();
            let breakdown_label = breakdown_label.clone();
            let cmd_text_label = cmd_text_label.clone();
            let items_container = items_container.clone();
            let current_commands = current_commands.clone();
            let settings_state = settings_state.clone();

            Rc::new(move || {
                let items = cart_state.items();
                let count = items.len();

                if count == 0 {
                    empty_state.set_visible(true);
                    content_box.set_visible(false);
                    current_commands.borrow_mut().clear();
                    return;
                }

                empty_state.set_visible(false);
                content_box.set_visible(true);

                // Çocukları temizle
                while let Some(child) = items_container.first_child() {
                    items_container.remove(&child);
                }

                let mut official_pkgs = Vec::new();
                let mut aur_pkgs = Vec::new();
                let mut flatpak_pkgs = Vec::new();

                for item in &items {
                    match item.source {
                        PackageSource::Official => official_pkgs.push(item.package_name.clone()),
                        PackageSource::Aur => aur_pkgs.push(item.package_name.clone()),
                        PackageSource::Flatpak => flatpak_pkgs.push(item.package_name.clone()),
                    }

                    // Her sepet öğesi için satır
                    let row = Box::builder()
                        .orientation(Orientation::Horizontal)
                        .spacing(12)
                        .css_classes(["aurora-card"])
                        .build();

                    let resolved_icon = crate::utils::IconResolver::resolve(&item.icon, item.category);
                    let icon = gtk4::Image::from_icon_name(&resolved_icon);
                    icon.set_pixel_size(32);
                    row.append(&icon);

                    let info_box = Box::builder()
                        .orientation(Orientation::Vertical)
                        .spacing(2)
                        .hexpand(true)
                        .build();

                    let name_label = Label::builder()
                        .label(&item.name)
                        .halign(Align::Start)
                        .css_classes(["heading"])
                        .build();

                    let meta_label = Label::builder()
                        .label(&format!("{} • Paket: {}", item.source.badge_label(), item.package_name))
                        .halign(Align::Start)
                        .css_classes(["caption", "dim-label"])
                        .build();

                    info_box.append(&name_label);
                    info_box.append(&meta_label);
                    row.append(&info_box);

                    let remove_btn = Button::builder()
                        .icon_name("user-trash-symbolic")
                        .css_classes(["flat", "destructive-action", "circular"])
                        .tooltip_text("Sepetten kaldır")
                        .build();

                    let cart_clone = cart_state.clone();
                    let pkg_to_remove = item.package_name.clone();
                    remove_btn.connect_clicked(move |_| {
                        cart_clone.remove(&pkg_to_remove);
                    });

                    row.append(&remove_btn);
                    items_container.append(&row);
                }

                breakdown_label.set_label(&format!(
                    "Toplam: {} uygulama (Resmi: {}, AUR: {}, Flatpak: {})",
                    count,
                    official_pkgs.len(),
                    aur_pkgs.len(),
                    flatpak_pkgs.len()
                ));

                // Komutları oluştur
                let sys = SystemCapabilities::detect();
                let configured_helper = settings_state.get().aur_helper;
                let aur_helper = if configured_helper.is_empty() {
                    sys.preferred_aur_helper().unwrap_or("paru")
                } else {
                    &configured_helper
                };

                let mut cmd_lines = Vec::new();

                if !official_pkgs.is_empty() {
                    cmd_lines.push(format!("sudo pacman -S --needed {}", official_pkgs.join(" ")));
                }

                if !aur_pkgs.is_empty() {
                    cmd_lines.push(format!("{} -S --needed {}", aur_helper, aur_pkgs.join(" ")));
                }

                if !flatpak_pkgs.is_empty() {
                    cmd_lines.push(format!("flatpak install flathub {}", flatpak_pkgs.join(" ")));
                }

                let final_cmd = cmd_lines.join("\n");
                *current_commands.borrow_mut() = final_cmd.clone();
                cmd_text_label.set_label(&final_cmd);
            })
        };

        // Sepet değiştikçe otomatik yenile
        let refresh_on_change = refresh.clone();
        cart_state.on_change(move |_| {
            refresh_on_change();
        });

        // Ayarlar değiştikçe otomatik yenile
        let refresh_on_settings = refresh.clone();
        settings_state.on_change(move |_| {
            refresh_on_settings();
        });

        // İlk tetikleme
        refresh();

        let scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&root)
            .build();

        (scroll, refresh)
    }
}
