use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::tabs("reference-inverted-tabs")
        .theme(theme)
        .name("Inverted tabs")
        .class("site-reference-tabs-inverted")
        .default_value("chat")
        .tab(
            sapid::tab("chat")
                .label("Chat")
                .content(|| text().content("Chat panel")),
        )
        .tab(
            sapid::tab("gallery")
                .label("Gallery")
                .content(|| text().content("Gallery panel")),
        )
        .tab(
            sapid::tab("account")
                .label("Account")
                .content(|| text().content("Account panel")),
        )
}
