use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::context_menu("reference-context-actions")
        .theme(theme)
        .width(200.)
        .target_name("Context menu area")
        .label("Actions")
        .target(view().class("site-context-menu-area").fill((
            text().content("Right-click anywhere inside this area"),
            text().content("The menu will open at the cursor position"),
        )))
        .item(sapid::menu_item("open").label("Open"))
        .item(sapid::menu_item("rename").label("Rename"))
        .item(sapid::menu_item("duplicate").label("Duplicate"))
        .separator("divider")
        .item(sapid::menu_item("delete").label("Delete"))
}
