use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("time-8-field")
        .theme(theme.clone())
        .label("Paste time here")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::time_picker("time-8")
                .field(field)
                .format(TimeFormat::new(
                    HourCycle::H12,
                    TimePrecision::Second,
                    DateLocale::En,
                ))
                .paste_split(|s| {
                    let (clock, pm) = s
                        .trim()
                        .strip_suffix("PM")
                        .map(|v| (v, true))
                        .or_else(|| s.trim().strip_suffix("AM").map(|v| (v, false)))?;
                    let clock = LocalTime::parse_iso(clock.trim()).ok()?;
                    let hour = clock.hour();
                    if !(1..=12).contains(&hour) {
                        return None;
                    }
                    LocalTime::new(
                        hour % 12 + if pm { 12 } else { 0 },
                        clock.minute(),
                        clock.second(),
                    )
                    .ok()
                })
        })
}
