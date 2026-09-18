# Aurora — Modern Arch Linux Application Center
## Proje Detay Dokümanı (`projedetay.md`)

> **Proje türü:** Linux masaüstü uygulaması  
> **Hedef dağıtımlar:** Arch Linux ve Arch tabanlı dağıtımlar  
> **Önerilen teknoloji:** Rust + GTK4 + libadwaita  
> **Paketleme:** Flatpak (opsiyonel), native package, AUR  
> **Lisans önerisi:** GPL-3.0-or-later  
> **Belge sürümü:** 1.0  
> **Durum:** Tasarım ve geliştirme planı

---

## 1. Proje Özeti

Bu proje, Arch Linux kullanıcılarının uygulamaları, geliştirme araçlarını, oyunları, sistem yardımcılarını ve günlük kullanım yazılımlarını modern, güvenli ve anlaşılır bir masaüstü arayüzü üzerinden keşfetmesini ve kurmasını sağlayan bir uygulama merkezi olacaktır.

Proje, referans alınan **NinitArch** uygulamasının temel fikrini geliştirir:

- Kategorilere ayrılmış uygulama listeleri
- Birden fazla uygulama seçebilme
- AUR yardımcısı seçimi
- Seçilen paketler için kurulum komutu üretme
- Hafif ve hızlı yapı

Ancak yeni sürüm yalnızca bir kurulum komutu oluşturucu olmayacaktır. Uygulama, Arch Linux ekosistemini masaüstü deneyimiyle birleştiren, gerçek paket durumunu okuyabilen, güvenlik kontrolleri uygulayan, kurulum sürecini izlenebilir hâle getiren ve modern GTK tasarım dilini kullanan tam teşekküllü bir **Linux Software Center / Package Assistant** olacaktır.

---

## 2. Uygulama Adı

### Önerilen ana isim: Aurora

**Tam isim:** Aurora Linux Application Center

**Kısa açıklama:**

> Arch Linux için hızlı, güvenli ve modern uygulama keşif ve kurulum merkezi.

### İsim gerekçesi

- Linux masaüstü deneyimine modern ve sade bir kimlik kazandırır.
- “Aurora” kelimesi görsel olarak ışık, başlangıç ve modernlik çağrışımı yapar.
- AUR ile doğrudan aynı isimde olmadığı için markalaşma açısından daha esnek olabilir.
- KDE, GNOME ve diğer masaüstü ortamlarında kullanılabilecek nötr bir isimdir.

### Alternatif isimler

1. **Aurora**
2. **ArchFlow**
3. **Pakku**
4. **Nexora**
5. **Astra Package Center**
6. **Forge**
7. **Lumen**
8. **Orbit**
9. **Pacifica**
10. **ArchNest**

> Son isim seçilmeden önce GitHub, AUR, Flatpak, alan adı ve marka çakışması kontrol edilmelidir.

---

## 3. Temel Hedefler

### 3.1. Kullanıcı hedefleri

- Uygulamaları terminal komutu ezberlemeden bulabilmek
- Birden fazla uygulamayı tek işlemde seçebilmek
- Paketlerin resmi repo, AUR veya Flatpak kaynağını görebilmek
- Kurulumdan önce paket adı ve kaynak bilgisini inceleyebilmek
- Kurulum sürecini canlı olarak takip edebilmek
- Başarısız işlemlerde anlaşılır hata mesajı almak
- Kurulu uygulamaları görüntüleyebilmek
- Uygulamaları kaldırma veya yeniden kurma işlemlerini yönetebilmek
- Sistem kaynaklarını gereksiz yere tüketmeyen bir uygulama kullanmak

### 3.2. Teknik hedefler

- Native Linux masaüstü uygulaması olmak
- GTK4 ve libadwaita ile modern görünüm sunmak
- Rust ile bellek güvenli ve sürdürülebilir kod tabanı oluşturmak
- Pacman, AUR yardımcıları ve Flatpak ile kontrollü entegrasyon
- Paket işlemlerinde kullanıcı onayı almak
- Root yetkisini yalnızca gerekli aşamalarda kullanmak
- Paket verilerini ağdan alırken önbellekleme kullanmak
- Modüler ve test edilebilir mimari kurmak
- GNOME, KDE Plasma ve diğer GTK uyumlu ortamlarda çalışmak

---

## 4. Hedef Kullanıcı Kitlesi

### Birincil kullanıcılar

- Arch Linux kullanıcıları
- CachyOS kullanıcıları
- EndeavourOS kullanıcıları
- Manjaro kullanıcıları
- Arch tabanlı dağıtımlara yeni başlayanlar
- Terminal yerine grafik arayüz tercih eden Linux kullanıcıları

### İkincil kullanıcılar

- Linux içerik üreticileri
- Sistem yöneticileri
- Yazılım geliştiriciler
- Oyun oynayan Linux kullanıcıları
- Eğitim kurumlarında Linux kullanan öğrenciler
- Yeni kurulum sonrası uygulama seti hazırlamak isteyen kullanıcılar

---

## 5. Teknoloji Yığını

## 5.1. Ana programlama dili: Rust

Rust önerilen ana dildir.

### Rust tercih edilme nedenleri

- Bellek güvenliği
- Null pointer ve birçok bellek hatasına karşı koruma
- Linux sistem araçlarıyla iyi entegrasyon
- Performanslı native uygulamalar
- GTK4 için güçlü Rust binding'leri
- Async görevler ve süreç yönetimi için uygun ekosistem
- Uzun vadede sürdürülebilir ve güvenilir kod tabanı

### Kullanılacak Rust bileşenleri

- `gtk4`
- `libadwaita`
- `gio`
- `glib`
- `tokio` veya GLib tabanlı async yaklaşım
- `serde`
- `serde_json`
- `reqwest`
- `thiserror`
- `anyhow`
- `tracing`
- `tracing-subscriber`
- `directories`
- `which`
- `semver`
- `url`
- `tempfile`
- `async-channel` veya GLib main context kanalları

> GTK uygulamalarında UI işlemleri ana thread üzerinde yürütülmeli, uzun süren paket işlemleri arka planda çalıştırılmalıdır.

---

## 5.2. Grafik arayüz: GTK4 + libadwaita

### Neden GTK4?

- Linux masaüstüyle doğal entegrasyon
- GNOME HIG ile uyumlu tasarım
- Modern widget sistemi
- Wayland ve X11 ortamlarında kullanım
- Rust binding desteği
- Erişilebilirlik özellikleri
- HiDPI ekranlara uyum

### Neden libadwaita?

- Modern GNOME tasarım dili
- Açık/koyu tema desteği
- Adaptive layout
- Mobil ve dar pencere uyumu
- Tutarlı spacing, renk ve tipografi
- `AdwApplicationWindow`
- `AdwNavigationView`
- `AdwHeaderBar`
- `AdwPreferencesPage`
- `AdwToastOverlay`
- `AdwStatusPage`
- `AdwViewStack`

### KDE uyumluluğu

Uygulama GTK4 tabanlı olsa da KDE Plasma üzerinde çalışabilir. KDE kullanıcıları için:

- Sistem koyu/açık tema tercihine mümkün olduğunca uyum
- Portal kullanımı
- Dosya seçicilerde XDG portal desteği
- Gereksiz GNOME bağımlılıklarını sınırlama
- Wayland ve X11 testleri
- KDE üzerinde font ve pencere davranışı testleri

> libadwaita, GNOME odaklı bir tasarım sistemi sunar. KDE ile tam görsel bütünlük garanti edilmez; ancak standart GTK4 davranışı ve portal kullanımıyla iyi entegrasyon hedeflenir.

---

## 5.3. Paketleme ve dağıtım

### Birincil dağıtım yöntemi

- Kaynak kodu GitHub üzerinde
- AUR için PKGBUILD
- Native binary
- İsteğe bağlı Flatpak

### Önerilen dağıtım sırası

1. GitHub kaynak kodu
2. Arch Linux için manuel derleme
3. AUR paketi
4. CachyOS ve EndeavourOS üzerinde test
5. Flatpak manifesti
6. İleride dağıtım deposu veya release binary'leri

### Flatpak notu

Flatpak sandbox nedeniyle doğrudan sistem paket yöneticisi işlemleri sınırlı olabilir. Bu nedenle:

- Flatpak sürümünde sistem paket işlemleri varsayılan olarak devre dışı bırakılabilir.
- Native/AUR sürümünde pacman ve AUR entegrasyonu etkin olabilir.
- Kullanıcıya paketleme türüne göre özellik farkları açıkça gösterilmelidir.

---

## 6. Temel Özellikler

## 6.1. Ana ekran

Ana ekran aşağıdaki bölümlerden oluşur:

- Üst başlık çubuğu
- Arama alanı
- Öne çıkan kategoriler
- Popüler uygulamalar
- Son kullanılanlar
- Güncelleme bildirimi
- Seçim sepeti
- Sistem ve paket yöneticisi durumu

### Örnek ana ekran metinleri

- “Linux deneyimini kendi ihtiyaçlarına göre oluştur.”
- “Bugün ne yüklemek istiyorsun?”
- “Uygulamaları keşfet”
- “Kurulum kuyruğun”
- “Sistem durumun”

---

## 6.2. Uygulama kataloğu

Uygulamalar aşağıdaki kategorilerde listelenebilir:

### Temel uygulamalar

- Dosya yöneticileri
- Arşiv yöneticileri
- PDF okuyucular
- Medya oynatıcılar
- Ekran görüntüsü araçları
- Not uygulamaları
- Hesap makineleri
- Şifre yöneticileri

### İnternet

- Web tarayıcıları
- E-posta istemcileri
- Torrent istemcileri
- VPN istemcileri
- DNS araçları
- Ağ analiz araçları

### Geliştirme

- IDE'ler
- Metin editörleri
- Git araçları
- Docker araçları
- Kubernetes araçları
- Veritabanı istemcileri
- API istemcileri
- Terminal araçları
- Programlama dili araç zincirleri

### Sistem

- Disk analiz araçları
- Sistem monitörleri
- Donanım bilgi araçları
- Yedekleme araçları
- Bölümleme araçları
- Servis yöneticileri
- Enerji yönetimi araçları

### Oyun

- Steam
- Lutris
- Heroic Games Launcher
- Bottles
- Proton araçları
- MangoHud
- Gamescope
- GameMode
- Emülatörler

### Tasarım ve medya

- Görsel düzenleme
- Video düzenleme
- Ses düzenleme
- 3D modelleme
- Ekran kaydı
- Yayıncılık araçları

---

## 6.3. Paket kaynağı gösterimi

Her uygulama kartında kaynak açıkça belirtilmelidir:

- Official Repository
- AUR
- Flatpak
- AppImage
- Custom Repository

### Kaynak rozetleri

- `OFFICIAL`
- `AUR`
- `FLATPAK`
- `COMMUNITY`
- `UNVERIFIED`

Kaynağı belirsiz uygulamalar varsayılan olarak güvenilir kabul edilmemelidir.

---

## 6.4. Arama sistemi

Arama aşağıdaki alanlarda çalışmalıdır:

- Uygulama adı
- Paket adı
- Açıklama
- Kategori
- Etiket
- Kaynak türü

### Arama özellikleri

- Anlık filtreleme
- Yazım toleransı
- Kategoriye göre filtre
- Kaynağa göre filtre
- Kurulu/kurulu değil filtresi
- Sıralama
- Son aramalar
- Klavye kısayolu: `Ctrl + K`

### Sıralama seçenekleri

- İsim
- Popülerlik
- Kurulu durum
- Kaynak
- Güncelleme tarihi
- Kullanıcı tarafından seçilenler

> Popülerlik verisi güvenilir bir kaynaktan gelmiyorsa uygulama içinde kesin popülerlik iddiası kullanılmamalıdır.

---

## 6.5. Uygulama detay sayfası

Her uygulamanın detay ekranında:

- Uygulama adı
- İkon
- Açıklama
- Paket adı
- Kaynak
- Lisans bilgisi
- Kurulum boyutu
- Bağımlılık özeti
- Kurulu sürüm
- Mevcut sürüm
- Ana sayfa bağlantısı
- Depo bağlantısı
- AUR bağlantısı
- Güvenlik uyarıları
- Kur / kaldır / yeniden kur butonları

### Detay sayfası davranışları

- Paket bilgisi yüklenirken loading state
- Ağ bağlantısı yoksa önbellekten veri
- Paket bulunamazsa açıklayıcı hata
- İşlem öncesinde kullanıcı onayı
- Kaynak değişikliklerinde yeniden onay

---

## 6.6. Çoklu seçim ve kurulum sepeti

Kullanıcı birden fazla uygulama seçebilir.

### Sepet özellikleri

- Seçilen uygulama sayısı
- Tahmini indirme boyutu
- Kaynaklara göre gruplanmış liste
- Çakışma uyarıları
- Eksik bağımlılık uyarıları
- İşlem öncesi özet
- Seçimi temizleme
- Kurulum kuyruğunu düzenleme

### Kurulum öncesi özet

- Kurulacak paketler
- Kaldırılacak paketler
- Değiştirilecek paketler
- Kullanılacak komutlar
- Paket kaynakları
- Tahmini boyut
- Kullanıcı onayı

---

## 6.7. Paket yöneticisi entegrasyonu

### Pacman

Desteklenmesi gereken işlemler:

- Kurulu paketleri sorgulama
- Paket bilgisi alma
- Resmi depodan kurulum
- Kaldırma
- Yeniden kurma
- Güncelleme kontrolü
- Paket arama

### AUR yardımcıları

İlk aşamada:

- `paru`
- `yay`

İsteğe bağlı:

- `pamac`

### AUR güvenlik yaklaşımı

AUR paketleri topluluk tarafından sağlandığından:

- PKGBUILD içeriği kullanıcıya gösterilebilmeli
- Kurulumdan önce kaynak URL'leri listelenmeli
- Build script çalıştırılacağı açıkça belirtilmeli
- Kullanıcı onayı zorunlu olmalı
- Root olarak AUR build yapılmamalı
- AUR işlemleri ayrı ve sınırlı bir süreçte yürütülmeli
- Paket imzaları ve checksum bilgileri gösterilmeli
- Güvenilirlik konusunda kesin garanti verilmemeli

> Uygulama AUR paketlerini otomatik olarak güvenli ilan etmemelidir.

---

## 6.8. Terminal ve işlem günlüğü

Uygulama kullanıcıya ham terminal çıktısını gösterebilir ancak terminal çıktısı tek bilgi kaynağı olmamalıdır.

### İşlem ekranı

- Aktif işlem
- İşlem aşaması
- İndirilen veri
- İlerleme çubuğu
- Çıktı günlüğü
- Hata günlüğü
- Durdur butonu
- Tekrar dene
- Günlükleri kopyala
- Günlükleri dosyaya kaydet

### İşlem aşamaları

1. Paket bilgisi doğrulanıyor
2. Bağımlılıklar kontrol ediliyor
3. Kullanıcı onayı bekleniyor
4. Paketler indiriliyor
5. Paket imzaları kontrol ediliyor
6. Kurulum yapılıyor
7. Sistem durumu yenileniyor
8. İşlem tamamlandı

---

## 6.9. Kurulu uygulamalar

Ayrı bir “Kurulu Uygulamalar” ekranı bulunmalıdır.

### Filtreler

- Resmi repo
- AUR
- Flatpak
- Yetim paketler
- Güncellemesi bulunanlar
- Son yüklenenler

### İşlemler

- Uygulama kaldırma
- Yeniden kurma
- Paket bilgisi
- Dosya listesi
- Bağımlılıkları görüntüleme
- Ana sayfayı açma
- Paket yöneticisi üzerinden işlem

---

## 6.10. Güncelleme merkezi

İlk sürümde yalnızca bilgilendirme yapılabilir. Daha sonraki sürümde kontrollü güncelleme eklenebilir.

### Özellikler

- Güncelleme sayısı
- Güncellenecek paketler
- Tahmini indirme boyutu
- Paket kaynakları
- Güncelleme öncesi onay
- İşlem günlüğü
- Hata sonrası kurtarma önerileri

### Kritik güvenlik kuralı

Sistem güncellemesi tek tıklamayla sessizce çalıştırılmamalıdır. Kullanıcı:

- İşlemi görmeli
- İşlemi onaylamalı
- Açık paket listesini inceleyebilmeli
- Gerekirse işlemi iptal edebilmelidir

---

## 7. Modern UI/UX Tasarım Sistemi

## 7.1. Tasarım yaklaşımı

Tasarım aşağıdaki ilkeleri izlemelidir:

- Sade
- Ferah
- Hızlı algılanabilir
- Tutarlı
- Erişilebilir
- Klavye ile kullanılabilir
- Koyu ve açık tema destekli
- Dar pencere ve geniş ekran uyumlu
- Gereksiz animasyonlardan kaçınan

### Görsel yön

- Modern kart yapısı
- Yumuşak köşe yarıçapları
- Az ama anlamlı renk kullanımı
- Açık hiyerarşi
- İkon ve metin dengesi
- Duruma göre renklenen rozetler
- İşlem durumlarında net görsel geri bildirim

---

## 7.2. Renk sistemi

Renkler sabit hard-code edilmemeli, GTK/libadwaita tema değişkenleri ve CSS değişkenleri üzerinden yönetilmelidir.

### Önerilen vurgu yaklaşımı

- Ana vurgu: mor/mavi geçişli marka rengi
- Başarı: sistemin başarı rengi
- Uyarı: sistemin uyarı rengi
- Hata: sistemin hata rengi
- Bilgi: sistemin bilgi rengi

> Renk kontrastı WCAG ve GTK erişilebilirlik önerileriyle test edilmelidir.

---

## 7.3. Tipografi

- Sistem fontu varsayılan tercih olmalı
- Başlıklar için belirgin ağırlık
- Açıklamalarda rahat satır aralığı
- Monospace font yalnızca terminal ve paket çıktılarında kullanılmalı
- Metinler gereksiz büyük harflerle yazılmamalı

### Metin hiyerarşisi

- Sayfa başlığı
- Bölüm başlığı
- Uygulama adı
- Açıklama
- Meta bilgi
- Yardımcı metin
- Hata ve uyarı metni

---

## 7.4. Responsive/adaptive tasarım

Uygulama aşağıdaki pencere genişliklerinde test edilmelidir:

- 360 px eşdeğeri dar görünüm
- 600 px
- 900 px
- 1280 px
- 1920 px

### Dar görünüm

- Sidebar daraltılır veya navigation stack kullanılır
- Kartlar tek sütuna düşer
- Arama alanı genişler
- Sepet ayrı sayfa veya bottom sheet olur
- Detay ekranı tam sayfa açılır

### Geniş görünüm

- Sidebar
- Ana içerik
- Sağ tarafta seçim sepeti veya detay paneli
- İki veya üç sütunlu kart düzeni

---

## 7.5. Erişilebilirlik

- Tüm butonların tooltip veya erişilebilir etiketi
- Klavye ile gezinme
- Focus görünürlüğü
- Ekran okuyucu uyumu
- Renk dışında durum açıklaması
- Hata mesajlarının anlaşılır olması
- Yeterli kontrast
- Animasyonları azaltma tercihi

---

## 8. Uygulama Mimarisi

Önerilen mimari:

```text
aurora/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── CHANGELOG.md
├── CONTRIBUTING.md
├── SECURITY.md
├── data/
│   ├── apps/
│   │   ├── essentials.json
│   │   ├── development.json
│   │   ├── gaming.json
│   │   ├── multimedia.json
│   │   ├── internet.json
│   │   └── system.json
│   └── schemas/
├── assets/
│   ├── icons/
│   ├── screenshots/
│   └── branding/
├── src/
│   ├── main.rs
│   ├── application.rs
│   ├── config.rs
│   ├── state/
│   │   ├── mod.rs
│   │   ├── app_state.rs
│   │   └── settings_state.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── window.rs
│   │   ├── navigation.rs
│   │   ├── home_page.rs
│   │   ├── catalog_page.rs
│   │   ├── app_detail_page.rs
│   │   ├── installed_page.rs
│   │   ├── updates_page.rs
│   │   ├── queue_page.rs
│   │   ├── settings_page.rs
│   │   └── widgets/
│   ├── models/
│   │   ├── app.rs
│   │   ├── package.rs
│   │   ├── repository.rs
│   │   └── transaction.rs
│   ├── services/
│   │   ├── catalog_service.rs
│   │   ├── package_service.rs
│   │   ├── cache_service.rs
│   │   ├── update_service.rs
│   │   └── telemetry_service.rs
│   ├── package_managers/
│   │   ├── mod.rs
│   │   ├── pacman.rs
│   │   ├── paru.rs
│   │   ├── yay.rs
│   │   └── flatpak.rs
│   ├── process/
│   │   ├── command_runner.rs
│   │   ├── permissions.rs
│   │   └── output_parser.rs
│   ├── security/
│   │   ├── validation.rs
│   │   ├── aur_review.rs
│   │   └── trust.rs
│   ├── storage/
│   │   ├── settings.rs
│   │   ├── history.rs
│   │   └── cache.rs
│   └── utils/
│       ├── errors.rs
│       ├── logging.rs
│       └── platform.rs
├── tests/
│   ├── catalog_tests.rs
│   ├── package_parser_tests.rs
│   └── transaction_tests.rs
├── packaging/
│   ├── aur/
│   ├── flatpak/
│   └── desktop/
└── docs/
    ├── architecture.md
    ├── security-model.md
    └── release-process.md
```

---

## 9. Katmanlar

### UI katmanı

Sorumluluklar:

- Kullanıcı etkileşimleri
- Sayfa ve widget oluşturma
- Loading, empty, error state
- UI state güncelleme
- Kullanıcı onayları

UI katmanı doğrudan shell komutu çalıştırmamalıdır.

### Service katmanı

Sorumluluklar:

- Paket verisi getirme
- Önbellek yönetimi
- Arama ve filtreleme
- İşlem planı oluşturma
- UI ile paket yöneticisi arasında koordinasyon

### Package manager katmanı

Sorumluluklar:

- Pacman/paru/yay/Flatpak komutlarını soyutlamak
- Komut argümanlarını güvenli biçimde oluşturmak
- Çıktıları parse etmek
- Exit code kontrolü
- İşlem iptali

### Security katmanı

Sorumluluklar:

- Paket adlarını doğrulama
- Komut enjeksiyonunu engelleme
- AUR uyarıları
- Kaynak doğrulama
- Yetki sınırlarını kontrol etme

---

## 10. Güvenlik Tasarımı

Bu uygulamanın en önemli bölümlerinden biri güvenliktir. Paket yöneticisi işlemleri sistem üzerinde kalıcı değişiklik yapabildiği için tasarım “kolaylık” kadar “kontrol edilebilirlik” üzerine kurulmalıdır.

### 10.1. Shell kullanımı

Kaçınılması gereken yaklaşım:

```text
sh -c "sudo pacman -S " + user_input
```

Önerilen yaklaşım:

- Komut ve argümanları ayrı parametreler olarak çalıştırmak
- Shell interpolation kullanmamak
- Paket adlarını whitelist/regex ile doğrulamak
- Kullanıcı girdisini doğrudan shell komutu olarak çalıştırmamak

### 10.2. Yetki yönetimi

- Uygulama normal kullanıcı olarak açılmalı
- Root olarak başlatılması önerilmemeli
- Yetkili işlemler gerektiğinde polkit veya paket yöneticisinin kendi yetki mekanizması kullanılmalı
- Şifre isteyen işlem öncesinde kullanıcıya neden açıklanmalı
- Yetki bilgileri kaydedilmemeli

### 10.3. AUR işlemleri

- AUR build işlemi normal kullanıcı hesabında yapılmalı
- PKGBUILD önizlemesi sunulmalı
- Kaynak URL'leri gösterilmeli
- Kullanıcıya AUR riskleri açıklanmalı
- Otomatik ve sessiz `--noconfirm` kullanımı varsayılan olmamalı
- Şüpheli veya geçersiz paket bilgileri için uyarı gösterilmeli

### 10.4. Ağ güvenliği

- HTTPS kullanılmalı
- TLS sertifika doğrulaması kapatılmamalı
- JSON verileri şema doğrulamasından geçirilmeli
- Uzak veriler çalıştırılabilir komut olarak değerlendirilmemeli
- Uygulama kataloğu ile gerçek paket yöneticisi verisi birbirinden ayrılmalı

### 10.5. Telemetri

Varsayılan olarak telemetri kapalı olmalıdır.

Eğer ileride anonim hata raporlama eklenirse:

- Açık opt-in
- Veri açıklaması
- Kapatma seçeneği
- Kişisel veri toplamama
- IP veya kullanıcı kimliği saklamama
- Kaynak kodu ve gizlilik politikası

---

## 11. Veri Modeli

### Uygulama modeli

```json
{
  "id": "org.mozilla.firefox",
  "name": "Firefox",
  "package_name": "firefox",
  "description": "Modern ve açık kaynaklı web tarayıcısı.",
  "category": "internet",
  "repository": "official",
  "icon": "firefox.svg",
  "homepage": "https://www.mozilla.org/firefox/",
  "license": "MPL-2.0",
  "tags": ["browser", "web", "privacy"],
  "requires_confirmation": false
}
```

### Kaynak türleri

```text
official
aur
flatpak
appimage
custom
unknown
```

### İşlem modeli

```json
{
  "id": "transaction-uuid",
  "operation": "install",
  "packages": ["firefox", "vlc"],
  "source": "official",
  "status": "pending",
  "requires_privilege": true,
  "created_at": "ISO-8601"
}
```

---

## 12. Hata Yönetimi

Kullanıcıya teknik hata yerine anlaşılır açıklama gösterilmelidir.

### Hata örnekleri

#### Paket bulunamadı

Kötü:

> Exit code 1

İyi:

> Seçilen paket depolarda bulunamadı. Paket adını veya kaynak seçimini kontrol edin.

#### Yetki hatası

> İşlem için yönetici yetkisi gerekiyor. İşlemi iptal ettiyseniz yeniden deneyebilirsiniz.

#### Ağ hatası

> Paket bilgisi alınamadı. İnternet bağlantınızı veya depo durumunu kontrol edin.

#### AUR build hatası

> AUR paketi oluşturulurken hata oluştu. Ayrıntılı günlükleri inceleyebilir veya işlemi yeniden deneyebilirsiniz.

### Hata kategorileri

- NetworkError
- PermissionError
- PackageNotFound
- PackageConflict
- DependencyError
- InvalidInput
- ProcessFailed
- UserCancelled
- UnsupportedPlatform
- CacheError

---

## 13. Performans Hedefleri

### Başlangıç

- Uygulama penceresi mümkün olduğunca hızlı açılmalı
- Ağ isteği UI başlangıcını bloklamamalı
- Büyük uygulama listeleri lazy loading ile gösterilmeli

### Bellek

- Tüm paket verisi gereksiz şekilde bellekte tutulmamalı
- İkonlar önbelleklenmeli
- Liste sanallaştırması veya sayfalama değerlendirilmeli

### İşlem yönetimi

- Paket işlemleri UI thread'ini bloklamamalı
- Aynı anda çakışan paket işlemleri engellenmeli
- Kullanıcı uygulamayı kapatırsa aktif işlem durumu ele alınmalı
- İşlem iptal edilemiyorsa kullanıcıya açıkça bildirilmeli

---

## 14. Ayarlar

### Genel

- Açılışta son sayfayı aç
- Otomatik güncelleme kontrolü
- Önbelleği temizle
- Dil seçimi
- Bildirimler

### Paket yöneticisi

- Varsayılan AUR yardımcısı
- Flatpak entegrasyonu
- İşlem öncesi onay
- Günlük saklama süresi
- Paralel indirme tercihi

### Görünüm

- Sistem teması
- Açık tema
- Koyu tema
- Kompakt görünüm
- Animasyonları azalt

### Güvenlik

- AUR PKGBUILD önizlemesini zorunlu tut
- `--noconfirm` kullanımını engelle
- Riskli işlemlerde ekstra onay
- Telemetri ayarı
- Günlüklerin saklanması

---

## 15. Klavye Kısayolları

| Kısayol | İşlev |
|---|---|
| `Ctrl + K` | Aramayı aç |
| `Ctrl + F` | Uygulama arama |
| `Ctrl + Enter` | Seçili uygulamayı sepete ekle |
| `Ctrl + Shift + C` | Kurulum komutunu kopyala |
| `Ctrl + R` | Verileri yenile |
| `Escape` | Diyaloğu kapat |
| `Ctrl + ,` | Ayarlar |
| `Ctrl + Q` | Uygulamadan çık |
| `F1` | Yardım |

---

## 16. Sürüm Planı

## v0.1 — Prototip

- GTK4 pencere
- libadwaita tema
- Statik uygulama kataloğu
- Kategoriler
- Arama
- Uygulama kartları
- Çoklu seçim
- Komut önizlemesi

## v0.2 — Paket Bilgisi

- Pacman tespiti
- Kurulu paket kontrolü
- Resmi repo araması
- Paket detay sayfası
- İşlem öncesi özet

## v0.3 — Kurulum İşlemleri

- Pacman kurulumu
- İşlem günlüğü
- Hata yönetimi
- İptal ve tekrar deneme
- Yetki akışı

## v0.4 — AUR

- paru tespiti
- yay tespiti
- AUR araması
- PKGBUILD önizlemesi
- AUR güvenlik uyarıları

## v0.5 — Kurulu Uygulamalar

- Kurulu paket listesi
- Kaldırma
- Yeniden kurma
- Paket geçmişi
- Güncelleme kontrolü

## v0.6 — Flatpak

- Flatpak tespiti
- Flatpak uygulama listesi
- Flatpak kurulumu
- Kaynak rozetleri

## v0.7 — Modernleştirme

- Adaptive layout
- Gelişmiş animasyonlar
- Bildirimler
- Daha iyi arama
- Favoriler
- Son kullanılanlar

## v1.0 — İlk kararlı sürüm

- Güvenlik denetimi
- Test kapsamı
- AUR paketi
- Release binary'leri
- Kullanıcı dokümantasyonu
- Hata raporlama süreci
- Sürüm notları

---

## 17. Test Stratejisi

### Birim testleri

- Paket adı doğrulama
- JSON veri parse etme
- Filtreleme
- Sıralama
- Komut argümanı oluşturma
- İşlem durumu geçişleri
- Hata dönüşümleri

### Entegrasyon testleri

- Pacman çıktısı parse etme
- AUR yardımcısı tespiti
- Flatpak tespiti
- Paket bulunamadı durumu
- Yetki reddi
- Ağ bağlantısı yokken çalışma

### UI testleri

- Dar pencere
- Koyu tema
- Klavye navigasyonu
- Ekran okuyucu etiketleri
- Loading state
- Empty state
- Error state

### Dağıtım testleri

- Arch Linux
- CachyOS
- EndeavourOS
- Manjaro
- GNOME Wayland
- KDE Plasma Wayland
- X11

> Testler gerçek sistemde paket kurulumunu otomatik olarak gerçekleştirmeden önce izole test ortamlarında yürütülmelidir.

---

## 18. Git ve GitHub İş Akışı

### Branch yapısı

```text
main
develop
feature/ui-home
feature/pacman-integration
feature/aur-support
fix/package-parser
release/v1.0.0
```

### Commit formatı

```text
feat: add application catalog
fix: handle missing package metadata
refactor: separate package manager service
docs: update installation guide
test: add package validation tests
```

### Pull request kuralları

- Açıklama zorunlu
- Değişen özelliklerin ekran görüntüsü
- Test sonucu
- Güvenlik etkisi
- Geriye dönük uyumluluk notu
- Büyük değişikliklerde tasarım önerisi

---

## 19. Lisans ve Topluluk

### Lisans önerisi

`GPL-3.0-or-later`

Gerekçeler:

- Linux ve özgür yazılım ekosistemiyle uyum
- Türetilmiş sürümlerde özgürlüklerin korunması
- Topluluk katkısını teşvik etme

Alternatifler:

- MIT
- Apache-2.0
- GPL-3.0-or-later

Lisans seçimi, proje hedeflerine ve dağıtım stratejisine göre kesinleştirilmelidir.

### Topluluk dosyaları

- `CODE_OF_CONDUCT.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `SUPPORT.md`
- Issue template
- Feature request template
- Bug report template

---

## 20. Kullanıcı Akışları

### Uygulama kurma akışı

```text
Ana ekran
   ↓
Arama veya kategori
   ↓
Uygulama detay sayfası
   ↓
Kaynak ve paket bilgisi
   ↓
Sepete ekle
   ↓
Kurulum özetini görüntüle
   ↓
Kullanıcı onayı
   ↓
Paket yöneticisi işlemi
   ↓
Canlı işlem günlüğü
   ↓
Başarı veya hata ekranı
   ↓
Kurulu durumunu yenile
```

### AUR akışı

```text
AUR uygulaması seçilir
   ↓
AUR uyarısı gösterilir
   ↓
PKGBUILD ve kaynaklar görüntülenir
   ↓
Kullanıcı onayı
   ↓
Normal kullanıcıyla build
   ↓
Paket doğrulama
   ↓
Kurulum için yetki talebi
   ↓
Sonuç ve günlük
```

---

## 21. Örnek Rust Tasarım Yaklaşımı

Paket yöneticileri ortak bir trait üzerinden soyutlanabilir:

```rust
pub trait PackageManager {
    fn is_available(&self) -> bool;
    fn search(&self, query: &str) -> Result<Vec<Package>, PackageError>;
    fn get_info(&self, package: &str) -> Result<PackageInfo, PackageError>;
    fn install(&self, packages: &[String]) -> Result<TransactionId, PackageError>;
    fn remove(&self, packages: &[String]) -> Result<TransactionId, PackageError>;
}
```

Gerçek uygulamada uzun süren işlemler için async veya arka plan thread yaklaşımı kullanılmalıdır. Trait tasarımı, kullanılan async mimariye göre yeniden düzenlenebilir.

---

## 22. Tasarımda Kaçınılması Gerekenler

- WebView tabanlı arayüzü native masaüstü uygulaması gibi sunmak
- Her işlemde terminal penceresi açmak
- Kullanıcı onayı olmadan sistem değişikliği yapmak
- Varsayılan olarak `--noconfirm` kullanmak
- AUR paketlerini güvenli olarak etiketlemek
- Root ile uygulamayı çalıştırmayı önermek
- Uzun süren işlemleri UI thread'inde yapmak
- Her şeyi tek bir `main.rs` dosyasına koymak
- Sabit ve kontrolsüz uzak JSON verisine güvenmek
- Sadece koyu tema tasarlamak
- Erişilebilirlik etiketlerini atlamak
- Hataları yalnızca teknik exit code olarak göstermek
- Paket kaynağını gizlemek
- Kullanıcıya ne kurulacağını göstermeden işlem başlatmak

---

## 23. İlk Geliştirme Sprinti

### Sprint hedefi

Çalışan ve görsel olarak modern bir GTK4/libadwaita prototipi oluşturmak.

### Görevler

- [ ] Rust projesini oluştur
- [ ] GTK4 ve libadwaita bağımlılıklarını ekle
- [ ] Uygulama ID'sini belirle
- [ ] Ana pencereyi oluştur
- [ ] Header bar ekle
- [ ] Sidebar/navigation oluştur
- [ ] Ana ekran tasarımını oluştur
- [ ] Kategori modeli ekle
- [ ] Uygulama kartı widget'ı oluştur
- [ ] Statik JSON kataloğu ekle
- [ ] Arama alanı ekle
- [ ] Sepet state'i ekle
- [ ] Açık/koyu tema testi yap
- [ ] README oluştur
- [ ] İlk ekran görüntülerini ekle

### Sprint sonunda beklenen çıktı

- Uygulama açılmalı
- Kategoriler görüntülenmeli
- Uygulama araması çalışmalı
- Uygulamalar sepete eklenebilmeli
- Kurulum komutu henüz çalıştırılmadan önizlenebilmeli
- UI donmadan kullanılabilmeli

---

## 24. Başarı Kriterleri

Proje aşağıdaki koşullar sağlandığında ilk kararlı sürüme yaklaşmış kabul edilir:

- Native GTK4/libadwaita uygulaması olarak çalışması
- Arch tabanlı en az üç dağıtımda test edilmesi
- Pacman işlemlerinin kontrollü çalışması
- AUR işlemlerinde kullanıcıya açık uyarılar verilmesi
- Paket kaynaklarının görünür olması
- Kullanıcı onayı olmadan kalıcı sistem değişikliği yapılmaması
- Hataların anlaşılır biçimde gösterilmesi
- Koyu/açık tema desteği
- Klavye navigasyonu
- Paket işlemlerinde UI donmasının engellenmesi
- Kaynak kodunun modüler olması
- AUR veya native paketleme yönergelerinin hazırlanması

---

## 25. Sonuç

Aurora, NinitArch'ın “birden fazla Arch Linux uygulamasını seçip kurulum komutu oluşturma” yaklaşımını temel alarak daha kapsamlı bir Linux masaüstü uygulamasına dönüştürülmelidir.

Önerilen temel teknoloji:

- **Rust:** Güvenli ve performanslı backend
- **GTK4:** Native Linux arayüzü
- **libadwaita:** Modern ve adaptive tasarım
- **Pacman:** Resmi paket yönetimi
- **paru/yay:** AUR entegrasyonu
- **Flatpak:** İsteğe bağlı alternatif uygulama kaynağı
- **Serde/JSON:** Katalog verileri
- **Tokio veya GLib async:** Arka plan işlemleri
- **GitHub Actions:** CI/CD ve test

İlk hedef, bütün özellikleri aynı anda geliştirmek yerine modern ve güvenli bir prototip oluşturmaktır. Önce katalog, arama, uygulama detayları ve kurulum önizlemesi hazırlanmalı; ardından gerçek paket işlemleri güvenlik kontrolleriyle aşamalı olarak eklenmelidir.

**Kısa ürün tanımı:**

> Aurora, Arch Linux kullanıcılarına uygulamaları keşfetme, inceleme ve güvenli biçimde yönetme imkânı sunan modern, açık kaynaklı ve native GTK4 masaüstü uygulamasıdır.
