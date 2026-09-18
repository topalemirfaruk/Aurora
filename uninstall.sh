#!/usr/bin/env bash
set -e

# Renkler
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Aurora — Sistemden Kaldırma (Uninstall) Betiği${NC}"
echo "----------------------------------------------------"

# Onay iste (eğer -y veya --force verilmemişse)
if [[ "$1" != "-y" && "$1" != "--yes" && "$1" != "--force" ]]; then
    read -p "Aurora sisteminizden tamamen kaldırılacak. Devam etmek istiyor musunuz? [e/H]: " confirm
    if [[ "$confirm" != "e" && "$confirm" != "E" && "$confirm" != "y" && "$confirm" != "Y" ]]; then
        echo -e "${YELLOW}Kaldırma işlemi iptal edildi.${NC}"
        exit 0
    fi
fi

PREFIX="/usr/local"
BIN_FILE="$PREFIX/bin/aurora"
DESKTOP_FILE="$PREFIX/share/applications/org.aurora.ApplicationCenter.desktop"
ICON_APP="$PREFIX/share/icons/hicolor/scalable/apps/org.aurora.ApplicationCenter.svg"
ICON_CART="$PREFIX/share/icons/hicolor/scalable/apps/aurora-empty-cart.svg"
LICENSE_DIR="$PREFIX/share/licenses/aurora"

LOCAL_DESKTOP="$HOME/.local/share/applications/org.aurora.ApplicationCenter.desktop"
LOCAL_ICON_APP="$HOME/.local/share/icons/hicolor/scalable/apps/org.aurora.ApplicationCenter.svg"
LOCAL_ICON_CART="$HOME/.local/share/icons/hicolor/scalable/apps/aurora-empty-cart.svg"
CONFIG_DIR="$HOME/.config/aurora"

echo -e "\n${YELLOW}Sistem düzeyindeki dosyalar siliniyor (sudo yetkisi gerekebilir)...${NC}"

# Sistem dosyaları
if [ -f "$BIN_FILE" ]; then
    sudo rm -f "$BIN_FILE"
    echo -e "  ${GREEN}[OK]${NC} $BIN_FILE silindi."
fi

if [ -f "$DESKTOP_FILE" ]; then
    sudo rm -f "$DESKTOP_FILE"
    echo -e "  ${GREEN}[OK]${NC} $DESKTOP_FILE silindi."
fi

if [ -f "$ICON_APP" ]; then
    sudo rm -f "$ICON_APP"
    echo -e "  ${GREEN}[OK]${NC} $ICON_APP silindi."
fi

if [ -f "$ICON_CART" ]; then
    sudo rm -f "$ICON_CART"
    echo -e "  ${GREEN}[OK]${NC} $ICON_CART silindi."
fi

if [ -d "$LICENSE_DIR" ]; then
    sudo rm -rf "$LICENSE_DIR"
    echo -e "  ${GREEN}[OK]${NC} $LICENSE_DIR silindi."
fi

echo -e "\n${YELLOW}Kullanıcı yerel dizinindeki dosyalar siliniyor...${NC}"

# Kullanıcı yerel dosyaları
if [ -f "$LOCAL_DESKTOP" ]; then
    rm -f "$LOCAL_DESKTOP"
    echo -e "  ${GREEN}[OK]${NC} $LOCAL_DESKTOP silindi."
fi

if [ -f "$LOCAL_ICON_APP" ]; then
    rm -f "$LOCAL_ICON_APP"
    echo -e "  ${GREEN}[OK]${NC} $LOCAL_ICON_APP silindi."
fi

if [ -f "$LOCAL_ICON_CART" ]; then
    rm -f "$LOCAL_ICON_CART"
    echo -e "  ${GREEN}[OK]${NC} $LOCAL_ICON_CART silindi."
fi

# Yapılandırma dosyaları temizlensin mi?
if [ -d "$CONFIG_DIR" ]; then
    if [[ "$1" == "-y" || "$1" == "--yes" || "$1" == "--force" ]]; then
        rm -rf "$CONFIG_DIR"
        echo -e "  ${GREEN}[OK]${NC} $CONFIG_DIR (yapılandırma dosyaları) silindi."
    else
        read -p "Kullanıcı ayarları ve önbellek ($CONFIG_DIR) silinsin mi? [e/H]: " rm_conf
        if [[ "$rm_conf" == "e" || "$rm_conf" == "E" || "$rm_conf" == "y" || "$rm_conf" == "Y" ]]; then
            rm -rf "$CONFIG_DIR"
            echo -e "  ${GREEN}[OK]${NC} $CONFIG_DIR silindi."
        else
            echo -e "  ${BLUE}[INFO]${NC} $CONFIG_DIR korundu."
        fi
    fi
fi

# Masaüstü ve ikon önbelleklerini güncelle
echo -e "\n${YELLOW}Masaüstü ve ikon veritabanları güncelleniyor...${NC}"

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    sudo gtk-update-icon-cache -f -t "$PREFIX/share/icons/hicolor" 2>/dev/null || true
    gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" 2>/dev/null || true
fi

if command -v update-desktop-database >/dev/null 2>&1; then
    sudo update-desktop-database "$PREFIX/share/applications" 2>/dev/null || true
    update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
fi

echo -e "\n${GREEN}Aurora sisteminizden başarıyla kaldırıldı.${NC}"
