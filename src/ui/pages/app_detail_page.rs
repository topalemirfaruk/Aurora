use crate::models::{AppItem, PackageSource};
use crate::state::CartState;
use crate::ui::widgets::PkgbuildDialog;
use crate::utils::IconResolver;
use gtk4::prelude::*;
use gtk4::{Box, Button, Image, Label, Orientation, Align};
use libadwaita as adw;
use libadwaita::prelude::*;

pub struct AppDetailPage;

impl AppDetailPage {
    pub fn show(parent: &impl IsA<gtk4::Window>, item: AppItem, cart_state: CartState) {
        let dialog = adw::Window::builder()
            .transient_for(parent)
            .modal(true)
            .title(&item.name)
            .default_width(540)
            .default_height(500)
            .build();

        let root_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();

        let header_bar = adw::HeaderBar::builder()
            .show_end_title_buttons(true)
            .build();
        root_box.append(&header_bar);

        let content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(16)
            .margin_start(24)
            .margin_end(24)
            .margin_top(16)
            .margin_bottom(24)
            .build();

        // Üst Başlık Bölümü: İkon + İsim + Rozet
        let top_row = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(16)
            .build();

        let resolved_icon = IconResolver::resolve(&item.icon, item.category);
        let icon_frame = Box::builder()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .css_classes(["aurora-icon-frame"])
            .build();

        let icon = Image::from_icon_name(&resolved_icon);
        icon.set_pixel_size(56);
        icon_frame.append(&icon);
        top_row.append(&icon_frame);

        let title_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .hexpand(true)
            .build();

        let name_label = Label::builder()
            .label(&item.name)
            .halign(Align::Start)
            .css_classes(["title-2"])
            .build();

        let badge = Label::builder()
            .label(item.source.badge_label())
            .halign(Align::Start)
            .css_classes([item.source.css_class()])
            .build();

        title_box.append(&name_label);
        title_box.append(&badge);
        top_row.append(&title_box);
        content.append(&top_row);

        // Açıklama
        let desc_label = Label::builder()
            .label(&item.description)
            .wrap(true)
            .halign(Align::Start)
            .build();
        content.append(&desc_label);

        // Bilgi Kartı
        let info_group = adw::PreferencesGroup::builder()
            .title("Paket Ayrıntıları")
            .build();

        let pkg_row = adw::ActionRow::builder()
            .title("Paket Adı")
            .subtitle(&item.package_name)
            .build();
        info_group.add(&pkg_row);

        let cat_row = adw::ActionRow::builder()
            .title("Kategori")
            .subtitle(item.category.title())
            .build();
        info_group.add(&cat_row);

        if let Some(ref license) = item.license {
            let lic_row = adw::ActionRow::builder()
                .title("Lisans")
                .subtitle(license)
                .build();
            info_group.add(&lic_row);
        }

        if let Some(ref homepage) = item.homepage {
            let hp_row = adw::ActionRow::builder()
                .title("Web Sitesi")
                .subtitle(homepage)
                .build();
            info_group.add(&hp_row);
        }

        content.append(&info_group);

        // Alt Eylemler Bölümü
        let action_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .margin_top(12)
            .build();

        // AUR paketi ise PKGBUILD İncele Butonu
        if item.source == PackageSource::Aur {
            let pkgbuild_btn = Button::builder()
                .label("📄 PKGBUILD İncele")
                .css_classes(["flat"])
                .tooltip_text("Paketin derleme scriptini ve kaynak adreslerini inceleyin")
                .build();

            let pkg_name_clone = item.package_name.clone();
            let dialog_weak = dialog.downgrade();
            pkgbuild_btn.connect_clicked(move |_| {
                if let Some(win) = dialog_weak.upgrade() {
                    PkgbuildDialog::show(&win, &pkg_name_clone);
                }
            });

            action_box.append(&pkgbuild_btn);
        }

        let spacer = Box::builder().hexpand(true).build();
        action_box.append(&spacer);

        // Sepete Ekle Butonu
        let add_btn = Button::builder()
            .css_classes(["suggested-action", "pill"])
            .build();

        let update_btn = {
            let cart = cart_state.clone();
            let pkg = item.package_name.clone();
            let btn = add_btn.clone();
            move || {
                if cart.contains(&pkg) {
                    btn.set_label("Sepetten Kaldır");
                    btn.remove_css_class("suggested-action");
                    btn.add_css_class("destructive-action");
                } else {
                    btn.set_label("+ Sepete Ekle");
                    btn.remove_css_class("destructive-action");
                    btn.add_css_class("suggested-action");
                }
            }
        };

        update_btn();

        let cart_clone = cart_state.clone();
        let item_clone = item.clone();
        let update_clone = update_btn.clone();
        add_btn.connect_clicked(move |_| {
            cart_clone.toggle(item_clone.clone());
            update_clone();
        });

        action_box.append(&add_btn);
        content.append(&action_box);

        root_box.append(&content);
        dialog.set_content(Some(&root_box));
        dialog.present();
    }
}
