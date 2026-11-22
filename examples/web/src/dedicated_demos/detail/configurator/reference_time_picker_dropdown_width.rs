use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("time-6-field")
        .theme(theme.clone())
        .label("Enter time")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::time_picker("time-6")
                .field(field)
                .with_dropdown(true)
                .format(TimeFormat::new(
                    HourCycle::H12,
                    TimePrecision::Second,
                    DateLocale::En,
                ))
                .popup_placement(sapid::AnchorPlacement {
                    match_width: true,
                    ..Default::default()
                })
        })
}
