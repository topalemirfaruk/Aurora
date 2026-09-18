use crate::config::APP_ID;
use crate::ui::MainWindow;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::CssProvider;
use libadwaita as adw;

pub struct Application {
    app: adw::Application,
}

impl Application {
    pub fn new() -> Self {
        let app = adw::Application::builder()
            .application_id(APP_ID)
            .build();

        app.connect_startup(|_| {
            let settings = crate::state::SettingsState::new();
            crate::utils::ThemeManager::apply_theme(settings.get().dark_theme);
            Self::load_css();
        });

        app.connect_activate(|app| {
            let window = MainWindow::build(app);
            window.present();
        });

        Self { app }
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
