# Aurora Katkıda Bulunma Rehberi (Contributing Guide)

Aurora projesine katkıda bulunmak istediğiniz için teşekkür ederiz! Aurora, Arch Linux ve tabanlı dağıtımlar için modern, güvenilir, hızlı ve güvenli bir grafiksel paket ve uygulama yönetim merkezi geliştirmeyi amaçlayan açık kaynaklı bir topluluk projesidir.

---

## ⚠️ Proje Durumu: Aktif Geliştirme Aşamasında (Alpha)

Aurora şu anda **aktif geliştirme aşamasındadır**. Mimari, kullanıcı arayüzü ve paket yönetim entegrasyonları sürekli genişletilmekte ve iyileştirilmektedir. Bu aşamada topluluktan gelen her türlü geri bildirim, hata bildirimi ve kod katkısı projenin olgunlaşması için kritik öneme sahiptir.

---

## 🎯 Nasıl Katkıda Bulunabilirsiniz?

Katkıda bulunmanın pek çok farklı yolu vardır. Yalnızca kod yazarak değil; hata bildirerek, fikir önererek, çeviri yaparak veya test ederek de projeyi büyütebilirsiniz:

### 1. Hata Bildirimi (Bug Reports)
Bir çökme, görsel bozulma, paket yönetim hatası veya beklenmeyen bir davranışla karşılaşırsanız lütfen [GitHub Issues](https://github.com/topalemirfaruk/Aurora/issues) üzerinden bildirin.

**Etkili bir hata bildiriminde bulunması gerekenler:**
- Kullandığınız işletim sistemi / dağıtım (Arch Linux, EndeavourOS, Manjaro, CachyOS vb.)
- Masaüstü ortamınız (GNOME, KDE Plasma, Hyprland, XFCE vb.) ve görüntü sunucusu (Wayland / X11)
- Hatayı yeniden üretme adımları
- Beklenen davranış ve gerçekleşen davranış
- Terminalden çalıştırıldığında alınan log çıktıları (`RUST_LOG=debug aurora`)

### 2. Yeni Özellik ve Paket Önerileri (Feature Requests)
- "Şu özellik olsa harika olurdu" dediğiniz fikirler
- Uygulama kataloğuna (`src/models/catalog_seed.rs`) eklenmesini istediğiniz popüler veya vazgeçilmez Linux yazılımları
- AUR, Flatpak veya sistem bakımına dair yeni iyileştirme önerileri

Yeni önerilerinizi [GitHub Issues](https://github.com/topalemirfaruk/Aurora/issues) üzerinden paylaşabilirsiniz.

### 3. Farklı Dağıtım ve Ortamlarda Test
Aurora saf Arch Linux'un yanı sıra Arch tabanlı farklı dağıtımları da desteklemeyi hedefler:
- EndeavourOS
- Manjaro
- CachyOS
- Garuda Linux
- Artix / Void (Pacman uyumluluğu)

Bu sistemlerde Aurora'yı derleyip test ederek sistem yöneticisi (yay / paru), paket kaynakları ve sistem bakımı araçlarının uyumluluğunu doğrulayabilir ve sonuçları bildirebilirsiniz.

### 4. Çeviri ve Yerelleştirme (Localization)
Aurora'nın arayüz metinlerinin farklı dillere çevrilmesi veya mevcut Türkçe açıklamaların daha akıcı hale getirilmesi konusunda katkı sunabilirsiniz.

### 5. Kod Katkısı (Pull Requests)
Yeni bir özellik geliştirmek veya bir hatayı çözmek için doğrudan kod katkısında bulunabilirsiniz.

---

## 🛠️ Geliştirici Ortamı Kurulumu

### Gereksinimler (Arch Linux ve Türevleri):
```bash
sudo pacman -S --needed base-devel git rust cargo gtk4 libadwaita
```

İsteğe bağlı (tam entegrasyon testleri için):
```bash
sudo pacman -S --needed flatpak pacman-contrib paru
```

### Projeyi Klonlama ve Derleme:
```bash
git clone https://github.com/topalemirfaruk/Aurora.git
cd Aurora

# Bağımlılıkları kontrol et ve hata ayıklama sürümünü derle
cargo build

# Uygulamayı çalıştır
cargo run
```

---

## 📋 Kodlama Standartları ve Kalite Güvencesi

Aurora kod tabanında yüksek mühendislik kalitesi, temiz kod ilkeleri ve katı Rust deyimleri esastır. Göndereceğiniz çekme isteklerinin (PR) şu kriterleri karşılaması beklenir:

1. **Sıfır Derleyici ve Linter Uyarısı:**
   ```bash
   cargo clippy -- -D warnings
   ```
   Tüm kodlar clippy kurallarına tam uyumlu olmalıdır.

2. **Birim ve Entegrasyon Testleri:**
   ```bash
   cargo test
   ```
   Eklenen yeni özellikler veya iş mantığı fonksiyonları için test yazılmalı, mevcut tüm testler eksiksiz geçmelidir.

3. **Kod Formatı:**
   ```bash
   cargo fmt --check
   ```
   Kod stili standart `rustfmt` kurallarına uygun olmalıdır (`cargo fmt` ile otomatik biçimlendirebilirsiniz).

4. **Kullanıcı Arayüzü İlkeleri:**
   - GNOME Human Interface Guidelines (HIG) standartlarına ve Libadwaita bileşenlerine sadık kalınmalıdır.
   - Sabit renk kodları veya emojiler yerine FreeDesktop standart sembolik ikonları (`IconResolver`) ve semantik CSS sınıfları kullanılmalıdır.

---

## 🌿 Git İş Akışı ve Commit Kuralları

1. Depoyu forklayın ve güncel `main` dalından yeni bir özellik dalı açın:
   ```bash
   git checkout -b feature/katalog-filtreleme
   # veya
   git checkout -b fix/kisayol-odak-problemi
   ```

2. **Anlamlı Commit Mesajları (Conventional Commits):**
   - `feat: flatpak arama desteği eklendi`
   - `fix: klavye kısayollarının bubble fazında yakalanamaması düzeltildi`
   - `docs: katkıda bulunma kılavuzu ve kısayol tablosu güncellendi`
   - `refactor: tema yöneticisi basitleştirildi`
   - `test: sepet dışa aktarım testleri genişletildi`

3. Değişikliklerinizi test ettikten sonra deponuza push edin ve `main` dalına bir **Pull Request** açın.

---

## 💬 İletişim ve Destek

- **GitHub Issues:** [https://github.com/topalemirfaruk/Aurora/issues](https://github.com/topalemirfaruk/Aurora/issues)
- **Geliştirici:** Emir Faruk Topal ([GitHub @topalemirfaruk](https://github.com/topalemirfaruk))

Aurora'ya katkıda bulunan ve Linux masaüstü ekosistemini geliştiren herkese içtenlikle teşekkür ederiz!
