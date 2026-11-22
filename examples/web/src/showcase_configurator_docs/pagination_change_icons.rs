use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let icons = [
        sapid::Icon::ArrowLeft,
        sapid::Icon::ArrowLeft,
        sapid::Icon::ArrowRight,
        sapid::Icon::ArrowRight,
    ];
    sapid::stack()
        .theme(theme.clone())
        .class("site-ref-page-pair site-ref-custom-pages")
        .fill((
            sapid::pagination("changed-icons")
                .theme(theme.clone())
                .total(Some(10))
                .control_icons(icons),
            sapid::pagination("compound-icons")
                .theme(theme)
                .total(Some(10))
                .control_icons(icons),
        ))
}
