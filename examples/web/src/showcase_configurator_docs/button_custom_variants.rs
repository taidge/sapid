use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .class("site-reference-actions")
        .fill((
            sapid::button()
                .theme(theme.clone())
                .label("Danger variant")
                .class("site-button-danger-reference"),
            sapid::button()
                .theme(theme)
                .label("Primary variant")
                .class("site-button-gradient-reference"),
        ))
}
