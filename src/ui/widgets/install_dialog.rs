use crate::models::{AppItem, PackageSource};
use crate::process::{CommandExecutor, ProcessMessage};
use crate::security::{is_running_as_root, is_valid_package_name};
use crate::state::SettingsState;
use crate::ui::widgets::PkgbuildDialog;
use crate::utils::SystemCapabilities;
use gtk4::prelude::*;
use gtk4::{
    Box, Button, Label, Orientation, ProgressBar, ScrolledWindow, TextView, Align,
};
use libadwaita as adw;
use libadwaita::prelude::*;
use std::rc::Rc;

pub struct InstallDialog;

impl InstallDialog {
    pub fn show(
        parent: &impl IsA<gtk4::Window>,
        items: Vec<AppItem>,
        settings_state: SettingsState,
        on_finished: impl Fn() + 'static,
    ) {
        let items = Rc::new(items);

        let dialog = adw::Window::builder()
            .transient_for(parent)
            .modal(true)
            .title("Kurulum İşlemi")
            .default_width(680)
            .default_height(560)
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
            .spacing(16)
            .margin_start(24)
            .margin_end(24)
            .margin_top(16)
            .margin_bottom(24)
            .build();

        let title_label = Label::builder()
            .label("Kurulum Onayı ve Güvenlik Denetimi")
            .halign(Align::Start)
            .css_classes(["title-2"])
            .build();
        content.append(&title_label);

        let aur_items_preview: Vec<_> = items.iter().filter(|i| i.source == PackageSource::Aur).collect();
        let has_aur = !aur_items_preview.is_empty();
        let running_root = is_running_as_root();
        let settings_data = settings_state.get();

        if running_root {
            let root_box = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(4)
                .css_classes(["aurora-card"])
                .build();

            let root_title = Label::builder()
                .label("Dikkat: Root Olarak Çalıştırılıyor")
                .halign(Align::Start)
                .css_classes(["heading"])
                .build();

            let root_text = Label::builder()
                .label("Aurora root yetkisiyle çalıştırılıyor. AUR yardımcıları (paru/yay) güvenlik nedeniyle root olarak derleme yapmayı reddeder veya sistem güvenliğini tehlikeye atar. Lütfen normal kullanıcı olarak çalıştırın.")
                .wrap(true)
                .halign(Align::Start)
                .css_classes(["caption"])
                .build();

            root_box.append(&root_title);
            root_box.append(&root_text);
            content.append(&root_box);
        }

        if has_aur {
            let aur_warning_box = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(8)
                .css_classes(["aurora-card"])
                .build();

            let warn_title = Label::builder()
                .label("AUR Güvenlik Bildirimi")
                .halign(Align::Start)
                .css_classes(["heading"])
                .build();

            let warn_text = Label::builder()
                .label("Seçtiğiniz paketler arasında Arch User Repository (AUR) kaynaklı yazılımlar bulunmaktadır. Topluluk paketleri sisteminizde derleme betikleri (PKGBUILD) koşturur.")
                .wrap(true)
                .halign(Align::Start)
                .css_classes(["caption"])
                .build();

            aur_warning_box.append(&warn_title);
            aur_warning_box.append(&warn_text);

            if settings_data.require_pkgbuild_review {
                let btn_box = Box::builder()
                    .orientation(Orientation::Horizontal)
                    .spacing(8)
                    .margin_top(4)
                    .build();

                let notice_label = Label::builder()
                    .label("Ayarınız gereği PKGBUILD incelemesi:")
                    .css_classes(["caption", "dim-label"])
                    .halign(Align::Start)
                    .build();
                btn_box.append(&notice_label);

                for item in &aur_items_preview {
                    let review_btn = Button::builder()
                        .label(&format!("{} İncele", item.package_name))
                        .icon_name("text-x-script-symbolic")
                        .css_classes(["flat", "pill"])
                        .build();

                    let pkg_name = item.package_name.clone();
                    let dialog_weak = dialog.downgrade();
                    review_btn.connect_clicked(move |_| {
                        if let Some(win) = dialog_weak.upgrade() {
                            PkgbuildDialog::show(&win, &pkg_name);
                        }
                    });

                    btn_box.append(&review_btn);
                }

                aur_warning_box.append(&btn_box);
            }

            content.append(&aur_warning_box);
        }

        let status_label = Label::builder()
            .label("Tüm paket adları doğrulandı. Başlatmak için aşağıdaki butona tıklayın.")
            .halign(Align::Start)
            .css_classes(["dim-label"])
            .build();
        content.append(&status_label);

        let progress_bar = ProgressBar::builder()
            .visible(false)
            .pulse_step(0.1)
            .build();
        content.append(&progress_bar);

        let text_view = TextView::builder()
            .editable(false)
            .cursor_visible(false)
            .monospace(true)
            .wrap_mode(gtk4::WrapMode::WordChar)
            .build();

        let console_scroller = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .height_request(200)
            .css_classes(["card"])
            .child(&text_view)
            .build();
        content.append(&console_scroller);

        let action_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::End)
            .spacing(12)
            .margin_top(8)
            .build();

        let close_btn = Button::builder()
            .label("İptal")
            .css_classes(["flat"])
            .build();

        let start_btn = Button::builder()
            .label("Kurulumu Başlat")
            .css_classes(["suggested-action", "pill"])
            .build();

        action_box.append(&close_btn);
        action_box.append(&start_btn);
        content.append(&action_box);

        root.append(&content);
        dialog.set_content(Some(&root));

        let dialog_for_close = dialog.clone();
        close_btn.connect_clicked(move |_| {
            dialog_for_close.close();
        });

        let text_buffer = text_view.buffer();
        let tv_clone = text_view.clone();
        let on_finished = Rc::new(on_finished);
        let items_for_action = items.clone();
        let settings_for_action = settings_state.clone();

        start_btn.connect_clicked(move |btn| {
            btn.set_sensitive(false);
            close_btn.set_label("Kapat");
            progress_bar.set_visible(true);
            status_label.set_label("İşlem başlatılıyor... (Gerekirse açılan pencereden yönetici şifrenizi onaylayın)");

            let buffer = text_buffer.clone();
            let tv = tv_clone.clone();
            let status = status_label.clone();
            let pbar = progress_bar.clone();
            let on_done = on_finished.clone();

            let official_items: Vec<_> = items_for_action.iter().filter(|i| i.source == PackageSource::Official).cloned().collect();
            let aur_items: Vec<_> = items_for_action.iter().filter(|i| i.source == PackageSource::Aur).cloned().collect();
            let flatpak_items: Vec<_> = items_for_action.iter().filter(|i| i.source == PackageSource::Flatpak).cloned().collect();

            let sys = SystemCapabilities::detect();
            let current_settings = settings_for_action.get();
            let aur_helper = if !current_settings.aur_helper.is_empty() {
                current_settings.aur_helper
            } else {
                sys.preferred_aur_helper().unwrap_or("paru").to_string()
            };

            let build_helper_args = |helper: &str, pkgs: Vec<String>| -> (String, Vec<String>) {
                let mut args = vec!["-S".to_string(), "--needed".to_string(), "--noconfirm".to_string()];
                if helper == "paru" {
                    args.push("--skipreview".to_string());
                    args.push("--sudoflags".to_string());
                    args.push("-A".to_string());
                } else if helper == "yay" {
                    args.push("--answeredit".to_string());
                    args.push("None".to_string());
                    args.push("--answerclean".to_string());
                    args.push("None".to_string());
                    args.push("--sudoflags".to_string());
                    args.push("-A".to_string());
                }
                args.push("--".to_string());
                args.extend(pkgs);
                (helper.to_string(), args)
            };

            let mut all_commands: Vec<(String, Vec<String>)> = Vec::new();

            if !official_items.is_empty() {
                let safe_pkgs: Vec<String> = official_items
                    .iter()
                    .map(|i| i.package_name.clone())
                    .filter(|p| is_valid_package_name(p))
                    .collect();

                if !safe_pkgs.is_empty() {
                    if sys.has_paru || sys.has_yay {
                        all_commands.push(build_helper_args(&aur_helper, safe_pkgs));
                    } else {
                        let mut args = vec!["-A".to_string(), "pacman".to_string(), "-S".to_string(), "--needed".to_string(), "--noconfirm".to_string(), "--".to_string()];
                        args.extend(safe_pkgs);
                        all_commands.push(("sudo".to_string(), args));
                    }
                }
            }

            if !aur_items.is_empty() {
                let safe_pkgs: Vec<String> = aur_items
                    .iter()
                    .map(|i| i.package_name.clone())
                    .filter(|p| is_valid_package_name(p))
                    .collect();

                if !safe_pkgs.is_empty() {
                    all_commands.push(build_helper_args(&aur_helper, safe_pkgs));
                }
            }

            if !flatpak_items.is_empty() {
                let safe_pkgs: Vec<String> = flatpak_items
                    .iter()
                    .map(|i| i.package_name.clone())
                    .filter(|p| is_valid_package_name(p))
                    .collect();

                if !safe_pkgs.is_empty() {
                    let mut args = vec!["install".to_string(), "-y".to_string(), "flathub".to_string(), "--".to_string()];
                    args.extend(safe_pkgs);
                    all_commands.push(("flatpak".to_string(), args));
                }
            }

            glib::spawn_future_local(async move {
                let mut overall_success = true;

                for (cmd, args) in all_commands {
                    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

                    let cmd_clone = cmd.clone();
                    let args_clone = args.clone();

                    let handle = tokio::spawn(async move {
                        let _ = CommandExecutor::run_streaming(&cmd_clone, &args_clone, move |msg| {
                            let _ = sender.send(msg);
                        }).await;
                    });

                    while let Some(msg) = receiver.recv().await {
                        match msg {
                            ProcessMessage::Stdout(line) => {
                                let mut end_iter = buffer.end_iter();
                                buffer.insert(&mut end_iter, &format!("{}\n", line));
                                pbar.pulse();
                                let mark = buffer.create_mark(None, &buffer.end_iter(), false);
                                tv.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);
                                buffer.delete_mark(&mark);
                            }
                            ProcessMessage::Stderr(line) => {
                                let mut end_iter = buffer.end_iter();
                                buffer.insert(&mut end_iter, &format!("[ERR] {}\n", line));
                                pbar.pulse();
                                let mark = buffer.create_mark(None, &buffer.end_iter(), false);
                                tv.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);
                                buffer.delete_mark(&mark);
                            }
                            ProcessMessage::Finished(success, code) => {
                                let mut end_iter = buffer.end_iter();
                                buffer.insert(&mut end_iter, &format!("\n--- İşlem bitti (Başarı: {}, Çıkış Kodu: {:?}) ---\n\n", success, code));
                                let mark = buffer.create_mark(None, &buffer.end_iter(), false);
                                tv.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);
                                buffer.delete_mark(&mark);
                                if !success {
                                    overall_success = false;
                                }
                            }
                        }
                    }

                    let _ = handle.await;
                }

                pbar.set_visible(false);
                if overall_success {
                    status.set_label("Kurulum işlemi başarıyla tamamlandı.");
                    on_done();
                } else {
                    status.set_label("Bazı işlemler tamamlanamadı veya iptal edildi. Günlükleri inceleyin.");
                }
            });
        });

        dialog.present();

        if !settings_data.require_summary_confirmation {
            start_btn.emit_clicked();
        }
    }
}
