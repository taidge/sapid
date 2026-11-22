use glory::prelude::*;

const PARTS: &[(&str, &str, &str)] = &[
    (
        "root",
        "Root element with `role=\"menubar\"`",
        ".sapid-menubar",
    ),
    (
        "target",
        "`Menubar.Target` top-level trigger button",
        ".sapid-menubar [role=\"menuitem\"][aria-haspopup=\"menu\"]",
    ),
];

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = widget(move |scope| {
        let theme = theme.clone();
        (sapid::menubar("style-menubar")
            .theme(theme)
            .name("Document menu styles")
            .menus([
                sapid::menubar_menu("file")
                    .label("File")
                    .width(220.)
                    .item(sapid::menu_item("new").label("New file"))
                    .item(sapid::menu_item("save").label("Save")),
                sapid::menubar_menu("edit")
                    .label("Edit")
                    .width(220.)
                    .item(sapid::menu_item("undo").label("Undo"))
                    .item(sapid::menu_item("redo").label("Redo")),
            ]))
        .show_in(scope)
    });
    crate::showcase_style_explorer::explorer("menubar-styles", Box::new(preview), PARTS)
}
