use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let row = |key: &'static str| {
        view()
            .class(format!("site-ref-pager-controls {key}"))
            .fill((
                sapid::pagination(key)
                    .theme(theme.clone())
                    .size(sapid::Size::Small)
                    .total(Some(45))
                    .with_edges(false),
                sapid::button()
                    .theme(theme.clone())
                    .size(sapid::Size::Small)
                    .label("sm button"),
                sapid::text_field()
                    .theme(theme.clone())
                    .size(sapid::Size::Small)
                    .name("sm input")
                    .placeholder("sm input"),
            ))
    };
    sapid::stack()
        .theme(theme.clone())
        .gap(16.)
        .fill((row("pager-size-small"), row("pager-size-input")))
}
