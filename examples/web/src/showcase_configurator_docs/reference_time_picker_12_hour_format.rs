use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("demo-time-picker-2-field")
        .theme(theme.clone())
        .label("Enter time")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::time_picker("demo-time-picker-2")
                .field(field)
                .format(TimeFormat::new(
                    HourCycle::H12,
                    TimePrecision::Minute,
                    DateLocale::En,
                ))
                .default_value(None)
        })
}
