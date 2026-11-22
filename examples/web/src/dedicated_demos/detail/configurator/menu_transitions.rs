use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::dropdown_menu("reference-menu-rotate")
        .width(173.3)
        .theme(theme)
        .label("Toggle menu")
        .class("site-reference-rotate-menu")
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
        )
}
