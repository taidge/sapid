use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("phone_input_disabled-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Phone number")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .enabled(false)
        .control(move |field| {
            sapid::phone_input("phone_input_disabled-example")
                .regions([("US", "United States"), ("GB", "United Kingdom")])
                .field(field)
                .placeholder("Phone number")
        })
}
