use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("demo-time-picker-7-field")
        .theme(theme.clone())
        .label("Enter time")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::time_picker("demo-time-picker-7")
                .field(field)
                .enabled(false)
                .default_value(None)
        })
}
