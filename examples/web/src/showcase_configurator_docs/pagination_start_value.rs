use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-ref-page-pair")
        .fill((
            text().content("Pages 5–15 (startValue=5, total=15)"),
            sapid::pagination("fifth-page-start")
                .theme(theme)
                .start_value(5)
                .total(Some(15))
                .with_edges(false),
        ))
}
