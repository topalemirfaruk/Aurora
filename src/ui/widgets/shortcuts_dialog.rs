use gtk4::prelude::*;
use gtk4::{ShortcutsGroup, ShortcutsSection, ShortcutsShortcut, ShortcutsWindow};

pub struct ShortcutsHelpDialog;

impl ShortcutsHelpDialog {
    pub fn show(parent: &impl IsA<gtk4::Window>) {
        let win = ShortcutsWindow::builder()
            .transient_for(parent)
            .modal(true)
            .build();

        let section = ShortcutsSection::builder().build();

        // 1. Gezinme Grubu
        let nav_group = ShortcutsGroup::builder()
            .title("Gezinme ve Sayfalar")
            .build();

        let sc_home = ShortcutsShortcut::builder()
            .title("Keşfet (Ana Sayfa)")
            .accelerator("<Primary>1")
            .build();
        let sc_cat = ShortcutsShortcut::builder()
            .title("Paket Kataloğu")
            .accelerator("<Primary>2")
            .build();
        let sc_cart = ShortcutsShortcut::builder()
            .title("Kurulum Sepeti")
            .accelerator("<Primary>3")
            .build();
        let sc_inst = ShortcutsShortcut::builder()
            .title("Yüklü Paketler")
            .accelerator("<Primary>4")
            .build();
        let sc_maint = ShortcutsShortcut::builder()
            .title("Sistem Bakımı")
            .accelerator("<Primary>5")
            .build();
        let sc_settings = ShortcutsShortcut::builder()
            .title("Ayarlar")
            .accelerator("<Primary>comma")
            .build();

        nav_group.append(&sc_home);
        nav_group.append(&sc_cat);
        nav_group.append(&sc_cart);
        nav_group.append(&sc_inst);
        nav_group.append(&sc_maint);
        nav_group.append(&sc_settings);

        // 2. Arama ve Eylemler Grubu
        let action_group = ShortcutsGroup::builder()
            .title("Arama ve Eylemler")
            .build();

        let sc_search = ShortcutsShortcut::builder()
            .title("Arama Kutusuna Odaklan")
            .accelerator("<Primary>f")
            .build();
        let sc_esc = ShortcutsShortcut::builder()
            .title("Aramayı Temizle / Odaktan Çık")
            .accelerator("Escape")
            .build();
        let sc_refresh = ShortcutsShortcut::builder()
            .title("Sayfayı / Listeleri Yenile")
            .accelerator("F5")
            .build();
        let sc_shortcuts = ShortcutsShortcut::builder()
            .title("Klavye Kısayolları Penceresi")
            .accelerator("<Primary>question")
            .build();
        let sc_about = ShortcutsShortcut::builder()
            .title("Aurora Hakkında")
            .accelerator("<Primary>slash")
            .build();
        let sc_quit = ShortcutsShortcut::builder()
            .title("Aurora'dan Çık")
            .accelerator("<Primary>q")
            .build();

        action_group.append(&sc_search);
        action_group.append(&sc_esc);
        action_group.append(&sc_refresh);
        action_group.append(&sc_shortcuts);
        action_group.append(&sc_about);
        action_group.append(&sc_quit);

        section.append(&nav_group);
        section.append(&action_group);
        win.set_child(Some(&section));

        win.present();
    }
}
