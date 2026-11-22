use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut lime = theme.clone();
    lime.accent = Color(169, 227, 75, 255);
    sapid::group()
        .theme(theme)
        .class("site-reference-actions")
        .fill((
            sapid::button().theme(lime.clone()).label("Default"),
            sapid::button()
                .theme(lime)
                .auto_contrast(true)
                .label("Auto contrast"),
        ))
}
