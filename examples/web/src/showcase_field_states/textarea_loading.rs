use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::textarea()
                .rows(2)
                .theme(theme)
                .name("Your comment")
                .placeholder("Your comment")
                .loading(true),
        )
}
