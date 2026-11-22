use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let panel = sapid::dropdown_menu("reference-profile-menu")
        .width(173.3)
        .theme(theme.clone())
        .label("Harriette Spoonlicker")
        .trigger_content(
            group().fill((
                sapid::image(format!(
                    "{}/menu-user-avatar.png",
                    option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                ))
                .decorative()
                .width(Length::Px(38.))
                .height(Length::Px(38.)),
                group().fill((
                    text()
                        .class("site-ref-profile-name")
                        .content("Harriette Spoonlicker"),
                    text()
                        .class("site-ref-profile-email")
                        .content("hspoonlicker@outlook.com"),
                )),
                glory::ui::graphic()
                    .drawing(sapid::Icon::ChevronRight.drawing(theme.muted))
                    .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
            )),
        )
        .item(sapid::menu_label("application", "Application"))
        .item(
            sapid::menu_item("settings")
                .left_section(
                    text()
                        .class("site-ref-menu-glyph site-ref-menu-settings")
                        .content(" "),
                )
                .label("Settings"),
        )
        .item(
            sapid::menu_item("messages")
                .left_section(
                    text()
                        .class("site-ref-menu-glyph site-ref-menu-messages")
                        .content(" "),
                )
                .label("Messages"),
        )
        .item(
            sapid::menu_item("gallery")
                .left_section(
                    text()
                        .class("site-ref-menu-glyph site-ref-menu-gallery")
                        .content(" "),
                )
                .label("Gallery"),
        )
        .item(
            sapid::menu_item("search")
                .left_section(
                    text()
                        .class("site-ref-menu-glyph site-ref-menu-search")
                        .content(" "),
                )
                .label("Search")
                .right_section(text().class("site-ref-menu-shortcut").content("⌘K")),
        )
        .separator("sections-divider")
        .item(sapid::menu_label("danger-zone", "Danger zone"))
        .item(
            sapid::menu_item("transfer")
                .left_section(
                    text()
                        .class("site-ref-menu-glyph site-ref-menu-transfer")
                        .content(" "),
                )
                .label("Transfer my data"),
        )
        .item(
            sapid::menu_item("delete")
                .left_section(
                    text()
                        .class("site-ref-menu-glyph site-ref-menu-delete")
                        .content(" "),
                )
                .label("Delete my account"),
        );
    view().class("site-reference-dialog-buttons").fill(panel)
}
