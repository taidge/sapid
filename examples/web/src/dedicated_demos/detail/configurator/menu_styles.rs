use glory::prelude::*;

const PARTS: &[(&str, &str, &str)] = &[
    ("dropdown", "Dropdown element", ".sapid-menu"),
    ("arrow", "Dropdown arrow", ".site-inline-style-menu::before"),
    ("overlay", "Overlay element", ":scope::before"),
    (
        "divider",
        "`Menu.Divider` root element",
        ".sapid-menu-separator",
    ),
    ("label", "`Menu.Label` root element", ".sapid-menu-label"),
    ("item", "`Menu.Item` root element", ".sapid-menu-item"),
    (
        "itemLabel",
        "Label of `Menu.Item`",
        ".sapid-menu-item-label",
    ),
    (
        "itemSection",
        "Left and right sections of `Menu.Item`",
        ".site-menu-style-section",
    ),
    (
        "itemIndicator",
        "Indicator slot of `Menu.CheckboxItem` and `Menu.RadioItem`",
        ".sapid-menu-check input",
    ),
    ("chevron", "Sub menu chevron", ".sapid-menu-chevron"),
    (
        "search",
        "`Menu.Search` input element",
        ".sapid-menu input:not([type=checkbox]):not([type=radio])",
    ),
];

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = widget(move |scope| {
        let theme = theme.clone();
        let mut placement =
            sapid::AnchorPlacement::new(sapid::AnchorSide::Left, sapid::AnchorAlign::Center);
        placement.arrow = Some(glory::render::AnchorArrow {
            size: 8.,
            radius: 1.,
            offset: 8.,
            position: glory::render::AnchorArrowPosition::Center,
        });
        (sapid::dropdown_menu("style-menu")
            .open(true)
            .inline_panel()
            .class("site-inline-style-menu")
            .theme(theme)
            .label("Toggle menu")
            .width(173.3)
            .position(placement)
            .search(sapid::menu_search().name("Search menu"))
            .item(sapid::menu_label("application", "Application"))
        .item(
            sapid::menu_item("settings")
                .left_section(
                    text()
                        .class("site-menu-style-section site-ref-menu-glyph site-ref-menu-settings")
                        .content(" "),
                )
                .label("Settings"),
        )
        .item(
            sapid::menu_item("messages")
                .left_section(
                    text()
                        .class("site-menu-style-section site-ref-menu-glyph site-ref-menu-messages")
                        .content(" "),
                )
                .label("Messages"),
        )
        .item(
            sapid::menu_item("gallery")
                .left_section(
                    text()
                        .class("site-menu-style-section site-ref-menu-glyph site-ref-menu-gallery")
                        .content(" "),
                )
                .label("Gallery"),
        )
        .item(
            sapid::menu_item("search")
                .left_section(
                    text()
                        .class("site-menu-style-section site-ref-menu-glyph site-ref-menu-search")
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
                        .class("site-menu-style-section site-ref-menu-glyph site-ref-menu-transfer")
                        .content(" "),
                )
                .label("Transfer my data"),
        )
        .item(
            sapid::menu_item("delete")
                .left_section(
                    text()
                        .class("site-menu-style-section site-ref-menu-glyph site-ref-menu-delete")
                        .content(" "),
                )
                .label("Delete my account"),
        )
            .item(
                sapid::menu_checkbox("ruler")
                    .label("Show ruler")
                    .checked(true),
            )
            .submenu(
                sapid::menu_submenu("export")
                    .label("Export")
                    .item(sapid::menu_item("pdf").label("PDF")),
            ))
        .show_in(scope)
    });
    crate::showcase_style_explorer::explorer("menu-styles", Box::new(preview), PARTS)
}
