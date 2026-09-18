use crate::state::SettingsState;
use gtk4::prelude::*;
use gtk4::{Button, Switch, Align};
use libadwaita as adw;
use libadwaita::prelude::*;

pub struct SettingsPage;

impl SettingsPage {
    pub fn build(settings_state: SettingsState) -> adw::PreferencesPage {
        let page = adw::PreferencesPage::builder()
            .title("Ayarlar")
            .icon_name("emblem-system-symbolic")
            .build();

        let initial_data = settings_state.get();

        // 1. Görünüm ve Tema Grubu
        let appearance_group = adw::PreferencesGroup::builder()
            .title("Görünüm ve Tema")
            .description("Arayüz temasını ve renk tercihlerini özelleştirin.")
            .build();

        let theme_row = adw::ActionRow::builder()
            .title("Koyu Tema Tercihi")
            .subtitle(if initial_data.dark_theme {
                "Koyu tema aktif (Karanlık modern arayüz)"
            } else {
                "Açık tema aktif (Aydınlık arayüz)"
            })
            .build();

        let theme_switch = Switch::builder()
            .valign(Align::Center)
            .active(initial_data.dark_theme)
            .build();

        theme_row.add_suffix(&theme_switch);
        theme_row.set_activatable_widget(Some(&theme_switch));

        let settings_clone_for_theme = settings_state.clone();
        let theme_row_for_sub = theme_row.clone();
        theme_switch.connect_active_notify(move |sw| {
            let is_dark = sw.is_active();
            crate::utils::ThemeManager::apply_theme(is_dark);
            settings_clone_for_theme.set_dark_theme(is_dark);
            theme_row_for_sub.set_subtitle(if is_dark {
                "Koyu tema aktif (Karanlık modern arayüz)"
            } else {
                "Açık tema aktif (Aydınlık arayüz)"
            });
        });

        // Üst bardaki butonla ayarlar sayfasını senkronize tut
        let sw_theme_for_sync = theme_switch.clone();
        let row_theme_for_sync = theme_row.clone();
        settings_state.on_change(move |data| {
            if sw_theme_for_sync.is_active() != data.dark_theme {
                sw_theme_for_sync.set_active(data.dark_theme);
                row_theme_for_sync.set_subtitle(if data.dark_theme {
                    "Koyu tema aktif (Karanlık modern arayüz)"
                } else {
                    "Açık tema aktif (Aydınlık arayüz)"
                });
            }
        });

        appearance_group.add(&theme_row);
        page.add(&appearance_group);

        // 2. Paket Yönetimi ve AUR Grubu
        let pm_group = adw::PreferencesGroup::builder()
            .title("Paket Yönetimi ve AUR")
            .description("Arch Linux depoları ve AUR entegrasyonu kuralları.")
            .build();

        // PKGBUILD İnceleme Switch
        let aur_row = adw::ActionRow::builder()
            .title("PKGBUILD Önizlemesini Zorunlu Tut")
            .subtitle(if initial_data.require_pkgbuild_review {
                "Etkin: AUR paketleri kurulmadan önce PKGBUILD inceleme penceresi açılır"
            } else {
                "Devre dışı: PKGBUILD incelemesi atlanır, doğrudan kuruluma geçilir"
            })
            .build();

        let aur_switch = Switch::builder()
            .valign(Align::Center)
            .active(initial_data.require_pkgbuild_review)
            .build();

        aur_row.add_suffix(&aur_switch);
        aur_row.set_activatable_widget(Some(&aur_switch));

        let settings_clone_for_aur = settings_state.clone();
        let aur_row_for_sub = aur_row.clone();
        aur_switch.connect_active_notify(move |sw| {
            let active = sw.is_active();
            settings_clone_for_aur.set_require_pkgbuild_review(active);
            aur_row_for_sub.set_subtitle(if active {
                "Etkin: AUR paketleri kurulmadan önce PKGBUILD inceleme penceresi açılır"
            } else {
                "Devre dışı: PKGBUILD incelemesi atlanır, doğrudan kuruluma geçilir"
            });
        });
        pm_group.add(&aur_row);

        // Özet Onayı Switch
        let confirm_row = adw::ActionRow::builder()
            .title("İşlem Öncesi Özet Onayı")
            .subtitle(if initial_data.require_summary_confirmation {
                "Etkin: Kurulum öncesi paket özeti ve komut onay penceresi gösterilir"
            } else {
                "Devre dışı: Sepetten kurulum doğrudan başlatılır"
            })
            .build();

        let confirm_switch = Switch::builder()
            .valign(Align::Center)
            .active(initial_data.require_summary_confirmation)
            .build();

        confirm_row.add_suffix(&confirm_switch);
        confirm_row.set_activatable_widget(Some(&confirm_switch));

        let settings_clone_for_confirm = settings_state.clone();
        let confirm_row_for_sub = confirm_row.clone();
        confirm_switch.connect_active_notify(move |sw| {
            let active = sw.is_active();
            settings_clone_for_confirm.set_require_summary_confirmation(active);
            confirm_row_for_sub.set_subtitle(if active {
                "Etkin: Kurulum öncesi paket özeti ve komut onay penceresi gösterilir"
            } else {
                "Devre dışı: Sepetten kurulum doğrudan başlatılır"
            });
        });
        pm_group.add(&confirm_row);

        // AUR Yardımcısı Tercihi
        let helper_row = adw::ActionRow::builder()
            .title("Varsayılan AUR Yardımcısı")
            .subtitle("AUR paketlerini aramak ve derlemek için kullanılacak araç (paru / yay)")
            .build();

        let helper_toggle_btn = Button::builder()
            .label(&format!("Araç: {}", initial_data.aur_helper))
            .valign(Align::Center)
            .css_classes(["flat"])
            .build();

        let settings_clone_for_helper = settings_state.clone();
        let btn_clone = helper_toggle_btn.clone();
        helper_toggle_btn.connect_clicked(move |_| {
            let current = settings_clone_for_helper.get().aur_helper;
            let next = if current == "paru" { "yay" } else { "paru" };
            settings_clone_for_helper.set_aur_helper(next);
            btn_clone.set_label(&format!("Araç: {}", next));
        });

        helper_row.add_suffix(&helper_toggle_btn);
        pm_group.add(&helper_row);

        page.add(&pm_group);

        // 3. Sistem ve Bakım Grubu
        let maint_group = adw::PreferencesGroup::builder()
            .title("Sistem Bakımı")
            .description("Depo önbellekleri ve temizlik.")
            .build();

        let cache_row = adw::ActionRow::builder()
            .title("Paket Önbelleği Temizliği")
            .subtitle("İndirilen eski paket arşivlerini temizleyin (sudo pacman -Sc)")
            .build();

        let clean_btn = Button::builder()
            .label("Önbelleği Temizle")
            .valign(Align::Center)
            .css_classes(["destructive-action", "pill"])
            .build();

        let btn_clean_clone = clean_btn.clone();
        clean_btn.connect_clicked(move |_| {
            btn_clean_clone.set_sensitive(false);
            btn_clean_clone.set_label("Temizleniyor...");
            let btn = btn_clean_clone.clone();

            glib::spawn_future_local(async move {
                let _ = crate::process::CommandExecutor::run_captured("pkexec", &["pacman", "-Sc", "--noconfirm"]).await;
                btn.set_sensitive(true);
                btn.set_label("Temizlendi ✓");
                let b = btn.clone();
                glib::timeout_add_local_once(std::time::Duration::from_millis(2000), move || {
                    b.set_label("Önbelleği Temizle");
                });
            });
        });

        cache_row.add_suffix(&clean_btn);
        maint_group.add(&cache_row);
        page.add(&maint_group);

        // 4. Hakkında Grubu
        let about_group = adw::PreferencesGroup::builder()
            .title("Hakkında")
            .build();

        let app_info_row = adw::ActionRow::builder()
            .title("Aurora Linux Application Center")
            .subtitle("Sürüm 0.1.0 • GPL-3.0-or-later • Yapılandırma: ~/.config/aurora/settings.json")
            .build();
        about_group.add(&app_info_row);

        page.add(&about_group);

        page
    }
}
