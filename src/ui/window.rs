use crate::config::{APP_NAME, APP_VERSION};
use crate::models::{AppCategory, AppItem};
use crate::security::is_running_as_root;
use crate::state::CartState;
use crate::ui::pages::{AppDetailPage, CartPage, CatalogPage, HomePage, InstalledPage, SettingsPage};
use crate::ui::widgets::InstallDialog;
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
            .title(&format!("{} v{}", APP_NAME, APP_VERSION))
            .default_width(1100)
            .default_height(720)
            .build();

        let cart_state = CartState::new();
        let settings_state = crate::state::SettingsState::new();
        let installed_state = crate::state::InstalledState::new();

        // Başlangıç temasını ayarlardan uygula
        crate::utils::ThemeManager::apply_theme(settings_state.get().dark_theme);

        // Ana Stack (Sayfa Yığını)
        let stack = Stack::builder()
            .transition_type(gtk4::StackTransitionType::Crossfade)
            .hexpand(true)
            .vexpand(true)
            .build();

        // 1. Arama Girişi
        let search_entry = SearchEntry::builder()
            .placeholder_text("Uygulama, paket adı veya etiket ara...")
            .width_request(280)
            .build();

        // 2. Sayfaları Oluştur
        // Detay açma callback'i
        let window_weak = window.downgrade();
        let cart_for_detail = cart_state.clone();
        let installed_for_detail = installed_state.clone();
        let on_detail = move |item: AppItem| {
            if let Some(win) = window_weak.upgrade() {
                AppDetailPage::show(&win, item, cart_for_detail.clone(), installed_for_detail.clone());
            }
        };

        // Katalog sayfasını kur
        let (catalog_scroll, refresh_catalog) =
            CatalogPage::build(cart_state.clone(), installed_state.clone(), on_detail.clone());

        // Ana sayfadan kategoriye tıklanınca kataloğa gitme
        let stack_for_cat = stack.clone();
        let refresh_cat = refresh_catalog.clone();
        let on_cat_clicked = move |cat: AppCategory| {
            stack_for_cat.set_visible_child_name("catalog");
            refresh_cat(Some(cat), "");
        };

        let home_scroll = HomePage::build(
            cart_state.clone(),
            installed_state.clone(),
            on_detail.clone(),
            on_cat_clicked,
        );

        // Kurulum Başlatma Eylemi
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
        let installed_scroll = InstalledPage::build(installed_state.clone());
        let settings_page = SettingsPage::build(settings_state.clone());

        stack.add_named(&home_scroll, Some("home"));
        stack.add_named(&catalog_scroll, Some("catalog"));
        stack.add_named(&cart_scroll, Some("cart"));
        stack.add_named(&installed_scroll, Some("installed"));
        stack.add_named(&settings_page, Some("settings"));

        // Arama kutusu değiştiğinde kataloğa yönlendir
        let stack_for_search = stack.clone();
        let refresh_search = refresh_catalog.clone();
        search_entry.connect_search_changed(move |entry| {
            let text = entry.text().to_string();
            if !text.is_empty() {
                stack_for_search.set_visible_child_name("catalog");
            }
            refresh_search(None, &text);
        });

        // Üst Başlık Çubuğu (HeaderBar)
        let header_bar = adw::HeaderBar::builder()
            .title_widget(&search_entry)
            .build();

        // Sepet Butonu (Sağ üst)
        let cart_header_btn = Button::builder()
            .css_classes(["flat"])
            .tooltip_text("Kurulum Sepetini Aç")
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
        cart_header_btn.connect_clicked(move |_| {
            stack_for_cart_btn.set_visible_child_name("cart");
        });

        // Tema Aç/Kapa Butonu
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

        header_bar.pack_end(&theme_toggle_btn);
        header_bar.pack_end(&cart_header_btn);

        // Siber Güvenlik: Root Güvenlik Uyarısı Banner'ı
        let root_banner = adw::Banner::builder()
            .title("⚠️ Aurora root olarak çalıştırılıyor! Güvenlik ve AUR uyumluluğu için normal kullanıcı olarak başlatın.")
            .revealed(is_running_as_root())
            .build();

        // Sol Kenar Çubuğu (Sidebar)
        let sidebar_box = Box::builder()
            .orientation(Orientation::Vertical)
            .width_request(220)
            .css_classes(["sidebar"])
            .build();

        // Sidebar Logo / Başlık
        let brand_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .margin_start(16)
            .margin_end(16)
            .margin_top(16)
            .margin_bottom(16)
            .build();

        let logo_icon = gtk4::Image::from_icon_name("system-software-install");
        logo_icon.set_pixel_size(24);

        let brand_label = Label::builder()
            .label(APP_NAME)
            .css_classes(["title-3"])
            .halign(Align::Start)
            .build();

        brand_box.append(&logo_icon);
        brand_box.append(&brand_label);
        sidebar_box.append(&brand_box);

        // Sidebar Menü ListBox
        let list_box = ListBox::builder()
            .css_classes(["navigation-sidebar"])
            .selection_mode(gtk4::SelectionMode::Single)
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
            unsafe {
                row.set_data("page_name", page_name.to_string());
            }
            row
        }

        let row_home = create_sidebar_row("user-home-symbolic", "Keşfet", "home");
        let row_catalog = create_sidebar_row("view-app-grid-symbolic", "Katalog", "catalog");
        let row_cart = create_sidebar_row(crate::utils::IconResolver::cart_icon(), "Kurulum Sepeti", "cart");
        let row_installed = create_sidebar_row("drive-harddisk-symbolic", "Kurulu Olanlar", "installed");
        let row_settings = create_sidebar_row("emblem-system-symbolic", "Ayarlar", "settings");

        list_box.append(&row_home);
        list_box.append(&row_catalog);
        list_box.append(&row_cart);
        list_box.append(&row_installed);
        list_box.append(&row_settings);

        // İlk satırı seç
        list_box.select_row(Some(&row_home));

        let stack_for_nav = stack.clone();
        list_box.connect_row_selected(move |_, selected_row| {
            if let Some(row) = selected_row {
                unsafe {
                    if let Some(page_name) = row.data::<String>("page_name") {
                        stack_for_nav.set_visible_child_name(page_name.as_ref());
                    }
                }
            }
        });

        sidebar_box.append(&list_box);

        // Sepet rozetini güncelleme
        let badge_clone = cart_badge_label.clone();
        cart_state.on_change(move |count| {
            badge_clone.set_label(&count.to_string());
        });

        // Ana Gövde Düzeni (Split Horizontal: Sol Sidebar + Sağ Stack)
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

        // Ana Pencere Dikey Kutu: HeaderBar + Root Banner + Body
        let main_layout = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();

        main_layout.append(&header_bar);
        main_layout.append(&root_banner);
        main_layout.append(&body_box);

        window.set_content(Some(&main_layout));
        window
    }
}
