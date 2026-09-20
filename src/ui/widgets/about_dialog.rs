use crate::config::{APP_CONTRIBUTING_URL, APP_GITHUB_URL, APP_ID, APP_ISSUES_URL, APP_NAME};
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

pub struct AboutAppDialog;

impl AboutAppDialog {
    pub fn show(parent: &impl IsA<gtk4::Widget>) {
        let dialog = adw::AboutDialog::builder()
            .application_name(APP_NAME)
            .application_icon(APP_ID)
            .version("0.1.0-alpha (Aktif Geliştirme)")
            .developer_name("Emir Faruk Topal")
            .website(APP_GITHUB_URL)
            .issue_url(APP_ISSUES_URL)
            .support_url(APP_CONTRIBUTING_URL)
            .comments("Arch Linux ve tabanlı sistemler için modern, hızlı ve güvenli grafiksel paket ve uygulama yönetim merkezi.\n\n⚠️ Bu yazılım aktif geliştirme aşamasındadır (Alpha). Topluluk katkılarına, hata bildirimlerine ve yeni özellik önerilerine tamamen açıktır.")
            .license_type(gtk4::License::Gpl30)
            .developers(["Emir Faruk Topal <topalemirfaruk@users.noreply.github.com>"])
            .build();

        dialog.present(Some(parent));
    }
}
