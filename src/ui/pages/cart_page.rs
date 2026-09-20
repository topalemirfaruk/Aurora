use crate::models::{AppItem, PackageSource};
use crate::state::CartState;
use crate::utils::SystemCapabilities;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow, Align};
use libadwaita as adw;
use libadwaita::prelude::*;
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

        let import_btn = Button::builder()
            .label("İçe Aktar")
            .icon_name("document-open-symbolic")
            .css_classes(["flat"])
            .tooltip_text("Metin veya paket listesinden toplu içe aktar")
            .build();
        header.append(&import_btn);

        let export_btn = Button::builder()
            .label("Dışa Aktar")
            .icon_name("document-save-symbolic")
            .css_classes(["flat"])
            .tooltip_text("Sepetteki paketlerin listesini panoya veya metne aktar")
            .build();
        header.append(&export_btn);

        let clear_btn = Button::builder()
            .label("Sepeti Temizle")
            .css_classes(["flat", "destructive-action"])
            .build();
        header.append(&clear_btn);
        root.append(&header);

        let empty_state = adw::StatusPage::builder()
            .icon_name(crate::utils::IconResolver::cart_icon())
            .title("Sepetiniz Boş")
            .description("Katalogdan veya ana sayfadan kurmak istediğiniz uygulamaları seçip sepete ekleyebilirsiniz.")
            .build();

        let content_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(16)
            .build();

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

        let start_install_btn = Button::builder()
            .label("Kurulumu Başlat")
            .icon_name("system-software-install-symbolic")
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
            .label("Kopyala")
            .icon_name("edit-copy-symbolic")
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

        let current_commands = Rc::new(RefCell::new(String::new()));
        let current_commands_copy = current_commands.clone();
        let copy_btn_clone = copy_btn.clone();
        copy_btn.connect_clicked(move |_| {
            let text = current_commands_copy.borrow().clone();
            if let Some(display) = Display::default() {
                let clipboard = display.clipboard();
                clipboard.set_text(&text);
                copy_btn_clone.set_label("Kopyalandı");
                let btn = copy_btn_clone.clone();
                glib::timeout_add_local_once(std::time::Duration::from_millis(1500), move || {
                    btn.set_label("Kopyala");
                });
            }
        });

        let cart_state_for_clear = cart_state.clone();
        clear_btn.connect_clicked(move |_| {
            cart_state_for_clear.clear();
        });

        let cart_for_import = cart_state.clone();
        import_btn.connect_clicked(move |btn| {
            if let Some(root_win) = btn.root().and_downcast::<gtk4::Window>() {
                let dialog = adw::Window::builder()
                    .transient_for(&root_win)
                    .modal(true)
                    .title("Paket Listesi İçe Aktar")
                    .default_width(520)
                    .default_height(400)
                    .build();

                let d_root = Box::builder()
                    .orientation(Orientation::Vertical)
                    .spacing(0)
                    .build();

                let d_header = adw::HeaderBar::builder()
                    .show_end_title_buttons(true)
                    .build();
                d_root.append(&d_header);

                let d_content = Box::builder()
                    .orientation(Orientation::Vertical)
                    .spacing(12)
                    .margin_start(20)
                    .margin_end(20)
                    .margin_top(12)
                    .margin_bottom(20)
                    .build();

                let d_label = Label::builder()
                    .label("Paket adlarını her satıra bir tane gelecek şekilde girin:")
                    .halign(Align::Start)
                    .css_classes(["heading"])
                    .build();
                d_content.append(&d_label);

                let text_view = gtk4::TextView::builder()
                    .wrap_mode(gtk4::WrapMode::WordChar)
                    .monospace(true)
                    .build();

                let scroller = ScrolledWindow::builder()
                    .height_request(200)
                    .css_classes(["card"])
                    .child(&text_view)
                    .build();
                d_content.append(&scroller);

                let action_bar = Box::builder()
                    .orientation(Orientation::Horizontal)
                    .halign(Align::End)
                    .spacing(10)
                    .margin_top(8)
                    .build();

                let cancel_b = Button::builder().label("İptal").css_classes(["flat"]).build();
                let apply_b = Button::builder().label("Sepete Ekle").css_classes(["suggested-action", "pill"]).build();

                action_bar.append(&cancel_b);
                action_bar.append(&apply_b);
                d_content.append(&action_bar);
                d_root.append(&d_content);
                dialog.set_content(Some(&d_root));

                let d_close = dialog.clone();
                cancel_b.connect_clicked(move |_| {
                    d_close.close();
                });

                let d_apply = dialog.clone();
                let cart = cart_for_import.clone();
                let buffer = text_view.buffer();
                apply_b.connect_clicked(move |_| {
                    let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
                    cart.import_from_text(&text);
                    d_apply.close();
                });

                dialog.present();
            }
        });

        let cart_for_export = cart_state.clone();
        export_btn.connect_clicked(move |btn| {
            if let Some(root_win) = btn.root().and_downcast::<gtk4::Window>() {
                let export_text = cart_for_export.export_list();

                let dialog = adw::Window::builder()
                    .transient_for(&root_win)
                    .modal(true)
                    .title("Paket Listesi Dışa Aktar")
                    .default_width(520)
                    .default_height(400)
                    .build();

                let d_root = Box::builder()
                    .orientation(Orientation::Vertical)
                    .spacing(0)
                    .build();

                let d_header = adw::HeaderBar::builder()
                    .show_end_title_buttons(true)
                    .build();
                d_root.append(&d_header);

                let d_content = Box::builder()
                    .orientation(Orientation::Vertical)
                    .spacing(12)
                    .margin_start(20)
                    .margin_end(20)
                    .margin_top(12)
                    .margin_bottom(20)
                    .build();

                let d_label = Label::builder()
                    .label("Sepetteki paketlerin metin listesi:")
                    .halign(Align::Start)
                    .css_classes(["heading"])
                    .build();
                d_content.append(&d_label);

                let text_view = gtk4::TextView::builder()
                    .wrap_mode(gtk4::WrapMode::WordChar)
                    .monospace(true)
                    .editable(false)
                    .build();
                text_view.buffer().set_text(&export_text);

                let scroller = ScrolledWindow::builder()
                    .height_request(200)
                    .css_classes(["card"])
                    .child(&text_view)
                    .build();
                d_content.append(&scroller);

                let action_bar = Box::builder()
                    .orientation(Orientation::Horizontal)
                    .halign(Align::End)
                    .spacing(10)
                    .margin_top(8)
                    .build();

                let copy_b = Button::builder().label("Panoya Kopyala").icon_name("edit-copy-symbolic").css_classes(["suggested-action", "pill"]).build();
                let close_b = Button::builder().label("Kapat").css_classes(["flat"]).build();

                action_bar.append(&close_b);
                action_bar.append(&copy_b);
                d_content.append(&action_bar);
                d_root.append(&d_content);
                dialog.set_content(Some(&d_root));

                let d_close = dialog.clone();
                close_b.connect_clicked(move |_| {
                    d_close.close();
                });

                let text_to_copy = export_text.clone();
                let copy_btn_clone = copy_b.clone();
                copy_b.connect_clicked(move |_| {
                    if let Some(display) = Display::default() {
                        display.clipboard().set_text(&text_to_copy);
                        copy_btn_clone.set_label("Kopyalandı!");
                    }
                });

                dialog.present();
            }
        });

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
                        .label(format!("{} • Paket: {}", item.source.badge_label(), item.package_name))
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

        let refresh_on_change = refresh.clone();
        cart_state.on_change(move |_| {
            refresh_on_change();
        });

        let refresh_on_settings = refresh.clone();
        settings_state.on_change(move |_| {
            refresh_on_settings();
        });

        refresh();

        let scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&root)
            .build();

        (scroll, refresh)
    }
}
