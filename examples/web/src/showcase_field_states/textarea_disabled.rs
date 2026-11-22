use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("textarea_disabled-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Disabled")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .enabled(false)
        .control(move |field| {
            sapid::textarea()
                .rows(2)
                .field(field)
                .placeholder("Disabled")
        })
}
