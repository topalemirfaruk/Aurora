#!/usr/bin/env bash
set -e

echo "Aurora — Derleme ve Kurulum Başlatılıyor..."

# 1. Release derlemesi
cargo build --release

# 2. Kurulum dizinleri
PREFIX="/usr/local"
BIN_DIR="$PREFIX/bin"
DESKTOP_DIR="$PREFIX/share/applications"
ICON_DIR="$PREFIX/share/icons/hicolor/scalable/apps"

echo "Dosyalar $PREFIX altına kuruluyor (sudo yetkisi gerekebilir)..."

sudo install -Dm755 "target/release/aurora" "$BIN_DIR/aurora"
sudo install -Dm644 "packaging/desktop/org.aurora.ApplicationCenter.desktop" "$DESKTOP_DIR/org.aurora.ApplicationCenter.desktop"
sudo install -Dm644 "assets/icons/org.aurora.ApplicationCenter.svg" "$ICON_DIR/org.aurora.ApplicationCenter.svg"
sudo install -Dm644 "assets/icons/aurora-empty-cart.svg" "$ICON_DIR/aurora-empty-cart.svg"

# Kullanıcı yerel dizinine de kopyala (KDE / GNOME oturumu için anında erişim)
mkdir -p "$HOME/.local/share/icons/hicolor/scalable/apps" "$HOME/.local/share/applications"
cp "assets/icons/org.aurora.ApplicationCenter.svg" "$HOME/.local/share/icons/hicolor/scalable/apps/"
cp "assets/icons/aurora-empty-cart.svg" "$HOME/.local/share/icons/hicolor/scalable/apps/"
cp "packaging/desktop/org.aurora.ApplicationCenter.desktop" "$HOME/.local/share/applications/"

# 3. İkon ve masaüstü veritabanını güncelle
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    sudo gtk-update-icon-cache -f -t "$PREFIX/share/icons/hicolor" 2>/dev/null || true
fi

if command -v update-desktop-database >/dev/null 2>&1; then
    sudo update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
fi

echo "Aurora başarıyla kuruldu. Uygulama menünüzden 'Aurora' olarak başlatabilir veya terminalden 'aurora' komutuyla çalıştırabilirsiniz."
