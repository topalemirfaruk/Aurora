use crate::config::{APP_CONTRIBUTING_URL, APP_GITHUB_URL, APP_ISSUES_URL, APP_NAME, APP_VERSION};
use crate::models::{AppCategory, AppItem};
use crate::security::is_running_as_root;
use crate::state::CartState;
use crate::ui::pages::{AppDetailPage, CartPage, CatalogPage, HomePage, InstalledPage, MaintenancePage, SettingsPage};
use crate::ui::widgets::{AboutAppDialog, InstallDialog, ShortcutsHelpDialog};
use gio::prelude::*;
use gtk4::prelude::*;
use gtk4::{
    Box, Button, Label, ListBox, ListBoxRow, Orientation, SearchEntry, Stack, Align
};
use libadwaita as adw;
use libadwaita::prelude::*;

pub struct MainWindow;

impl MainWindow {
    pub fn build(app: &adw::Application) -> adw::ApplicationWindow {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title(format!("{} v{}", APP_NAME, APP_VERSION))
            .default_width(1100)
            .default_height(720)
            .icon_name(crate::config::APP_ID)
            .build();

        let cart_state = CartState::new();
        let settings_state = crate::state::SettingsState::new();
        let installed_state = crate::state::InstalledState::new();

        crate::utils::ThemeManager::apply_theme(settings_state.get().dark_theme);

        let stack = Stack::builder()
            .transition_type(gtk4::StackTransitionType::Crossfade)
            .hexpand(true)
            .vexpand(true)
            .build();

        let search_entry = SearchEntry::builder()
            .placeholder_text("Uygulama, paket adı veya etiket ara... (Ctrl+F)")
            .width_request(300)
            .build();

        fn create_sidebar_row(icon_name: &str, label_text: &str, page_name: &str) -> ListBoxRow {
            let row = ListBoxRow::new();
            let row_box = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(12)
                .margin_start(12)
                .margin_end(12)
                .margin_top(8)
                .margin_bottom(8)
                .build();

            let icon = gtk4::Image::from_icon_name(icon_name);
            let label = Label::builder()
                .label(label_text)
                .halign(Align::Start)
                .hexpand(true)
                .build();

            row_box.append(&icon);
            row_box.append(&label);
            row.set_child(Some(&row_box));
            row.set_widget_name(page_name);
            row
        }

        let list_box = ListBox::builder()
            .css_classes(["navigation-sidebar"])
            .selection_mode(gtk4::SelectionMode::Single)
            .build();

        let row_home = create_sidebar_row("user-home-symbolic", "Keşfet", "home");
        let row_catalog = create_sidebar_row("view-app-grid-symbolic", "Katalog", "catalog");
        let row_cart = create_sidebar_row(crate::utils::IconResolver::cart_icon(), "Kurulum Sepeti", "cart");
        let row_installed = create_sidebar_row("system-software-uninstall-symbolic", "Kurulu & Kaldır", "installed");
        let row_maintenance = create_sidebar_row(
            crate::utils::IconResolver::first_available(
                &["preferences-system-cleanup-symbolic", "edit-clear-all-symbolic", "system-run-symbolic"],
                "system-run-symbolic",
            ),
            "Sistem Bakımı",
            "maintenance",
        );
        let row_settings = create_sidebar_row("emblem-system-symbolic", "Ayarlar", "settings");

        list_box.append(&row_home);
        list_box.append(&row_catalog);
        list_box.append(&row_cart);
        list_box.append(&row_installed);
        list_box.append(&row_maintenance);
        list_box.append(&row_settings);

        list_box.select_row(Some(&row_home));

        let stack_for_nav = stack.clone();
        list_box.connect_row_selected(move |_, selected_row| {
            if let Some(row) = selected_row {
                let page_name = row.widget_name();
                stack_for_nav.set_visible_child_name(&page_name);
            }
        });

        let window_weak = window.downgrade();
        let cart_for_detail = cart_state.clone();
        let installed_for_detail = installed_state.clone();
        let on_detail = move |item: AppItem| {
            if let Some(win) = window_weak.upgrade() {
                AppDetailPage::show(&win, item, cart_for_detail.clone(), installed_for_detail.clone());
            }
        };

        let (catalog_scroll, refresh_catalog) =
            CatalogPage::build(cart_state.clone(), installed_state.clone(), on_detail.clone());

        let stack_for_cat = stack.clone();
        let refresh_cat = refresh_catalog.clone();
        let list_for_cat = list_box.clone();
        let row_cat_for_cb = row_catalog.clone();
        let on_cat_clicked = move |cat: AppCategory| {
            list_for_cat.select_row(Some(&row_cat_for_cb));
            stack_for_cat.set_visible_child_name("catalog");
            refresh_cat(Some(cat), "");
        };

        let home_scroll = HomePage::build(
            cart_state.clone(),
            installed_state.clone(),
            on_detail.clone(),
            on_cat_clicked,
        );

        let window_weak_for_install = window.downgrade();
        let cart_for_install_done = cart_state.clone();
        let settings_for_install = settings_state.clone();
        let installed_for_install = installed_state.clone();
        let on_install = move |items: Vec<AppItem>| {
            if let Some(win) = window_weak_for_install.upgrade() {
                let cart_clear = cart_for_install_done.clone();
                let current_settings = settings_for_install.clone();
                let inst_refresh = installed_for_install.clone();
                InstallDialog::show(&win, items, current_settings, move || {
                    cart_clear.clear();
                    inst_refresh.refresh_background();
                });
            }
        };

        let (cart_scroll, _refresh_cart) = CartPage::build(cart_state.clone(), settings_state.clone(), on_install);
        let installed_scroll = InstalledPage::build(installed_state.clone(), settings_state.clone());
        let maintenance_scroll = MaintenancePage::build();
        let settings_page = SettingsPage::build(settings_state.clone());

        stack.add_named(&home_scroll, Some("home"));
        stack.add_named(&catalog_scroll, Some("catalog"));
        stack.add_named(&cart_scroll, Some("cart"));
        stack.add_named(&installed_scroll, Some("installed"));
        stack.add_named(&maintenance_scroll, Some("maintenance"));
        stack.add_named(&settings_page, Some("settings"));

        let stack_for_search = stack.clone();
        let refresh_search = refresh_catalog.clone();
        let list_for_search = list_box.clone();
        let row_cat_for_search = row_catalog.clone();
        search_entry.connect_search_changed(move |entry| {
            let text = entry.text().to_string();
            if !text.is_empty() {
                list_for_search.select_row(Some(&row_cat_for_search));
                stack_for_search.set_visible_child_name("catalog");
            }
            refresh_search(None, &text);
        });

        let header_bar = adw::HeaderBar::builder()
            .title_widget(&search_entry)
            .build();

        let cart_header_btn = Button::builder()
            .css_classes(["flat"])
            .tooltip_text("Kurulum Sepetini Aç (Ctrl+3)")
            .build();

        let cart_btn_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .build();

        let cart_btn_icon = gtk4::Image::from_icon_name(crate::utils::IconResolver::cart_icon());
        let cart_badge_label = Label::builder()
            .label("0")
            .css_classes(["cart-counter"])
            .build();

        cart_btn_box.append(&cart_btn_icon);
        cart_btn_box.append(&cart_badge_label);
        cart_header_btn.set_child(Some(&cart_btn_box));

        let stack_for_cart_btn = stack.clone();
        let list_for_cart_btn = list_box.clone();
        let row_cart_for_btn = row_cart.clone();
        cart_header_btn.connect_clicked(move |_| {
            list_for_cart_btn.select_row(Some(&row_cart_for_btn));
            stack_for_cart_btn.set_visible_child_name("cart");
        });

        let initial_dark = settings_state.get().dark_theme;
        let theme_toggle_btn = Button::builder()
            .icon_name(crate::utils::IconResolver::theme_icon_for(initial_dark))
            .tooltip_text(if initial_dark { "Açık Temaya Geç" } else { "Koyu Temaya Geç" })
            .css_classes(["flat", "circular"])
            .build();

        let settings_clone_for_btn = settings_state.clone();
        theme_toggle_btn.connect_clicked(move |_| {
            let next_dark = !settings_clone_for_btn.get().dark_theme;
            crate::utils::ThemeManager::apply_theme(next_dark);
            settings_clone_for_btn.set_dark_theme(next_dark);
        });

        let btn_for_state = theme_toggle_btn.clone();
        settings_state.on_change(move |data| {
            crate::utils::ThemeManager::apply_theme(data.dark_theme);
            btn_for_state.set_icon_name(crate::utils::IconResolver::theme_icon_for(data.dark_theme));
            btn_for_state.set_tooltip_text(Some(if data.dark_theme {
                "Açık Temaya Geç"
            } else {
                "Koyu Temaya Geç"
            }));
        });

        let menu = gio::Menu::new();
        menu.append(Some("Klavye Kısayolları (F1)"), Some("win.shortcuts"));
        menu.append(Some("Katkıda Bulunma Kılavuzu"), Some("win.contribute_guide"));
        menu.append(Some("GitHub Deposu"), Some("win.contribute"));
        menu.append(Some("Hata / Öneri Bildir"), Some("win.issues"));
        menu.append(Some("Aurora Hakkında"), Some("win.about"));

        let primary_menu_btn = gtk4::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Ana Menü")
            .menu_model(&menu)
            .css_classes(["flat", "circular"])
            .build();

        header_bar.pack_end(&primary_menu_btn);
        header_bar.pack_end(&theme_toggle_btn);
        header_bar.pack_end(&cart_header_btn);

        let root_banner = adw::Banner::builder()
            .title("Aurora root olarak çalıştırılıyor! Güvenlik ve AUR uyumluluğu için normal kullanıcı olarak başlatın.")
            .revealed(is_running_as_root())
            .build();

        let sidebar_box = Box::builder()
            .orientation(Orientation::Vertical)
            .width_request(220)
            .css_classes(["sidebar"])
            .build();

        let brand_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .margin_start(16)
            .margin_end(16)
            .margin_top(16)
            .margin_bottom(16)
            .build();

        let logo_icon = gtk4::Image::from_icon_name(crate::config::APP_ID);
        logo_icon.set_pixel_size(28);

        let brand_label = Label::builder()
            .label(APP_NAME)
            .css_classes(["title-3"])
            .halign(Align::Start)
            .build();

        brand_box.append(&logo_icon);
        brand_box.append(&brand_label);
        sidebar_box.append(&brand_box);
        sidebar_box.append(&list_box);

        let badge_clone = cart_badge_label.clone();
        cart_state.on_change(move |count| {
            badge_clone.set_label(&count.to_string());
        });

        let body_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .hexpand(true)
            .vexpand(true)
            .build();

        let separator = gtk4::Separator::new(Orientation::Vertical);

        body_box.append(&sidebar_box);
        body_box.append(&separator);
        body_box.append(&stack);

        let main_layout = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();

        main_layout.append(&header_bar);
        main_layout.append(&root_banner);
        main_layout.append(&body_box);

        // Pencere Eylemleri (Window Actions)
        let act_search = gio::SimpleAction::new("search", None);
        let entry_for_act = search_entry.clone();
        act_search.connect_activate(move |_, _| {
            entry_for_act.grab_focus();
            entry_for_act.select_region(0, -1);
        });
        window.add_action(&act_search);

        let act_home = gio::SimpleAction::new("page-home", None);
        let list_home = list_box.clone();
        let row_home_act = row_home.clone();
        let stack_home = stack.clone();
        act_home.connect_activate(move |_, _| {
            list_home.select_row(Some(&row_home_act));
            stack_home.set_visible_child_name("home");
        });
        window.add_action(&act_home);

        let act_cat = gio::SimpleAction::new("page-catalog", None);
        let list_cat = list_box.clone();
        let row_cat_act = row_catalog.clone();
        let stack_cat = stack.clone();
        act_cat.connect_activate(move |_, _| {
            list_cat.select_row(Some(&row_cat_act));
            stack_cat.set_visible_child_name("catalog");
        });
        window.add_action(&act_cat);

        let act_cart = gio::SimpleAction::new("page-cart", None);
        let list_cart = list_box.clone();
        let row_cart_act = row_cart.clone();
        let stack_cart = stack.clone();
        act_cart.connect_activate(move |_, _| {
            list_cart.select_row(Some(&row_cart_act));
            stack_cart.set_visible_child_name("cart");
        });
        window.add_action(&act_cart);

        let act_inst = gio::SimpleAction::new("page-installed", None);
        let list_inst = list_box.clone();
        let row_inst_act = row_installed.clone();
        let stack_inst = stack.clone();
        act_inst.connect_activate(move |_, _| {
            list_inst.select_row(Some(&row_inst_act));
            stack_inst.set_visible_child_name("installed");
        });
        window.add_action(&act_inst);

        let act_maint = gio::SimpleAction::new("page-maintenance", None);
        let list_maint = list_box.clone();
        let row_maint_act = row_maintenance.clone();
        let stack_maint = stack.clone();
        act_maint.connect_activate(move |_, _| {
            list_maint.select_row(Some(&row_maint_act));
            stack_maint.set_visible_child_name("maintenance");
        });
        window.add_action(&act_maint);

        let act_sett = gio::SimpleAction::new("page-settings", None);
        let list_sett = list_box.clone();
        let row_sett_act = row_settings.clone();
        let stack_sett = stack.clone();
        act_sett.connect_activate(move |_, _| {
            list_sett.select_row(Some(&row_sett_act));
            stack_sett.set_visible_child_name("settings");
        });
        window.add_action(&act_sett);

        let act_refresh = gio::SimpleAction::new("refresh", None);
        let inst_for_refresh = installed_state.clone();
        let refresh_cat_for_act = refresh_catalog.clone();
        let entry_for_refresh = search_entry.clone();
        act_refresh.connect_activate(move |_, _| {
            inst_for_refresh.refresh_background();
            let current_text = entry_for_refresh.text().to_string();
            refresh_cat_for_act(None, &current_text);
        });
        window.add_action(&act_refresh);

        let act_shortcuts = gio::SimpleAction::new("shortcuts", None);
        let win_weak_sc = window.downgrade();
        act_shortcuts.connect_activate(move |_, _| {
            if let Some(win) = win_weak_sc.upgrade() {
                ShortcutsHelpDialog::show(&win);
            }
        });
        window.add_action(&act_shortcuts);

        let act_about = gio::SimpleAction::new("about", None);
        let win_weak_ab = window.downgrade();
        act_about.connect_activate(move |_, _| {
            if let Some(win) = win_weak_ab.upgrade() {
                AboutAppDialog::show(&win);
            }
        });
        window.add_action(&act_about);

        let act_contribute = gio::SimpleAction::new("contribute", None);
        act_contribute.connect_activate(|_, _| {
            let _ = gio::AppInfo::launch_default_for_uri(
                APP_GITHUB_URL,
                None::<&gio::AppLaunchContext>,
            );
        });
        window.add_action(&act_contribute);

        let act_contribute_guide = gio::SimpleAction::new("contribute_guide", None);
        act_contribute_guide.connect_activate(|_, _| {
            let _ = gio::AppInfo::launch_default_for_uri(
                APP_CONTRIBUTING_URL,
                None::<&gio::AppLaunchContext>,
            );
        });
        window.add_action(&act_contribute_guide);

        let act_issues = gio::SimpleAction::new("issues", None);
        act_issues.connect_activate(|_, _| {
            let _ = gio::AppInfo::launch_default_for_uri(
                APP_ISSUES_URL,
                None::<&gio::AppLaunchContext>,
            );
        });
        window.add_action(&act_issues);

        // Capture fazında klavye dinleyicisi (tüm alt bileşen odaklanmalarından önce çalışır)
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);

        let search_entry_for_keys = search_entry.clone();
        let list_for_keys = list_box.clone();
        let stack_for_keys = stack.clone();
        let row_home_key = row_home.clone();
        let row_cat_key = row_catalog.clone();
        let row_cart_key = row_cart.clone();
        let row_inst_key = row_installed.clone();
        let row_maint_key = row_maintenance.clone();
        let row_sett_key = row_settings.clone();
        let inst_for_key_ref = installed_state.clone();
        let cat_ref_for_key = refresh_catalog.clone();
        let win_weak_key = window.downgrade();

        key_controller.connect_key_pressed(move |_, keyval, _keycode, state| {
            let is_ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
            if is_ctrl {
                match keyval {
                    gtk4::gdk::Key::f | gtk4::gdk::Key::F => {
                        search_entry_for_keys.grab_focus();
                        search_entry_for_keys.select_region(0, -1);
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::_1 | gtk4::gdk::Key::KP_1 => {
                        list_for_keys.select_row(Some(&row_home_key));
                        stack_for_keys.set_visible_child_name("home");
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::_2 | gtk4::gdk::Key::KP_2 => {
                        list_for_keys.select_row(Some(&row_cat_key));
                        stack_for_keys.set_visible_child_name("catalog");
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::_3 | gtk4::gdk::Key::KP_3 => {
                        list_for_keys.select_row(Some(&row_cart_key));
                        stack_for_keys.set_visible_child_name("cart");
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::_4 | gtk4::gdk::Key::KP_4 => {
                        list_for_keys.select_row(Some(&row_inst_key));
                        stack_for_keys.set_visible_child_name("installed");
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::_5 | gtk4::gdk::Key::KP_5 => {
                        list_for_keys.select_row(Some(&row_maint_key));
                        stack_for_keys.set_visible_child_name("maintenance");
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::comma => {
                        list_for_keys.select_row(Some(&row_sett_key));
                        stack_for_keys.set_visible_child_name("settings");
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::r | gtk4::gdk::Key::R => {
                        inst_for_key_ref.refresh_background();
                        cat_ref_for_key(None, &search_entry_for_keys.text());
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::question | gtk4::gdk::Key::slash => {
                        if let Some(win) = win_weak_key.upgrade() {
                            ShortcutsHelpDialog::show(&win);
                        }
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::q | gtk4::gdk::Key::Q => {
                        if let Some(win) = win_weak_key.upgrade() {
                            win.close();
                        }
                        return glib::Propagation::Stop;
                    }
                    _ => {}
                }
            } else {
                match keyval {
                    gtk4::gdk::Key::Escape => {
                        if !search_entry_for_keys.text().is_empty() {
                            search_entry_for_keys.set_text("");
                            search_entry_for_keys.grab_focus();
                            return glib::Propagation::Stop;
                        } else if search_entry_for_keys.has_focus() {
                            if let Some(win) = win_weak_key.upgrade() {
                                gtk4::prelude::GtkWindowExt::set_focus(&win, None::<&gtk4::Widget>);
                            }
                            return glib::Propagation::Stop;
                        }
                    }
                    gtk4::gdk::Key::F5 => {
                        inst_for_key_ref.refresh_background();
                        cat_ref_for_key(None, &search_entry_for_keys.text());
                        return glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::F1 => {
                        if let Some(win) = win_weak_key.upgrade() {
                            ShortcutsHelpDialog::show(&win);
                        }
                        return glib::Propagation::Stop;
                    }
                    _ => {}
                }
            }
            glib::Propagation::Proceed
        });
        window.add_controller(key_controller);

        window.set_content(Some(&main_layout));
        window
    }
}
