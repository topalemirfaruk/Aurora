# 🌌 Aurora — Modern Arch Linux Uygulama ve Paket Yönetim Merkezi

<p align="center">
  <img src="assets/icons/org.aurora.ApplicationCenter.svg" width="128" height="128" alt="Aurora Logo" />
</p>

<p align="center">
  <strong>Arch Linux ve Arch tabanlı dağıtımlar için ultra hızlı, güvenli, modern ve native masaüstü yazılım mağazası.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Arch%20Linux%20%7C%20CachyOS%20%7C%20EndeavourOS%20%7C%20Manjaro-1793d1?style=for-the-badge&logo=arch-linux&logoColor=white" alt="Arch Linux" />
  <img src="https://img.shields.io/badge/Language-Rust%202021-DEA584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/UI-GTK4%20%2B%20Libadwaita-3584e4?style=for-the-badge&logo=gnome&logoColor=white" alt="GTK4 Libadwaita" />
  <img src="https://img.shields.io/badge/License-GPL--3.0--or--later-blue?style=for-the-badge" alt="GPL-3.0" />
</p>

---

## 🧭 Aurora Nedir?

**Aurora**, Arch Linux ekosistemindeki kullanıcıların terminal komutlarına bağımlı kalmadan; uygulamaları, geliştirme ortamlarını, oyunları ve sistem araçlarını **keşfetmesini, toplu olarak kurmasını ve kolayca kaldırmasını (uninstall)** sağlayan yeni nesil bir masaüstü yazılım merkezidir.

### 🎯 Neden Aurora?
Arch Linux kullanıcıları geleneksel olarak iki seçenek arasında sıkışmaktadır:
1. **Terminal Komutları:** Hızlı ve güçlüdür ancak onlarca paketi aramak, bağımlılıkları denetlemek ve toplu kurulumları yönetmek yeni ve orta seviye kullanıcılar için zordur.
2. **Mevcut Grafiksel Paket Yöneticileri (Pamac, Discover, GNOME Software):** Çoğu zaman aşırı bellek tüketir, arayüzleri AUR derlemeleri sırasında kilitlenir veya terminaldeki şifre isteklerinde (`sudo`) donup kalırlar.

**Aurora**, bu sorunları kökten çözmek için **Rust**'ın sıfır maliyetli soyutlamaları ve **GTK4 + Libadwaita**'nın akıcı arayüz yetenekleriyle sıfırdan geliştirilmiştir. Sistem kaynaklarını minimum düzeyde tüketir, arayüzü asla dondurmaz ve kullanıcısına hem görsel bir mağaza deneyimi hem de tam kontrol sunar.

---

## ✨ Öne Çıkan Özellikler

### 1. 🛍️ Toplu Kurulum Sepeti & Ninite Modu (Batch Queue & Ninite Bundles)
- **Ninite Felsefesi (1-Tıkla Toplu Kurulum Paketleri):** Yeni kurulmuş bir Arch Linux sisteminde ihtiyaç duyulan multimedya kodekleri, oyun uyumluluk katmanları (Wine, Mesa, Vulkan), ses/bluetooth sürücüleri ve geliştirici dillerini tek tıkla sepete ekleyip kurabilme.
- **Toplu Kurulum Sepeti:** Tek tek uygulama kurup beklemek yerine, istediğiniz tüm uygulamaları sepetinize ekleyin.
- Seçtiğiniz paketlerin resmi repo, AUR veya Flatpak dağılımını özet halinde görün.
- Arka planda donmayan tek bir akış ile tüm sepeti sırayla kurun.

### 2. 🎮 Donanım Sürücüleri, Medya Kodekleri ve Çalışma Zamanları
- **Sürücüler & Donanım:** `mesa`, `vulkan-radeon`, `vulkan-intel`, `nvidia-open`, `nvidia-utils`, `pipewire-pulse`, `wireplumber`, `bluez`, `cups`.
- **Medya Kodekleri:** MP4, MKV, AV1 ve tüm formatlar için `ffmpeg`, `gst-plugins-good`, `gst-plugins-bad`, `gst-plugins-ugly`, `gst-libav`, `intel-media-driver`, `libva-mesa-driver`, `dav1d`, `x264`, `x265`.
- **Çalışma Zamanları & Uyumluluk:** Windows yazılımları ve modern oyunlar için `wine`, `winetricks`, `gamemode`, `gamescope` ile `jdk-openjdk`, `nodejs`, `python`, `dotnet-sdk`.

### 3. 🗑️ Doğrudan ve Güvenli Paket Kaldırma (Uninstall System)
- **Karttan Doğrudan Kaldırma:** Sisteminizde kurulu olan herhangi bir uygulamanın kartında anında kırmızı `🗑️ Kaldır` butonu belirir.
- **Detay Sayfasından Kaldırma:** Uygulama detayında kurulu uygulamalar için ana eylem butonu otomatik olarak `🗑️ Sistemden Kaldır` haline gelir.
- **Kurulu Olanlar Merkezi:** Sistemde kurulu tüm paketleri anlık olarak listeleyin, filtreleyin ve tek tıkla sistemden temizleyin.
- **Kritik Çekirdek Koruması:** `linux`, `systemd`, `glibc`, `pacman` gibi hayati sistem bileşenlerinin yanlışlıkla kaldırılması Aurora'nın güvenlik kalkanı tarafından kesin olarak engellenir.

### 4. 📦 Üçlü Ekosistem Desteği (Official + AUR + Flatpak)
- **Resmi Depolar (Pacman):** Arch Linux resmi depolarındaki kararlı paketler (`OFFICIAL` rozeti).
- **Arch User Repository (AUR):** `paru` veya `yay` aracılığıyla topluluk paketlerini tek tıkla derleme ve kurma (`AUR` rozeti).
- **Flatpak:** İzolasyonlu, sandbox tabanlı modern masaüstü uygulamaları (`FLATPAK` rozeti).

### 5. ⚡ Canlı Asenkron Yürütme ve Akıllı Askpass Konsolu
- **Donmayan UI:** GTK ana döngüsü ile Tokio thread havuzu bağımsız çalışır; arayüz işlem sürerken %100 tepkisel kalır.
- **Grafiksel Parola Doğrulama (`SUDO_ASKPASS`):** Terminal şifre isteklerinde (`[sudo] password:`) uygulamanın takılı kalması engellenmiştir; sistem KDE (`kdialog`) veya GNOME (`zenity`) grafik penceresiyle şifre onayını güvenle alır.
- **Canlı Log Akışı:** Derleme ve indirme adımları renkli terminal çıktısıyla gerçek zamanlı akar, otomatik en alta kayar.

### 6. 🛡️ Siber Güvenlik ve Katı Doğrulama
- **Enjeksiyon Koruması:** Paket adları katı regex kurallarıyla denetlenir; `&&`, `;`, `|`, `$(...)` gibi zararlı kabuk enjeksiyonları filtrelenir.
- **PKGBUILD Önizleme:** AUR paketlerinin derleme scriptleri ve kaynak kod adresleri kurulmadan önce dahili editörde satır satır incelenebilir.
- **Root Uyarı Kalkanı:** Aurora root (`sudo`) olarak başlatıldığında AUR derleyicilerinin güvenlik açıklarına karşı kullanıcıyı uyaran güvenlik banner'ı devreye girer.

### 7. 🎨 Kusursuz Görsel Deneyim ve Akıllı İkonlar
- **Özel Vektörel Simgeler:** Boş sepet durumunda veya uygulama simgelerinde bulanık/düşük pikselli bitmap'ler yerine yüksek çözünürlüklü özel SVG vektörleri (`aurora-empty-cart.svg`, `org.aurora.ApplicationCenter.svg`) kullanılır.
- **Gelişmiş Simge Çözümleyici (`IconResolver`):** Brave, KeePassXC, VS Code, Discord, Spotify, Vulkan, NVIDIA, Wine gibi uygulamaların masaüstü simge takma adları dinamik olarak çözümlenir.
- **KDE Plasma & GNOME Uyumlu Tema Motoru:** Kullanıcının yerel GTK konfigürasyonunu ezmeden çalışan yüksek öncelikli dinamik CSS sağlayıcısı ile kusursuz Açık ve Koyu tema desteği.

### 8. 📋 Bellek İçi Reaktif Durum Takibi (`InstalledState`)
- `pacman -Q` ve `flatpak list` verilerini arka planda önbelleğe alarak \(O(1)\) hızında kurulu paket kontrolü yapar.
- Bir paket kurulduğunda veya kaldırıldığında tüm sayfalar (`Keşfet`, `Katalog`, `Kurulu Olanlar`) kullanıcı arayüzü yenilemeye gerek kalmadan anında güncellenir.

---

## 🗂️ Sayfa ve Gezinti Mimarisi

| Sayfa | Açıklama |
| :--- | :--- |
| **🏠 Keşfet (Home)** | Sistem yetenek hapları, kategori kısayolları, **⚡ Hızlı Kurulum Paketleri (Ninite Modu)** ve öne çıkan popüler uygulamalar. |
| **📚 Katalog (Catalog)** | 86 adet temel uygulama, sürücü, kodek ve çalışma zamanı; 9 kategori filtresi, anlık arama ve depolarda canlı arama. |
| **🛒 Kurulum Sepeti (Cart)** | Toplu kurulacak paketlerin listesi, kaynak dağılım özeti ve canlı kurulum başlatıcı. |
| **🗑️ Kurulu & Kaldır (Installed)** | Sistemdeki kurulu paketleri filtreleme, sürüm görüntüleme ve tek tıkla sistemden kaldırma. |
| **⚙️ Ayarlar (Settings)** | AUR yardımcısı tercihi (`paru`/`yay`), PKGBUILD zorunlu inceleme ayarı, özet onay penceresi ve tema seçimi. |

---

## 🛠️ Sistem Gereksinimleri

- **İşletim Sistemi:** Arch Linux, CachyOS, EndeavourOS, Manjaro veya herhangi bir Arch tabanlı dağıtım
- **Derleyici & Araçlar:** Rust & Cargo 1.80+
- **Grafik Kütüphaneleri:** GTK4 (4.12+) ve libadwaita (1.5+)
- **İsteğe Bağlı:** `paru` veya `yay` (AUR desteği için), `flatpak`

---

## 🚀 Kurulum ve Çalıştırma

### 1. Depoyu Klonlayın:
```bash
git clone https://github.com/topalemirfaruk/Aurora.git
cd Aurora
```

### 2. Geliştirme Ortamında Çalıştırma:
```bash
cargo run
```

### 3. Otomatik Testleri Çalıştırma:
```bash
cargo test
```
*(Güvenlik doğrulamaları, Ninite paket bütünlüğü, sepet yönetimi, ayarlar ve 86 paketlik katalog testleri dahil 19 testin tamamı otomatize olarak koşulur).*

### 4. Sisteme Kalıcı Kurulum:
Hazırlanan kurulum betiği uygulamayı optimize release modunda derler, masaüstü kısayolunu (`.desktop`) ve yüksek çözünürlüklü SVG logolarını sisteminize kaydeder:
```bash
chmod +x install.sh
./install.sh
```
Kurulum tamamlandıktan sonra uygulama menünüzden **"Aurora"** olarak başlatabilir veya terminalde doğrudan `aurora` komutunu verebilirsiniz.

### 5. Sistemden Kaldırma (Uninstall):
Aurora'yı, masaüstü kısayollarını, sistem ve kullanıcı simgelerini temizlemek için hazır kaldırma betiğini çalıştırabilirsiniz:
```bash
chmod +x uninstall.sh
./uninstall.sh
```
*(İsteğe bağlı olarak `~/.config/aurora` yapılandırma dizinini de temizleme seçeneği sunulur).*

---

## 📁 Proje Dizin Yapısı

```text
Aurora/
├── assets/
│   └── icons/                       # Aurora marka SVG ve vektörel sepet simgeleri
│       ├── aurora-empty-cart.svg
│       └── org.aurora.ApplicationCenter.svg
├── packaging/
│   ├── aur/PKGBUILD                 # Arch Linux resmi AUR paketleme şablonu
│   └── desktop/                     # XDG masaüstü entegrasyon dosyası (.desktop)
├── resources/
│   ├── style.css                    # Aurora Adwaita stil ve rozet tanımları
│   ├── theme_dark.css               # Koyu tema renk paleti
│   └── theme_light.css              # Açık tema renk paleti
├── src/
│   ├── main.rs                      # Giriş noktası ve Tokio çalışma zamanı başlatıcısı
│   ├── lib.rs                       # Modül kök tanımları
│   ├── application.rs               # AdwApplication yaşam döngüsü ve ikon kayıtları
│   ├── config.rs                    # Sabitler (Uygulama Adı, Sürüm, App ID)
│   ├── models/                      # AppItem, PackageSource, AppCategory veri modelleri
│   ├── package_managers/            # Pacman, AUR ve Flatpak sistem çağrıları
│   ├── process/                     # Canlı asenkron komut yürütücü (CommandExecutor)
│   ├── security/                    # Paket adı doğrulama, regex filtreleme ve root kalkanı
│   ├── services/                    # 86 uygulamalı zenginleştirilmiş katalog ve arama
│   ├── state/                       # Reaktif sepet (CartState), ayarlar ve kurulu durum (InstalledState)
│   ├── ui/
│   │   ├── window.rs                # Ana pencere, kenar çubuğu ve başlık çubuğu
│   │   ├── pages/                   # Keşfet, Katalog, Sepet, Kurulu Olanlar ve Ayarlar
│   │   └── widgets/                 # AppCard, InstallDialog, UninstallDialog, PkgbuildDialog
│   └── utils/                       # Sudo Askpass yardımcısı, tema motoru ve IconResolver
├── tests/                           # Birim, entegrasyon ve siber güvenlik testleri
├── install.sh                       # Sisteme tek tıkla kurma betiği
├── uninstall.sh                     # Sistemden tamamen kaldırma betiği
├── Cargo.toml                       # Rust bağımlılık yapılandırması
└── README.md                        # Proje dokümantasyonu
```

---

## 🔒 Güvenlik Politikası

Aurora, Arch Linux kullanıcılarının sistem bütünlüğünü korumak için tasarlanmıştır:
- Sistem kabuğuna (`sh -c`) asla parametresiz komut dizgisi gönderilmez; tüm süreçler `tokio::process::Command` ile bağımsız argüman vektörleri olarak çalıştırılır.
- Paket isimleri Linux dosya sistemi ve paket yöneticisi standartlarına (`^[a-zA-Z0-9][a-zA-Z0-9@._+-]*$`) göre sıkı bir denetimden geçer.
- Sistem için kritik öneme sahip çekirdek paketler silinemez listesinde tutulur.

---

## 📜 Lisans

Bu proje **GNU General Public License v3.0 or later (GPL-3.0-or-later)** altında lisanslanmıştır. Detaylar için lisans metnini inceleyebilirsiniz.

