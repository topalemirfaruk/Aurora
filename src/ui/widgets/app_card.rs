use crate::models::AppItem;
use crate::state::{CartState, InstalledState};
use crate::utils::IconResolver;
use gtk4::prelude::*;
use gtk4::{Box, Button, Image, Label, Orientation, Align};

pub struct AppCard;

impl AppCard {
    pub fn new<F>(
        item: AppItem,
        cart_state: CartState,
        installed_state: InstalledState,
        on_detail_clicked: F,
    ) -> Box
    where
        F: Fn(AppItem) + 'static,
    {
        let root = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(16)
            .css_classes(["aurora-card"])
            .build();

        // Akıllı İkon Çözümleme (Kırık simgeleri engeller)
        let resolved_icon = IconResolver::resolve(&item.icon, item.category);

        let icon_frame = Box::builder()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .css_classes(["aurora-icon-frame"])
            .build();

        let icon_image = Image::from_icon_name(&resolved_icon);
        icon_image.set_pixel_size(44);
        icon_frame.append(&icon_image);
        root.append(&icon_frame);

        // Orta içerik alanı
        let content_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .hexpand(true)
            .build();

        // Başlık satırı: İsim + Kaynak Rozeti + Kurulu Rozeti
        let title_row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        let name_label = Label::builder()
            .label(&item.name)
            .css_classes(["heading"])
            .halign(Align::Start)
            .build();

        let badge = Label::builder()
            .label(item.source.badge_label())
            .css_classes([item.source.css_class()])
            .halign(Align::Start)
            .build();

        let installed_badge = Label::builder()
            .label("✓ Kurulu")
            .css_classes(["badge-installed"])
            .halign(Align::Start)
            .visible(false)
            .build();

        title_row.append(&name_label);
        title_row.append(&badge);
        title_row.append(&installed_badge);

        // Açıklama
        let desc_label = Label::builder()
            .label(&item.description)
            .wrap(true)
            .lines(2)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .halign(Align::Start)
            .css_classes(["dim-label"])
            .build();

        // Paket adı ve kategori etiketi
        let meta_row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();

        let pkg_label = Label::builder()
            .label(&format!("📦 {}", item.package_name))
            .css_classes(["caption", "dim-label"])
            .halign(Align::Start)
            .build();

        let cat_label = Label::builder()
            .label(&format!("🏷️ {}", item.category.title()))
            .css_classes(["caption", "dim-label"])
            .halign(Align::Start)
            .build();

        meta_row.append(&pkg_label);
        meta_row.append(&cat_label);

        content_box.append(&title_row);
        content_box.append(&desc_label);
        content_box.append(&meta_row);
        root.append(&content_box);

        // Sağ eylem butonları
        let actions_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .valign(Align::Center)
            .build();

        // İncele / Detay Butonu
        let detail_btn = Button::builder()
            .icon_name("document-properties-symbolic")
            .tooltip_text("Uygulama ayrıntılarını görüntüle")
            .css_classes(["flat", "circular"])
            .build();

        let item_clone_for_detail = item.clone();
        detail_btn.connect_clicked(move |_| {
            on_detail_clicked(item_clone_for_detail.clone());
        });

        // Doğrudan Sistemden Kaldır Butonu (Sadece kuruluyken görünür)
        let uninstall_btn = Button::builder()
            .label("Kaldır")
            .icon_name("user-trash-symbolic")
            .css_classes(["flat", "destructive-action"])
            .tooltip_text("Bu uygulamayı sistemden kaldır")
            .visible(false)
            .build();

        let pkg_name_for_uninstall = item.package_name.clone();
        let inst_for_uninstall = installed_state.clone();
        uninstall_btn.connect_clicked(move |btn| {
            if let Some(root_win) = btn.root().and_downcast::<gtk4::Window>() {
                let inst = inst_for_uninstall.clone();
                crate::ui::widgets::UninstallDialog::show(&root_win, &pkg_name_for_uninstall, move || {
                    inst.refresh_background();
                });
            }
        });

        // Sepete Ekle / Çıkar / Kurulu Butonu
        let cart_btn = Button::builder()
            .css_classes(["suggested-action"])
            .build();

        let update_btn_state = {
            let cart_state = cart_state.clone();
            let installed_state = installed_state.clone();
            let item_clone = item.clone();
            let cart_btn = cart_btn.clone();
            let uninstall_btn = uninstall_btn.clone();
            let installed_badge = installed_badge.clone();
            move || {
                let is_installed = installed_state.is_installed_item(&item_clone);
                installed_badge.set_visible(is_installed);
                uninstall_btn.set_visible(is_installed);

                if cart_state.contains(&item_clone.package_name) {
                    cart_btn.set_label("Eklendi ✓");
                    cart_btn.remove_css_class("suggested-action");
                    cart_btn.remove_css_class("flat");
                    cart_btn.add_css_class("destructive-action");
                    cart_btn.set_tooltip_text(Some("Sepetten çıkar"));
                } else if is_installed {
                    cart_btn.set_label("Yeniden Kur");
                    cart_btn.remove_css_class("destructive-action");
                    cart_btn.remove_css_class("suggested-action");
                    cart_btn.add_css_class("flat");
                    cart_btn.set_tooltip_text(Some("Yeniden kurmak veya derlemek için sepete ekleyin"));
                } else {
                    cart_btn.set_label("+ Ekle");
                    cart_btn.remove_css_class("destructive-action");
                    cart_btn.remove_css_class("flat");
                    cart_btn.add_css_class("suggested-action");
                    cart_btn.set_tooltip_text(Some("Kurulum sepetine ekle"));
                }
            }
        };

        update_btn_state();

        let item_for_toggle = item.clone();
        let cart_for_toggle = cart_state.clone();
        let update_for_click = update_btn_state.clone();
        cart_btn.connect_clicked(move |_| {
            cart_for_toggle.toggle(item_for_toggle.clone());
            update_for_click();
        });

        // Sepet ve sistem kurulu paket değişimlerini dinle
        let update_for_cart = update_btn_state.clone();
        cart_state.on_change(move |_| {
            update_for_cart();
        });

        let update_for_installed = update_btn_state;
        installed_state.on_change(move || {
            update_for_installed();
        });

        actions_box.append(&detail_btn);
        actions_box.append(&uninstall_btn);
        actions_box.append(&cart_btn);
        root.append(&actions_box);

        root
    }
}
