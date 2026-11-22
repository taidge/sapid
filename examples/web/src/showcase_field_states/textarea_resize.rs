use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("textarea-resize")
        .class("site-field-state-example")
        .theme(theme)
        .label("Disabled")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(|field| {
            sapid::textarea()
                .field(field)
                .class("site-vertical-resize")
                .rows(2)
                .placeholder("Your comment")
        })
}
