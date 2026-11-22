use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("json_input_disabled-example")
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
            sapid::json_input("json_input_disabled-example")
                .field(field)
                .placeholder("Disabled")
                .default_value(r#"{ "a": 1, "B": 2 }"#)
                .show_formatter(false)
        })
}
