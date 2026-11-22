use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut lime = theme.clone();
    lime.accent = Color(169, 227, 75, 255);
    let mut contrast = lime.clone();
    contrast.on_accent = Color(0, 0, 0, 255);
    sapid::stack()
        .theme(theme)
        .class("site-reference-chunked")
        .fill((
            text().content("autoContrast: off"),
            sapid::pagination("contrast-off")
                .theme(lime)
                .total(Some(10))
                .with_edges(false),
            text().content("autoContrast: on"),
            sapid::pagination("contrast-on")
                .theme(contrast)
                .total(Some(10))
                .with_edges(false),
        ))
}
