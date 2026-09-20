# Aurora

Arch Linux ve Arch tabanlı dağıtımlar için GTK4 ve Libadwaita ile geliştirilmiş grafiksel paket ve uygulama yönetim merkezi.

<p align="center">
  <img src="assets/icons/org.aurora.ApplicationCenter.svg" width="128" height="128" alt="Aurora Logo" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Arch%20Linux%20%7C%20CachyOS%20%7C%20EndeavourOS%20%7C%20Manjaro-1793d1?style=flat-square&logo=arch-linux&logoColor=white" alt="Arch Linux" />
  <img src="https://img.shields.io/badge/Rust-2021-DEA584?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/GTK-4.12%2B-3584e4?style=flat-square&logo=gnome&logoColor=white" alt="GTK4 Libadwaita" />
  <img src="https://img.shields.io/badge/License-GPL--3.0--or--later-blue?style=flat-square" alt="GPL-3.0" />
</p>

> [!IMPORTANT]
> **Proje Durumu: Aktif Geliştirme Aşamasında (Alpha / Pre-release)**
> Aurora projesi şu anda **aktif geliştirme aşamasındadır**. Mimari, kullanıcı arayüzü, paket yönetim motoru ve sistem bakımı özellikleri hızla geliştirilmekte, yeni yetenekler eklenmektedir. Karşılaştığınız sorunları veya önerilerinizi paylaşarak ya da kod katkısında bulunarak projenin gelişimine ortak olabilirsiniz.
> 
> Ayrıntılı adımlar için [Katkıda Bulunma Rehberi'ni (CONTRIBUTING.md)](CONTRIBUTING.md) inceleyebilirsiniz.

---

## Genel Bakış

Aurora; resmi depolar (pacman), Arch User Repository (AUR - paru/yay) ve Flatpak paketlerini tek bir arayüzde yönetmeyi sağlayan bir masaüstü uygulamasıdır. Kullanıcıların paketleri tek tek kurup beklemek yerine bir sepete ekleyerek sırayla kurmasını, temel sistem paketlerini hazır setler halinde seçebilmesini ve kurulu uygulamaları doğrudan kaldırabilmesini sağlar.

## Özellikler

- **Toplu Kurulum Sepeti:** Seçilen paketleri kuyruğa ekleyerek tek oturumda toplu olarak kurabilme.
- **Toplu İçe & Dışa Aktarma:** Sepetteki paket listesini tek tıkla dışa aktarabilme ve hazır metin listelerinden (`packages.txt`) toplu içe aktarım yapabilme.
- **Hızlı Kurulum Paketleri:** Temel medya kodekleri, oyun uyumluluk katmanları (Wine, Mesa, Vulkan), ses sürücüleri ve geliştirici araçları setleri.
- **Çoklu Paket Yöneticisi:** Resmi depolar (Pacman), Arch User Repository (AUR helper ve bağımsız AUR RPC v5 desteği) ve Flatpak kaynaklarını entegre yönetebilme.
- **Sistem Güncelleme Denetimi:** `checkupdates` / `pacman -Qu` ile bekleyen paket güncellemelerini algılama ve arayüz içi terminal akışı ile tek tıkla sistemi yükseltme (`-Syu`).
- **Sistem Bakımı & Temizlik:** Pacman önbellek boyutunu ölçme (`paccache -r` / `-ruk0`), yetim paket tespiti ve güvenli temizliği (`pacman -Qtdq` ➔ `pacman -Rns`) ile başarısız systemd servislerini denetleme.
- **Kaynak Duyarlı Paket Kaldırma:** Kurulu uygulamaları arayüzden veya kurulu paketler sekmesinden doğrudan kaldırabilme. Pacman, AUR ve Flatpak ayrımı otomatik yapılır. Çekirdek sistem paketleri (`systemd`, `linux`, `glibc`, `pacman`) koruma altındadır.
- **Asenkron Yürütme:** Kurulum, arama, derleme ve bakım işlemleri Tokio thread havuzunda arka planda yürütülür; arayüz işlem sırasında akıcı kalır.
- **Parola Yönetimi:** `SUDO_ASKPASS` entegrasyonu sayesinde terminal gerektirmeden grafiksel parola penceresi (`kdialog` / `zenity`) ile yetkilendirme.
- **AUR İnceleme:** Kurulum öncesinde PKGBUILD derleme betiklerini doğrudan görüntüleyebilme.
- **Masaüstü Entegrasyonu:** Sistem temasına uyumlu Açık/Koyu tema desteği, FreeDesktop standartlarında akıllı simge çözümleme (`IconTheme`, sonek arındırma ve takma ad haritası).
- **Klavye Kısayolları:** Hızlı gezinme ve anında arama odaklaması için küresel kısayol desteği.

## Klavye Kısayolları

Aurora, hızlı gezinme ve işlem yönetimi için GTK4 ve Libadwaita'nın küresel eylem (Action) ve öncelikli yakalama (Capture) sistemini kullanır:

| Kısayol | Numpad | İşlev |
| :--- | :--- | :--- |
| `Ctrl + 1` | `Ctrl + KP 1` | **Keşfet** (Ana Sayfa) sayfasına geç |
| `Ctrl + 2` | `Ctrl + KP 2` | **Paket Kataloğu** sayfasına geç |
| `Ctrl + 3` | `Ctrl + KP 3` | **Kurulum Sepeti** sayfasına geç |
| `Ctrl + 4` | `Ctrl + KP 4` | **Kurulu Paketler & Güncellemeler** sayfasına geç |
| `Ctrl + 5` | `Ctrl + KP 5` | **Sistem Bakımı** sayfasına geç |
| `Ctrl + ,` | - | **Ayarlar** penceresine geç |
| `Ctrl + F` | - | Arama kutusuna odaklan (mevcut metni otomatik seçer) |
| `Esc` | - | Arama metnini temizle veya aramadan çık |
| `F5` / `Ctrl + R` | - | Sayfayı, kurulu paketleri ve kataloğu yenile |
| `F1` / `Ctrl + ?` | - | **Klavye Kısayolları Penceresi**'ni aç |
| `Ctrl + Q` | - | Aurora uygulamasından çık |

> [!TIP]
> Arayüz içindeyken istediğiniz zaman `F1` veya `Ctrl + ?` tuşlayarak veya sağ üstteki **Ana Menü (⋮)** butonundan **Klavye Kısayolları** kılavuzuna ulaşabilirsiniz.

## Sayfalar

| Sayfa | Açıklama |
| :--- | :--- |
| **Keşfet** | Sistem durumu, kategori kısayolları, hızlı kurulum paketleri ve öne çıkan uygulamalar. |
| **Katalog** | 86 temel uygulama ve sürücü; kategori filtreleme ve Pacman/AUR/Flatpak canlı arama. |
| **Kurulum Sepeti** | Seçilen paketlerin listesi, kaynak dağılımı, içe/dışa aktarma ve kurulum başlatıcı. |
| **Kurulu & Kaldır** | Sistemde yüklü Pacman, AUR ve Flatpak paketlerini listeleme, güncelleme denetimi ve güvenli kaldırma. |
| **Sistem Bakımı** | Pacman önbellek pruneri, yetim paket temizliği ve systemd servis sağlığı denetleyicisi. |
| **Ayarlar** | AUR yardımcısı tercihi (`paru`/`yay`), PKGBUILD güvenlik incelemesi ve renk teması seçimi. |

## Gereksinimler

- Arch Linux veya Arch tabanlı bir dağıtım (EndeavourOS, CachyOS, Manjaro vb.)
- Rust & Cargo 1.80+
- GTK4 (4.12+) ve Libadwaita (1.5+)
- İsteğe bağlı: `paru` veya `yay`, `flatpak`

## Kurulum ve Derleme

### Kaynak Koddan Çalıştırma
```bash
git clone https://github.com/topalemirfaruk/Aurora.git
cd Aurora
cargo run
```

### Testleri Çalıştırma
```bash
cargo test
```

### Sisteme Kurulum
Uygulamayı release modunda derleyip `/usr/local` dizinine ve masaüstü menünüze kaydeder:
```bash
chmod +x install.sh
./install.sh
```

### Sistemden Kaldırma
```bash
chmod +x uninstall.sh
./uninstall.sh
```

## Dizin Yapısı

```text
Aurora/
├── assets/                          # Vektörel SVG simgeleri
├── packaging/                       # .desktop ve AUR PKGBUILD şablonları
├── resources/                       # CSS temaları
├── src/                             # Rust kaynak kodları
│   ├── application.rs               # AdwApplication yaşam döngüsü
│   ├── models/                      # Veri modelleri
│   ├── package_managers/            # Pacman, AUR ve Flatpak modülleri
│   ├── process/                     # Asenkron komut yürütücü
│   ├── security/                    # Girdi doğrulama ve korumalı paketler
│   ├── services/                    # Uygulama kataloğu ve paket setleri
│   ├── state/                       # Sepet ve kurulu paket durum yönetimi
│   ├── ui/                          # Sayfalar ve bileşenler
│   └── utils/                       # Tema, askpass ve simge çözümleyici
├── tests/                           # Birim ve entegrasyon testleri
├── install.sh                       # Sisteme kurma betiği
├── uninstall.sh                     # Sistemden kaldırma betiği
├── Cargo.toml                       # Rust paket yapılandırması
├── LICENSE                          # GNU General Public License v3.0
├── CONTRIBUTING.md                  # Katkıda bulunma rehberi
└── README.md                        # Dokümantasyon
```

## 🤝 Katkıda Bulunma (Contributing)

Aurora, topluluk desteğiyle hızla gelişen açık kaynaklı bir projedir. Kod yazarak, hata bildirerek veya yeni fikirler sunarak projeyi birlikte büyütebiliriz:

1. **Hata Bildirimi (Bug Reports):** Bir sorunla karşılaştığınızda [GitHub Issues](https://github.com/topalemirfaruk/Aurora/issues) üzerinden detaylı bir hata kaydı oluşturabilirsiniz.
2. **Yeni Özellik & Paket Talepleri:** Kataloğa eklenmesini istediğiniz Linux uygulamalarını ve kullanıcı deneyimini iyileştirecek önerilerinizi bildirebilirsiniz.
3. **Kod Katkısı (Pull Requests):** Hata düzeltmeleri veya yeni özellikler için doğrudan kod katkısında bulunabilirsiniz.
4. **Farklı Dağıtımlarda Test:** EndeavourOS, Manjaro, CachyOS, Garuda veya Artix gibi Arch tabanlı dağıtımlarda Aurora'yı test ederek geri bildirimde bulunabilirsiniz.
5. **Çeviri:** Uygulamanın farklı dillere yerelleştirilmesi çalışmalarına katılabilirsiniz.

### Geliştirici Standartları:
- Gönderilecek kodların derleyici ve linter uyarıları içermemesi ve testlerden geçmesi beklenir:
  ```bash
  cargo clippy -- -D warnings
  cargo test
  cargo fmt --check
  ```
- Detaylı adımlar, mimari detaylar ve Git commit kuralları için lütfen [**Katkıda Bulunma Rehberi (CONTRIBUTING.md)**](CONTRIBUTING.md) dosyasını inceleyin.

## Lisans

Bu proje [GPL-3.0-or-later](LICENSE) altında lisanslanmıştır.
