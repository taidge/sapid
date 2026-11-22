use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().class("site-reference-dialog-buttons").fill(
        sapid::dropdown_menu("native-link-menu")
            .theme(theme.clone())
            .label("Toggle menu")
            .item(
                sapid::menu_item("website")
                    .label("Project website")
                    .href("https://glory.rs", glory::render::LinkOpen::Current),
            )
            .item(
                sapid::menu_item("external")
                    .label("External link")
                    .left_section(glory::ui::text().content("↗"))
                    .href("https://glory.rs", glory::render::LinkOpen::NewContext),
            ),
    )
}
