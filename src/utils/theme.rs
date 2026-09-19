use gtk4::gdk::Display;
use gtk4::CssProvider;
use libadwaita as adw;
use std::cell::RefCell;

thread_local! {
    static THEME_PROVIDER: RefCell<Option<CssProvider>> = const { RefCell::new(None) };
}

pub struct ThemeManager;

impl ThemeManager {
    /// Hem Libadwaita katmanını hem de KDE/XFCE GTK4 CSS kurallarını ezen tema motoru.
    pub fn apply_theme(is_dark: bool) {
        let manager = adw::StyleManager::default();
        manager.set_color_scheme(if is_dark {
            adw::ColorScheme::ForceDark
        } else {
            adw::ColorScheme::ForceLight
        });

        if let Some(display) = Display::default() {
            let disp_manager = adw::StyleManager::for_display(&display);
            disp_manager.set_color_scheme(if is_dark {
                adw::ColorScheme::ForceDark
            } else {
                adw::ColorScheme::ForceLight
            });

            // Priority 850 overrides any custom priority 800 styles set by desktop environments
            let css_content = if is_dark {
                include_str!("../../resources/theme_dark.css")
            } else {
                include_str!("../../resources/theme_light.css")
            };

            THEME_PROVIDER.with(|cell| {
                let mut provider_opt = cell.borrow_mut();
                if provider_opt.is_none() {
                    let provider = CssProvider::new();
                    gtk4::style_context_add_provider_for_display(
                        &display,
                        &provider,
                        gtk4::STYLE_PROVIDER_PRIORITY_USER + 50,
                    );
                    *provider_opt = Some(provider);
                }

                if let Some(ref provider) = *provider_opt {
                    provider.load_from_string(css_content);
                }
            });
        }
    }
}
