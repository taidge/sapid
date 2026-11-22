use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut tabs = sapid::tabs("can-deselect-tabs")
        .theme(theme)
        .allow_deactivation(true)
        .default_value("chat");
    for (key, label) in [
        ("chat", "Chat"),
        ("gallery", "Gallery"),
        ("account", "Account"),
    ] {
        tabs = tabs.tab(
            sapid::tab(key)
                .label(label)
                .content(move || text().content(format!("{label} panel"))),
        );
    }
    tabs
}
