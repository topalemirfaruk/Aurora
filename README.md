# 🌌 Aurora — Modern Arch Linux Application Center

Aurora, Arch Linux ve Arch tabanlı dağıtımlar (EndeavourOS, CachyOS, Manjaro vb.) için tasarlanmış; modern, güvenli, hafif ve native **GTK4 + libadwaita** masaüstü uygulama merkezidir.

NinitArch konseptini modern masaüstü teknolojileri, gerçek zamanlı paket sorgulama ve güvenlik kontrolleri ile birleştirir.

---

## ✨ Temel Özellikler

- 🎨 **Modern Native GTK4 & libadwaita:** Sistemle kusursuz bütünleşen Adwaita tasarım dili, koyu/açık tema geçişi ve responsive düzen.
- 🏷️ **Çoklu Kaynak Desteği:** Resmi depolar (`pacman`), AUR (`paru`, `yay`) ve `flatpak` uygulamaları belirgin rozetlerle listelenir (`OFFICIAL`, `AUR`, `FLATPAK`).
- 🔍 **Akıllı Arama ve Kategoriler:** Uygulama adı, paket adı, kategori veya etiketlere göre anlık filtreleme.
- 🛒 **Kurulum Sepeti & Komut Önizlemesi:** İstediğiniz uygulamaları sepete ekleyin, oluşturulan kurulum komutlarını tek tıkla kopyalayın veya doğrudan canlı konsoldan çalıştırın.
- 🚀 **Canlı Asenkron Kurulum Konsolu:** Arka planda donmayan Tokio süreçleri ile stdout/stderr canlı log akışı, ilerleme çubuğu ve tamamlama bildirimleri.
- 🔒 **Sıfır Komut Enjeksiyonu Riski:** Paket isimleri katı regex kurallarıyla denetlenir (`security::is_valid_package_name`), kabuk birleştirmeleri (`sh -c`) asla kullanılmaz.
- 📄 **Canlı PKGBUILD İnceleme:** AUR uygulamaları için derleme scripti ve kaynak URL'leri tek tıkla pencere içinde açılıp incelenebilir.
- 💾 **Canlı Kurulu Paket Yönetimi:** Sisteminizde kurulu olan paketler dinamik taranır, anlık aranır ve yönetilebilir.

---

## 🛠️ Sistem Gereksinimleri

- **Arch Linux** veya Arch tabanlı dağıtım (EndeavourOS, CachyOS, Manjaro vb.)
- **Rust & Cargo** (1.80+)
- **GTK4** (4.12+)
- **libadwaita** (1.5+)
- İsteğe bağlı: `paru` veya `yay` (AUR desteği için), `flatpak`

---

## 🚀 Çalıştırma ve Kurulum

### Geliştirme Modunda Çalıştırma:
```bash
cd /home/emirft/Masaüstü/Aurora
cargo run
```

### Testleri Çalıştırma:
```bash
cargo test
```

### Sisteme Kurulum:
Hazırlanan pratik kurulum scripti ile uygulamayı, masaüstü simgesini ve `.desktop` dosyasını sisteminize kurabilirsiniz:
```bash
./install.sh
```

---

## 📂 Mimari Yapı

```text
aurora/
├── Cargo.toml          # Rust bağımlılıkları (gtk4, libadwaita, tokio, serde, which)
├── install.sh          # Sisteme kurma scripti
├── resources/
│   └── style.css       # Aurora mor/mavi Adwaita tema stilleri
├── assets/
│   └── icons/          # Modern SVG uygulama logosu
├── packaging/
│   ├── aur/PKGBUILD    # Arch Linux AUR resmi paket şablonu
│   └── desktop/        # XDG .desktop uygulama kısayolu
├── src/
│   ├── main.rs         # Giriş noktası ve loglama
│   ├── lib.rs          # Kütüphane kökü
│   ├── application.rs  # AdwApplication yaşam döngüsü ve CSS yönetimi
│   ├── config.rs       # Uygulama sabitleri (ID: org.aurora.ApplicationCenter)
│   ├── models/         # AppItem, PackageSource, PackageDetail modelleri
│   ├── package_managers/ # Pacman, AUR ve Flatpak API servisleri
│   ├── process/        # Canlı asenkron komut yürütücü (CommandExecutor)
│   ├── security/       # Paket adı validatörü ve komut enjeksiyon engeli
│   ├── services/       # Katalog servisi ve arama motoru
│   ├── state/          # Reaktif sepet (CartState) durumu
│   └── ui/
│       ├── window.rs   # Ana pencere, Sidebar ve HeaderBar
│       ├── pages/      # Keşfet, Katalog, Sepet, Kurulu ve Ayarlar sayfaları
│       └── widgets/    # AppCard, InstallDialog, PkgbuildDialog bileşenleri
└── tests/              # Otomatize birim ve güvenlik testleri
```

---

## 📜 Lisans

Bu proje **GPL-3.0-or-later** lisansı ile lisanslanmıştır.
