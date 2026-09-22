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
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct InstallDialog;

impl InstallDialog {
    /// AUR helper kurulum komut ve argümanlarını üretir.
    /// `skip_review: true` olduğunda etkileşimsiz ortamda kilitlenmeyi önlemek için
    /// helper'a ait PKGBUILD inceleme atlama bayrakları eklenir.
    pub fn build_helper_args(helper: &str, pkgs: &[String], skip_review: bool) -> (String, Vec<String>) {
        let mut args = vec!["-S".to_string(), "--needed".to_string(), "--noconfirm".to_string()];
        if helper == "paru" {
            if skip_review {
                args.push("--skipreview".to_string());
            }
            args.push("--sudoflags".to_string());
            args.push("-A".to_string());
        } else if helper == "yay" {
            if skip_review {
                args.push("--answeredit".to_string());
                args.push("None".to_string());
                args.push("--answerclean".to_string());
                args.push("None".to_string());
            }
            args.push("--sudoflags".to_string());
            args.push("-A".to_string());
        }
        args.push("--".to_string());
        args.extend(pkgs.iter().cloned());
        (helper.to_string(), args)
    }

    /// Sistem güncellemesi için komut ve argümanları üretir.
    pub fn build_system_upgrade_args(helper: &str, skip_review: bool) -> (String, Vec<String>) {
        if helper == "paru" {
            let mut args = vec!["-Syu".to_string()];
            if skip_review {
                args.push("--skipreview".to_string());
            }
            args.push("--sudoflags".to_string());
            args.push("-A".to_string());
            args.push("--noconfirm".to_string());
            ("paru".to_string(), args)
        } else if helper == "yay" {
            let mut args = vec!["-Syu".to_string()];
            if skip_review {
                args.push("--answeredit".to_string());
                args.push("None".to_string());
                args.push("--answerclean".to_string());
                args.push("None".to_string());
            }
            args.push("--sudoflags".to_string());
            args.push("-A".to_string());
            args.push("--noconfirm".to_string());
            ("yay".to_string(), args)
        } else {
            ("sudo".to_string(), vec!["-A".to_string(), "pacman".to_string(), "-Syu".to_string(), "--noconfirm".to_string()])
        }
    }

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

        let require_review = has_aur && settings_data.require_pkgbuild_review;
        let pending_reviews = Rc::new(RefCell::new(
            aur_items_preview.iter().map(|i| i.package_name.clone()).collect::<HashSet<String>>()
        ));

        let status_label = Label::builder()
            .label(if require_review {
                "Güvenlik ayarınız gereği kuruluma geçmeden önce aşağıdaki AUR paketlerinin PKGBUILD betiklerini incelemeniz zorunludur."
            } else {
                "Tüm paket adları doğrulandı. Başlatmak için aşağıdaki butona tıklayın."
            })
            .halign(Align::Start)
            .css_classes(["dim-label"])
            .wrap(true)
            .build();

        let start_btn = Button::builder()
            .label("Kurulumu Başlat")
            .css_classes(["suggested-action", "pill"])
            .sensitive(!require_review)
            .build();

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

            let btn_box = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(8)
                .margin_top(4)
                .build();

            let notice_label = Label::builder()
                .label(if require_review {
                    "Ayarınız gereği PKGBUILD incelemesi zorunludur:"
                } else {
                    "İsteğe bağlı PKGBUILD incelemesi:"
                })
                .css_classes(["caption", "dim-label"])
                .halign(Align::Start)
                .build();
            btn_box.append(&notice_label);

            for item in &aur_items_preview {
                let review_btn = Button::builder()
                    .label(format!("{} İncele", item.package_name))
                    .icon_name("text-x-script-symbolic")
                    .css_classes(["flat", "pill"])
                    .build();

                let pkg_name = item.package_name.clone();
                let dialog_weak = dialog.downgrade();
                let pending_reviews_clone = pending_reviews.clone();
                let start_btn_clone = start_btn.clone();
                let status_label_clone = status_label.clone();
                let review_btn_clone = review_btn.clone();
                let require_review_val = require_review;

                review_btn.connect_clicked(move |_| {
                    if let Some(win) = dialog_weak.upgrade() {
                        PkgbuildDialog::show(&win, &pkg_name);
                    }

                    if require_review_val {
                        let mut pending = pending_reviews_clone.borrow_mut();
                        pending.remove(&pkg_name);
                        review_btn_clone.set_label(&format!("{} (İncelendi)", pkg_name));
                        review_btn_clone.set_icon_name("emblem-ok-symbolic");
                        review_btn_clone.add_css_class("success");

                        if pending.is_empty() {
                            start_btn_clone.set_sensitive(true);
                            status_label_clone.set_label("Tüm AUR PKGBUILD betikleri incelendi. Kuruluma başlayabilirsiniz.");
                        }
                    }
                });

                btn_box.append(&review_btn);
            }

            aur_warning_box.append(&btn_box);
            content.append(&aur_warning_box);
        }

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

        action_box.append(&close_btn);
        action_box.append(&start_btn);
        content.append(&action_box);

        root.append(&content);
        dialog.set_content(Some(&root));

        let active_cancel_tx = Rc::new(RefCell::new(Option::<tokio::sync::watch::Sender<bool>>::None));
        let is_running = Rc::new(RefCell::new(false));

        let active_cancel_for_close = active_cancel_tx.clone();
        let is_running_for_close = is_running.clone();
        dialog.connect_close_request(move |_| {
            if *is_running_for_close.borrow() {
                if let Some(tx) = active_cancel_for_close.borrow().as_ref() {
                    let _ = tx.send(true);
                }
            }
            glib::Propagation::Proceed
        });

        let dialog_for_close = dialog.clone();
        let active_cancel_for_btn = active_cancel_tx.clone();
        let is_running_for_btn = is_running.clone();
        close_btn.connect_clicked(move |btn| {
            if *is_running_for_btn.borrow() {
                if let Some(tx) = active_cancel_for_btn.borrow().as_ref() {
                    let _ = tx.send(true);
                }
                btn.set_sensitive(false);
                btn.set_label("İptal Ediliyor...");
            } else {
                dialog_for_close.close();
            }
        });

        let text_buffer = text_view.buffer();
        let tv_clone = text_view.clone();
        let on_finished = Rc::new(on_finished);
        let items_for_action = items.clone();
        let settings_for_action = settings_state.clone();

        start_btn.connect_clicked(move |btn| {
            *is_running.borrow_mut() = true;
            btn.set_sensitive(false);
            close_btn.set_label("İptal Et");
            close_btn.add_css_class("destructive-action");
            progress_bar.set_visible(true);
            status_label.set_label("İşlem başlatılıyor... (Gerekirse açılan pencereden yönetici şifrenizi onaylayın)");

            let buffer = text_buffer.clone();
            let tv = tv_clone.clone();
            let status = status_label.clone();
            let pbar = progress_bar.clone();
            let on_done = on_finished.clone();
            let active_cancel_in_task = active_cancel_tx.clone();
            let is_running_in_task = is_running.clone();
            let close_btn_in_task = close_btn.clone();

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

            let mut all_commands: Vec<(String, Vec<String>)> = Vec::new();

            if !official_items.is_empty() {
                let safe_pkgs: Vec<String> = official_items
                    .iter()
                    .map(|i| i.package_name.clone())
                    .filter(|p| is_valid_package_name(p))
                    .collect();

                if !safe_pkgs.is_empty() {
                    if sys.has_paru || sys.has_yay {
                        all_commands.push(Self::build_helper_args(&aur_helper, &safe_pkgs, true));
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
                    all_commands.push(Self::build_helper_args(&aur_helper, &safe_pkgs, true));
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
                    if !*is_running_in_task.borrow() {
                        overall_success = false;
                        break;
                    }

                    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
                    let (cancel_tx, cancel_rx) = tokio::sync::watch::channel(false);
                    *active_cancel_in_task.borrow_mut() = Some(cancel_tx);

                    let cmd_clone = cmd.clone();
                    let args_clone = args.clone();

                    let handle = tokio::spawn(async move {
                        let _ = CommandExecutor::run_streaming_cancellable(&cmd_clone, &args_clone, cancel_rx, move |msg| {
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

                    if !*is_running_in_task.borrow() {
                        overall_success = false;
                        break;
                    }
                }

                *is_running_in_task.borrow_mut() = false;
                *active_cancel_in_task.borrow_mut() = None;
                close_btn_in_task.set_sensitive(true);
                close_btn_in_task.set_label("Kapat");
                close_btn_in_task.remove_css_class("destructive-action");

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

        if !settings_data.require_summary_confirmation && !require_review {
            start_btn.emit_clicked();
        }
    }

    pub fn show_system_upgrade(
        parent: &impl IsA<gtk4::Window>,
        settings_state: SettingsState,
        on_finished: impl Fn() + 'static,
    ) {
        let sys = SystemCapabilities::detect();
        let configured = settings_state.get().aur_helper;
        let helper = if !configured.is_empty() {
            configured
        } else {
            sys.preferred_aur_helper().unwrap_or("paru").to_string()
        };

        let (cmd, args) = Self::build_system_upgrade_args(&helper, true);

        Self::show_command_stream(
            parent,
            "Sistem Güncellemesi",
            "Bu işlem resmi Arch Linux depolarındaki ve AUR üzerindeki tüm güncellemeleri kontrol edip sisteminizi en güncel sürüme yükseltecektir.",
            cmd,
            args,
            on_finished,
        );
    }

    pub fn show_command_stream(
        parent: &impl IsA<gtk4::Window>,
        title: &str,
        operation_desc: &str,
        cmd: String,
        args: Vec<String>,
        on_finished: impl Fn() + 'static,
    ) {
        let dialog = adw::Window::builder()
            .transient_for(parent)
            .modal(true)
            .title(title)
            .default_width(700)
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
            .spacing(16)
            .margin_start(24)
            .margin_end(24)
            .margin_top(16)
            .margin_bottom(24)
            .build();

        let title_label = Label::builder()
            .label(title)
            .halign(Align::Start)
            .css_classes(["title-2"])
            .build();
        content.append(&title_label);

        let desc_label = Label::builder()
            .label(operation_desc)
            .halign(Align::Start)
            .wrap(true)
            .css_classes(["dim-label"])
            .build();
        content.append(&desc_label);

        let status_label = Label::builder()
            .label("Başlatmak için 'İşlemi Başlat' butonuna tıklayın.")
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
            .height_request(240)
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
            .label("İşlemi Başlat")
            .css_classes(["suggested-action", "pill"])
            .build();

        action_box.append(&close_btn);
        action_box.append(&start_btn);
        content.append(&action_box);

        root.append(&content);
        dialog.set_content(Some(&root));

        let active_cancel_tx = Rc::new(RefCell::new(Option::<tokio::sync::watch::Sender<bool>>::None));
        let is_running = Rc::new(RefCell::new(false));

        let active_cancel_for_close = active_cancel_tx.clone();
        let is_running_for_close = is_running.clone();
        dialog.connect_close_request(move |_| {
            if *is_running_for_close.borrow() {
                if let Some(tx) = active_cancel_for_close.borrow().as_ref() {
                    let _ = tx.send(true);
                }
            }
            glib::Propagation::Proceed
        });

        let dialog_for_close = dialog.clone();
        let active_cancel_for_btn = active_cancel_tx.clone();
        let is_running_for_btn = is_running.clone();
        close_btn.connect_clicked(move |btn| {
            if *is_running_for_btn.borrow() {
                if let Some(tx) = active_cancel_for_btn.borrow().as_ref() {
                    let _ = tx.send(true);
                }
                btn.set_sensitive(false);
                btn.set_label("İptal Ediliyor...");
            } else {
                dialog_for_close.close();
            }
        });

        let text_buffer = text_view.buffer();
        let tv_clone = text_view.clone();
        let on_finished = Rc::new(on_finished);

        start_btn.connect_clicked(move |btn| {
            *is_running.borrow_mut() = true;
            btn.set_sensitive(false);
            close_btn.set_label("İptal Et");
            close_btn.add_css_class("destructive-action");
            progress_bar.set_visible(true);
            status_label.set_label("İşlem yürütülüyor... Gerekirse parolanızı onaylayın.");

            let buffer = text_buffer.clone();
            let tv = tv_clone.clone();
            let status = status_label.clone();
            let pbar = progress_bar.clone();
            let on_done = on_finished.clone();
            let active_cancel_in_task = active_cancel_tx.clone();
            let is_running_in_task = is_running.clone();
            let close_btn_in_task = close_btn.clone();

            let cmd_clone = cmd.clone();
            let args_clone = args.clone();

            glib::spawn_future_local(async move {
                let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
                let (cancel_tx, cancel_rx) = tokio::sync::watch::channel(false);
                *active_cancel_in_task.borrow_mut() = Some(cancel_tx);

                let handle = tokio::spawn(async move {
                    let _ = CommandExecutor::run_streaming_cancellable(&cmd_clone, &args_clone, cancel_rx, move |msg| {
                        let _ = sender.send(msg);
                    }).await;
                });

                let mut overall_success = false;

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
                            overall_success = success;
                            let mut end_iter = buffer.end_iter();
                            buffer.insert(&mut end_iter, &format!("\n--- İşlem bitti (Başarı: {}, Kod: {:?}) ---\n", success, code));
                            let mark = buffer.create_mark(None, &buffer.end_iter(), false);
                            tv.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);
                            buffer.delete_mark(&mark);
                        }
                    }
                }

                let _ = handle.await;

                *is_running_in_task.borrow_mut() = false;
                *active_cancel_in_task.borrow_mut() = None;
                close_btn_in_task.set_sensitive(true);
                close_btn_in_task.set_label("Kapat");
                close_btn_in_task.remove_css_class("destructive-action");

                pbar.set_visible(false);

                if overall_success {
                    status.set_label("İşlem başarıyla tamamlandı.");
                    on_done();
                } else {
                    status.set_label("İşlem sırasında bir hata oluştu veya işlem iptal edildi.");
                }
            });
        });

        dialog.present();
    }
}
