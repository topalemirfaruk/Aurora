use crate::models::PackageSource;
use crate::process::{CommandExecutor, ProcessMessage};
use crate::security::{is_critical_system_package, is_valid_package_name};
use crate::utils::SystemCapabilities;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, ProgressBar, ScrolledWindow, TextView, Align};
use libadwaita as adw;
use libadwaita::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct UninstallDialog;

impl UninstallDialog {
    pub fn show(
        parent: &impl IsA<gtk4::Window>,
        package_name: &str,
        source: PackageSource,
        on_success: impl Fn() + 'static,
    ) {
        let is_flatpak = source == PackageSource::Flatpak;
        let is_critical = !is_flatpak && is_critical_system_package(package_name);
        let is_valid = is_valid_package_name(package_name);

        let dialog = adw::Window::builder()
            .transient_for(parent)
            .modal(true)
            .title(format!("Paket Kaldır — {}", package_name))
            .default_width(580)
            .default_height(460)
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
            .label(format!("'{}' Paketini Kaldır", package_name))
            .halign(Align::Start)
            .css_classes(["title-2"])
            .build();
        content.append(&title_label);

        if !is_valid {
            let err_box = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(4)
                .css_classes(["aurora-card"])
                .build();

            let err_title = Label::builder()
                .label("Geçersiz veya Tehlikeli Paket Adı")
                .halign(Align::Start)
                .css_classes(["heading"])
                .build();

            let err_desc = Label::builder()
                .label("Paket adı Linux güvenlik standartlarına uymuyor. Güvenlik nedeniyle işlem engellendi.")
                .halign(Align::Start)
                .css_classes(["caption"])
                .build();

            err_box.append(&err_title);
            err_box.append(&err_desc);
            content.append(&err_box);

            root.append(&content);
            dialog.set_content(Some(&root));
            dialog.present();
            return;
        }

        if is_critical {
            let crit_box = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(4)
                .css_classes(["aurora-card"])
                .build();

            let crit_title = Label::builder()
                .label("Kritik Sistem Bileşeni")
                .halign(Align::Start)
                .css_classes(["heading"])
                .build();

            let crit_desc = Label::builder()
                .label(format!(
                    "'{}' paketi Arch Linux'un hayati çekirdek bileşenidir. Bu paketin kaldırılması sistemin çökmesine ve bir daha açılmamasına yol açar. Aurora bu paketin kaldırılmasına izin vermez.",
                    package_name
                ))
                .wrap(true)
                .halign(Align::Start)
                .css_classes(["caption"])
                .build();

            crit_box.append(&crit_title);
            crit_box.append(&crit_desc);
            content.append(&crit_box);

            let close_btn = Button::builder()
                .label("Anladım, Kapat")
                .halign(Align::End)
                .css_classes(["suggested-action", "pill"])
                .build();

            let dialog_clone = dialog.clone();
            close_btn.connect_clicked(move |_| {
                dialog_clone.close();
            });
            content.append(&close_btn);

            root.append(&content);
            dialog.set_content(Some(&root));
            dialog.present();
            return;
        }

        let desc_text = if is_flatpak {
            format!(
                "Bu işlem '{}' Flatpak uygulamasını ve yerel verilerini sisteminizden kaldıracaktır.\nKomut: flatpak uninstall -y -- {}",
                package_name, package_name
            )
        } else {
            format!(
                "Bu işlem '{}' paketini ve varsa bağımlılıklarını sisteminizden kaldıracaktır.\nKomut: sudo pacman -R -- {}",
                package_name, package_name
            )
        };

        let desc_label = Label::builder()
            .label(&desc_text)
            .wrap(true)
            .halign(Align::Start)
            .css_classes(["dim-label"])
            .build();
        content.append(&desc_label);

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
            .height_request(180)
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

        let cancel_btn = Button::builder()
            .label("Vazgeç")
            .css_classes(["flat"])
            .build();

        let remove_btn = Button::builder()
            .label("Kaldırmayı Onayla")
            .css_classes(["destructive-action", "pill"])
            .build();

        action_box.append(&cancel_btn);
        action_box.append(&remove_btn);
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

        let dialog_for_cancel = dialog.clone();
        let active_cancel_for_btn = active_cancel_tx.clone();
        let is_running_for_btn = is_running.clone();
        cancel_btn.connect_clicked(move |btn| {
            if *is_running_for_btn.borrow() {
                if let Some(tx) = active_cancel_for_btn.borrow().as_ref() {
                    let _ = tx.send(true);
                }
                btn.set_sensitive(false);
                btn.set_label("İptal Ediliyor...");
            } else {
                dialog_for_cancel.close();
            }
        });

        let pkg_name_str = package_name.to_string();
        let buffer = text_view.buffer();
        let on_success = Rc::new(on_success);

        remove_btn.connect_clicked(move |btn| {
            *is_running.borrow_mut() = true;
            btn.set_sensitive(false);
            cancel_btn.set_label("İptal Et");
            cancel_btn.add_css_class("destructive-action");
            progress_bar.set_visible(true);

            let buffer = buffer.clone();
            let pbar = progress_bar.clone();
            let on_done = on_success.clone();
            let pkg = pkg_name_str.clone();
            let active_cancel_in_task = active_cancel_tx.clone();
            let is_running_in_task = is_running.clone();
            let cancel_btn_in_task = cancel_btn.clone();

            let sys = SystemCapabilities::detect();
            let (cmd, args) = if is_flatpak {
                ("flatpak".to_string(), vec!["uninstall".to_string(), "-y".to_string(), "--".to_string(), pkg])
            } else if sys.has_paru {
                ("paru".to_string(), vec!["-R".to_string(), "--noconfirm".to_string(), "--sudoflags".to_string(), "-A".to_string(), "--".to_string(), pkg])
            } else if sys.has_yay {
                ("yay".to_string(), vec!["-R".to_string(), "--noconfirm".to_string(), "--sudoflags".to_string(), "-A".to_string(), "--".to_string(), pkg])
            } else {
                ("pkexec".to_string(), vec!["pacman".to_string(), "-R".to_string(), "--noconfirm".to_string(), "--".to_string(), pkg])
            };

            glib::spawn_future_local(async move {
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

                let mut success_status = false;

                while let Some(msg) = receiver.recv().await {
                    match msg {
                        ProcessMessage::Stdout(line) => {
                            let mut end = buffer.end_iter();
                            buffer.insert(&mut end, &format!("{}\n", line));
                            pbar.pulse();
                        }
                        ProcessMessage::Stderr(line) => {
                            let mut end = buffer.end_iter();
                            buffer.insert(&mut end, &format!("[ERR] {}\n", line));
                            pbar.pulse();
                        }
                        ProcessMessage::Finished(success, code) => {
                            success_status = success;
                            let mut end = buffer.end_iter();
                            buffer.insert(&mut end, &format!("\n--- Kaldırma tamamlandı (Başarı: {}, Kod: {:?}) ---\n", success, code));
                        }
                    }
                }

                let _ = handle.await;

                *is_running_in_task.borrow_mut() = false;
                *active_cancel_in_task.borrow_mut() = None;
                cancel_btn_in_task.set_sensitive(true);
                cancel_btn_in_task.set_label("Kapat");
                cancel_btn_in_task.remove_css_class("destructive-action");

                pbar.set_visible(false);

                if success_status {
                    on_done();
                }
            });
        });

        dialog.present();
    }
}
