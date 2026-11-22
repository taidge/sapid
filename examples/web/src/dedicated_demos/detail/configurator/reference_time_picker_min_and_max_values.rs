use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |h, m| LocalTime::new(h, m, 0).unwrap();
    let bounds = TimeBounds::new(Some(at(10, 0)), Some(at(18, 30))).unwrap();
    sapid::stack().theme(theme.clone()).gap(12.).fill((
        sapid::field("time-7-24-field")
            .theme(theme.clone())
            .label("Enter time (24h format)")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .control(move |field| sapid::time_picker("time-7-24").field(field).bounds(bounds)),
        sapid::field("time-7-12-field")
            .theme(theme.clone())
            .label("Enter time (12h format)")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .control(move |field| {
                sapid::time_picker("time-7-12")
                    .field(field)
                    .bounds(bounds)
                    .format(TimeFormat::new(
                        HourCycle::H12,
                        TimePrecision::Minute,
                        DateLocale::En,
                    ))
            }),
    ))
}
