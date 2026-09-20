use crate::config::APP_ID;
use crate::ui::MainWindow;
use gio::prelude::*;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::CssProvider;
use libadwaita as adw;

pub struct Application {
    app: adw::Application,
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    pub fn new() -> Self {
        let app = adw::Application::builder()
            .application_id(APP_ID)
            .build();

        app.set_accels_for_action("win.search", &["<Primary>f"]);
        app.set_accels_for_action("win.page-home", &["<Primary>1", "<Primary>KP_1"]);
        app.set_accels_for_action("win.page-catalog", &["<Primary>2", "<Primary>KP_2"]);
        app.set_accels_for_action("win.page-cart", &["<Primary>3", "<Primary>KP_3"]);
        app.set_accels_for_action("win.page-installed", &["<Primary>4", "<Primary>KP_4"]);
        app.set_accels_for_action("win.page-maintenance", &["<Primary>5", "<Primary>KP_5"]);
        app.set_accels_for_action("win.page-settings", &["<Primary>comma"]);
        app.set_accels_for_action("win.refresh", &["F5", "<Primary>r"]);
        app.set_accels_for_action("win.shortcuts", &["<Primary>question", "F1"]);
        app.set_accels_for_action("win.about", &["<Primary>slash"]);
        app.set_accels_for_action("app.quit", &["<Primary>q"]);

        let quit_action = gio::SimpleAction::new("quit", None);
        let app_weak = app.downgrade();
        quit_action.connect_activate(move |_, _| {
            if let Some(app) = app_weak.upgrade() {
                app.quit();
            }
        });
        app.add_action(&quit_action);

        app.connect_startup(|_| {
            let settings = crate::state::SettingsState::new();
            crate::utils::ThemeManager::apply_theme(settings.get().dark_theme);
            Self::load_css();
            Self::setup_icons();
        });

        app.connect_activate(|app| {
            let window = MainWindow::build(app);
            window.present();
        });

        Self { app }
    }

    fn setup_icons() {
        if let Some(display) = Display::default() {
            let icon_theme = gtk4::IconTheme::for_display(&display);
            icon_theme.add_search_path("/usr/share/icons/hicolor");
            icon_theme.add_search_path("/usr/share/icons/Papirus");
            icon_theme.add_search_path("/usr/share/icons/Papirus-Dark");
            icon_theme.add_search_path("/usr/share/pixmaps");
            if let Some(home) = std::env::var_os("HOME") {
                let user_icons = std::path::PathBuf::from(home).join(".local/share/icons/hicolor");
                icon_theme.add_search_path(user_icons);
            }
            icon_theme.add_search_path("assets/icons");
            icon_theme.add_search_path("assets/icons/hicolor");
            if let Ok(exe) = std::env::current_exe() {
                if let Some(dir) = exe.parent() {
                    icon_theme.add_search_path(dir.join("assets/icons"));
                    icon_theme.add_search_path(dir.join("../assets/icons"));
                    icon_theme.add_search_path(dir.join("../../assets/icons"));
                    icon_theme.add_search_path(dir.join("../../assets/icons/hicolor"));
                }
            }
        }
    }

    fn load_css() {
        let provider = CssProvider::new();
        // Stil dosyasını yükle
        let css_content = include_str!("../resources/style.css");
        provider.load_from_string(css_content);

        if let Some(display) = Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    pub fn run(&self) -> glib::ExitCode {
        self.app.run()
    }
}
