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
            .release_notes_version("0.1.0-alpha")
            .release_notes("<p>Aurora v0.1.0-alpha sürümü ile gelen temel özellikler ve yenilikler:</p>\
                <ul>\
                    <li><b>Sistem Bakımı:</b> Pacman önbellek boyutu ölçümü, paccache ile temizlik, yetim paket (pacman -Qtdq) tespiti ve güvenli temizliği, başarısız systemd servislerini listeleme ve yeniden başlatma.</li>\
                    <li><b>Klavye Kısayolları (Capture Modu):</b> Ctrl+1..5 ve Numpad tuşları ile sayfa geçişleri, Ctrl+F ile arama odaklaması ve metin seçimi, Esc ile arama temizleme, F5 ile yenileme, F1 ile kısayol kılavuzu.</li>\
                    <li><b>Sepet İçe ve Dışa Aktarımı:</b> Seçili paketleri düz metin (.txt) olarak dışa aktarma ve panodan hazır paket listelerini tek tıkla sepete ekleme.</li>\
                    <li><b>Çoklu Paket Yönetimi:</b> Resmi Pacman depoları, AUR (paru, yay ve yerel AUR RPC v5 desteği) ve Flatpak kaynaklarını entegre arama ve kaynak duyarlı kaldırma.</li>\
                    <li><b>Ninite Hızlı Kurulum Modu:</b> Donanım sürücüleri, medya kodekleri, oyun altyapısı ve geliştirici araçları için 1-tıkla toplu kurulum setleri.</li>\
                    <li><b>Gelişmiş İkon Çözümleyici:</b> FreeDesktop hicolor ve sistem ikon temalarından otomatik simge çözümleme ve takma ad desteği.</li>\
                </ul>")
            .build();

        dialog.present(Some(parent));
    }
}
