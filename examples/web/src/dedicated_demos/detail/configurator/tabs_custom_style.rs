use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut tabs = sapid::tabs("reference-styled-tabs")
        .theme(theme.clone())
        .class("site-ref-segment-tabs")
        .name("Styled tabs")
        .default_value("settings");
    for (id, label, shape) in [
        ("settings", "Settings", "⚙"),
        ("messages", "Messages", "◌"),
        ("gallery", "Gallery", "▧"),
    ] {
        tabs = tabs.tab(
            sapid::tab(id)
                .label(label)
                .trigger_content(move || {
                    group().fill((
                        text().class("site-ref-tab-glyph").content(shape),
                        text().content(label),
                    ))
                })
                .content(group),
        );
    }
    tabs
}
