use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("password_input_disabled-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Disabled password input")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .enabled(false)
        .control(move |field| {
            sapid::password_input()
                .field(field)
                .placeholder("Disabled password input")
        })
}
