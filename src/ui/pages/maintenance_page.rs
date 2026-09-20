use crate::services::maintenance_service::MaintenanceService;
use crate::ui::widgets::InstallDialog;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow, Spinner, Align};
use libadwaita as adw;
use libadwaita::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MaintenancePage;

impl MaintenancePage {
    pub fn build() -> ScrolledWindow {
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
            .label("Sistem Bakımı & Temizlik")
            .halign(Align::Start)
            .css_classes(["title-1"])
            .hexpand(true)
            .build();
        header.append(&title);

        let refresh_btn = Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Sistem sağlık ve önbellek durumunu yeniden tara")
            .css_classes(["flat", "circular"])
            .build();
        header.append(&refresh_btn);
        root.append(&header);

        let spinner = Spinner::builder()
            .spinning(true)
            .width_request(32)
            .height_request(32)
            .halign(Align::Center)
            .margin_top(40)
            .margin_bottom(40)
            .build();
        root.append(&spinner);

        let content_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(16)
            .visible(false)
            .build();
        root.append(&content_box);

        let health_card = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(16)
            .css_classes(["aurora-card"])
            .build();

        let health_icon = gtk4::Image::from_icon_name("emblem-default-symbolic");
        health_icon.set_pixel_size(32);
        health_card.append(&health_icon);

        let health_text = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .build();

        let health_title = Label::builder()
            .label("Sistem Durumu: Taranıyor")
            .halign(Align::Start)
            .css_classes(["heading"])
            .build();

        let health_subtitle = Label::builder()
            .label("Systemd servisleri ve paket durumu kontrol ediliyor...")
            .halign(Align::Start)
            .css_classes(["caption", "dim-label"])
            .build();

        health_text.append(&health_title);
        health_text.append(&health_subtitle);
        health_card.append(&health_text);
        content_box.append(&health_card);

        let cache_group = adw::PreferencesGroup::builder()
            .title("Paket Önbelleği (Pacman Cache)")
            .description("İndirilen paket arşivleri /var/cache/pacman/pkg dizininde saklanır.")
            .build();

        let cache_size_row = adw::ActionRow::builder()
            .title("Önbellek Boyutu")
            .subtitle("Hesaplanıyor...")
            .build();
        cache_group.add(&cache_size_row);

        let clean_cache_row = adw::ActionRow::builder()
            .title("Eski Paket Sürümlerini Temizle")
            .subtitle("Son 3 sürüm haricindeki eski paket arşivlerini silerek disk alanı açar (paccache -r).")
            .build();

        let clean_cache_btn = Button::builder()
            .label("Önbelleği Temizle")
            .css_classes(["suggested-action", "pill"])
            .valign(Align::Center)
            .build();
        clean_cache_row.add_suffix(&clean_cache_btn);
        cache_group.add(&clean_cache_row);

        let clean_uninstalled_row = adw::ActionRow::builder()
            .title("Kaldırılmış Paketlerin Önbelleğini Sil")
            .subtitle("Artık sistemde kurulu olmayan paketlerin arşiv kalıntılarını tamamen siler (paccache -ruk0).")
            .build();

        let clean_uninstalled_btn = Button::builder()
            .label("Kalıntıları Temizle")
            .css_classes(["flat", "pill"])
            .valign(Align::Center)
            .build();
        clean_uninstalled_row.add_suffix(&clean_uninstalled_btn);
        cache_group.add(&clean_uninstalled_row);

        content_box.append(&cache_group);

        let orphans_group = adw::PreferencesGroup::builder()
            .title("Yetim Paketler (Orphaned Packages)")
            .description("Başka hiçbir paket tarafından gereksinim duyulmayan kullanılmayan bağımlılıklar (pacman -Qtdq).")
            .build();

        let orphans_row = adw::ActionRow::builder()
            .title("Yetim Paket Durumu")
            .subtitle("Taranıyor...")
            .build();

        let clean_orphans_btn = Button::builder()
            .label("Yetimleri Kaldır")
            .css_classes(["destructive-action", "pill"])
            .valign(Align::Center)
            .visible(false)
            .build();
        orphans_row.add_suffix(&clean_orphans_btn);
        orphans_group.add(&orphans_row);
        content_box.append(&orphans_group);

        let services_group = adw::PreferencesGroup::builder()
            .title("Sistem Servisleri (Systemd Health)")
            .description("Sistem başlangıcında veya çalışma esnasında hata veren arka plan servisleri.")
            .build();

        let services_row = adw::ActionRow::builder()
            .title("Başarısız Servisler")
            .subtitle("Taranıyor...")
            .build();
        services_group.add(&services_row);
        content_box.append(&services_group);

        let current_orphans = Rc::new(RefCell::new(Vec::<String>::new()));
        type RefreshHealthFn = Rc<dyn Fn()>;
        let load_health_ref: Rc<RefCell<Option<RefreshHealthFn>>> = Rc::new(RefCell::new(None));

        let load_health = {
            let spinner = spinner.clone();
            let content_box = content_box.clone();
            let health_title = health_title.clone();
            let health_subtitle = health_subtitle.clone();
            let cache_size_row = cache_size_row.clone();
            let orphans_row = orphans_row.clone();
            let clean_orphans_btn = clean_orphans_btn.clone();
            let services_row = services_row.clone();
            let current_orphans = current_orphans.clone();

            Rc::new(move || {
                spinner.set_visible(true);
                content_box.set_visible(false);

                let spinner = spinner.clone();
                let content_box = content_box.clone();
                let health_title = health_title.clone();
                let health_subtitle = health_subtitle.clone();
                let cache_size_row = cache_size_row.clone();
                let orphans_row = orphans_row.clone();
                let clean_orphans_btn = clean_orphans_btn.clone();
                let services_row = services_row.clone();
                let current_orphans = current_orphans.clone();

                glib::spawn_future_local(async move {
                    let info = MaintenanceService::check_health().await;

                    if info.is_running == "running" && info.failed_services.is_empty() {
                        health_title.set_label("Sistem Durumu: Sağlıklı");
                        health_subtitle.set_label("Tüm systemd servisleri ve çekirdek bileşenler sorunsuz çalışıyor.");
                    } else if !info.failed_services.is_empty() {
                        health_title.set_label(&format!("Sistem Durumu: {} Başarısız Servis", info.failed_services.len()));
                        health_subtitle.set_label("Bazı arka plan servisleri başlatılamadı veya durduruldu.");
                    } else {
                        health_title.set_label(&format!("Sistem Durumu: {}", info.is_running));
                        health_subtitle.set_label("Systemd çalışma durumu güncellendi.");
                    }

                    cache_size_row.set_subtitle(&format!("Pacman önbellek boyutu: {}", info.cache_size));

                    let orphan_count = info.orphaned_packages.len();
                    *current_orphans.borrow_mut() = info.orphaned_packages.clone();

                    if orphan_count == 0 {
                        orphans_row.set_subtitle("Sisteminizde gereksiz yetim paket bulunmuyor.");
                        clean_orphans_btn.set_visible(false);
                    } else {
                        let preview = if orphan_count <= 4 {
                            info.orphaned_packages.join(", ")
                        } else {
                            format!("{}, ... ve {} diğer", info.orphaned_packages[..3].join(", "), orphan_count - 3)
                        };
                        orphans_row.set_subtitle(&format!("{} yetim paket bulundu: {}", orphan_count, preview));
                        clean_orphans_btn.set_visible(true);
                    }

                    if info.failed_services.is_empty() {
                        services_row.set_subtitle("Hata veren veya çöken servis bulunamadı.");
                    } else {
                        services_row.set_subtitle(&format!("Başarısız olan servisler: {}", info.failed_services.join(", ")));
                    }

                    spinner.set_visible(false);
                    content_box.set_visible(true);
                });
            })
        };

        *load_health_ref.borrow_mut() = Some(load_health.clone());

        let load_for_refresh = load_health.clone();
        refresh_btn.connect_clicked(move |_| {
            load_for_refresh();
        });

        let load_for_cache = load_health_ref.clone();
        clean_cache_btn.connect_clicked(move |btn| {
            if let Some(root_win) = btn.root().and_downcast::<gtk4::Window>() {
                let (cmd, args) = MaintenanceService::build_cache_clean_command(false);
                let load_fn = load_for_cache.borrow().clone();
                InstallDialog::show_command_stream(
                    &root_win,
                    "Pacman Önbellek Temizleme",
                    "Eski paket arşivleri temizleniyor (Son 3 sürüm korunur)...",
                    cmd,
                    args,
                    move || {
                        if let Some(ref f) = load_fn {
                            f();
                        }
                    },
                );
            }
        });

        let load_for_uninstalled = load_health_ref.clone();
        clean_uninstalled_btn.connect_clicked(move |btn| {
            if let Some(root_win) = btn.root().and_downcast::<gtk4::Window>() {
                let (cmd, args) = MaintenanceService::build_cache_clean_command(true);
                let load_fn = load_for_uninstalled.borrow().clone();
                InstallDialog::show_command_stream(
                    &root_win,
                    "Kaldırılmış Paket Kalıntılarını Temizleme",
                    "Sistemde artık kurulu olmayan paketlerin önbellek kalıntıları temizleniyor...",
                    cmd,
                    args,
                    move || {
                        if let Some(ref f) = load_fn {
                            f();
                        }
                    },
                );
            }
        });

        let load_for_orphans = load_health_ref.clone();
        let orphans_data = current_orphans.clone();
        clean_orphans_btn.connect_clicked(move |btn| {
            if let Some(root_win) = btn.root().and_downcast::<gtk4::Window>() {
                let orphans = orphans_data.borrow().clone();
                if let Some((cmd, args)) = MaintenanceService::build_orphan_clean_command(&orphans) {
                    let load_fn = load_for_orphans.borrow().clone();
                    InstallDialog::show_command_stream(
                        &root_win,
                        "Yetim Paketleri Temizleme",
                        "Gereksiz bağımlılıklar sistemden güvenle kaldırılıyor (pacman -Rns)...",
                        cmd,
                        args,
                        move || {
                            if let Some(ref f) = load_fn {
                                f();
                            }
                        },
                    );
                }
            }
        });

        load_health();

        ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&root)
            .build()
    }
}
