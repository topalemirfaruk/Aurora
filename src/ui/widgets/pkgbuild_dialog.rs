use crate::package_managers::AurManager;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow, Spinner, TextView, Align};
use libadwaita as adw;
use libadwaita::prelude::*;

pub struct PkgbuildDialog;

impl PkgbuildDialog {
    pub fn show(parent: &impl IsA<gtk4::Window>, package_name: &str) {
        let dialog = adw::Window::builder()
            .transient_for(parent)
            .modal(true)
            .title(&format!("PKGBUILD İncele — {}", package_name))
            .default_width(680)
            .default_height(540)
            .build();

        let root = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();

        let header = adw::HeaderBar::builder()
            .show_end_title_buttons(true)
            .build();
        root.append(&header);

        let content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_start(20)
            .margin_end(20)
            .margin_top(12)
            .margin_bottom(20)
            .build();

        // Bilgi Başlığı
        let info_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        let info_label = Label::builder()
            .label(&format!("📦 AUR Kaynak ve Derleme Scripti: {}", package_name))
            .halign(Align::Start)
            .css_classes(["heading"])
            .hexpand(true)
            .build();
        info_box.append(&info_label);

        let copy_btn = Button::builder()
            .label("📋 Kopyala")
            .css_classes(["flat"])
            .build();
        info_box.append(&copy_btn);
        content.append(&info_box);

        // Spinner / Yükleniyor
        let spinner_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .valign(Align::Center)
            .halign(Align::Center)
            .margin_top(60)
            .margin_bottom(60)
            .build();

        let spinner = Spinner::builder()
            .spinning(true)
            .width_request(32)
            .height_request(32)
            .build();

        let loading_label = Label::builder()
            .label("PKGBUILD içeriği indiriliyor...")
            .css_classes(["dim-label"])
            .build();

        spinner_box.append(&spinner);
        spinner_box.append(&loading_label);
        content.append(&spinner_box);

        // Monospace Metin Alanı
        let text_view = TextView::builder()
            .editable(false)
            .cursor_visible(false)
            .monospace(true)
            .wrap_mode(gtk4::WrapMode::None)
            .build();

        let scroller = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .css_classes(["card"])
            .hexpand(true)
            .vexpand(true)
            .child(&text_view)
            .visible(false)
            .build();
        content.append(&scroller);

        root.append(&content);
        dialog.set_content(Some(&root));

        // Kopyalama Eylemi
        let buffer = text_view.buffer();
        let copy_btn_clone = copy_btn.clone();
        let buffer_clone = buffer.clone();
        copy_btn.connect_clicked(move |_| {
            let start = buffer_clone.start_iter();
            let end = buffer_clone.end_iter();
            let text = buffer_clone.text(&start, &end, false);
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

        // Arka planda PKGBUILD çekme
        let pkg_name = package_name.to_string();
        glib::spawn_future_local(async move {
            let result = AurManager::get_pkgbuild(&pkg_name).await;
            spinner_box.set_visible(false);
            scroller.set_visible(true);

            match result {
                Ok(content) => {
                    buffer.set_text(&content);
                }
                Err(err) => {
                    buffer.set_text(&format!("PKGBUILD alınırken hata oluştu:\n{}", err));
                }
            }
        });

        dialog.present();
    }
}
