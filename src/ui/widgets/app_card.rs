use crate::models::AppItem;
use crate::state::CartState;
use crate::utils::IconResolver;
use gtk4::prelude::*;
use gtk4::{Box, Button, Image, Label, Orientation, Align};

pub struct AppCard;

impl AppCard {
    pub fn new<F>(item: AppItem, cart_state: CartState, on_detail_clicked: F) -> Box
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

        // Başlık satırı: İsim + Kaynak Rozeti
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

        title_row.append(&name_label);
        title_row.append(&badge);

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

        // Sepete Ekle / Çıkar Butonu
        let cart_btn = Button::builder()
            .css_classes(["suggested-action"])
            .build();

        let update_btn_state = {
            let cart_state = cart_state.clone();
            let pkg_name = item.package_name.clone();
            let cart_btn = cart_btn.clone();
            move || {
                if cart_state.contains(&pkg_name) {
                    cart_btn.set_label("Eklendi ✓");
                    cart_btn.remove_css_class("suggested-action");
                    cart_btn.add_css_class("destructive-action");
                    cart_btn.set_tooltip_text(Some("Sepetten çıkar"));
                } else {
                    cart_btn.set_label("+ Ekle");
                    cart_btn.remove_css_class("destructive-action");
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

        // Sepet değişimlerini dinle
        let update_for_listen = update_btn_state;
        cart_state.on_change(move |_| {
            update_for_listen();
        });

        actions_box.append(&detail_btn);
        actions_box.append(&cart_btn);
        root.append(&actions_box);

        root
    }
}
