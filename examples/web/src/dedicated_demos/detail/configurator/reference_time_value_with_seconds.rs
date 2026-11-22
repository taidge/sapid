use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let value = Some(LocalTime::new(18, 45, 34).unwrap());
    sapid::stack().theme(theme.clone()).gap(4.).fill((
        sapid::group().theme(theme.clone()).gap(4.).fill((
            sapid::text().theme(theme.clone()).content("24h format:"),
            sapid::time_value(value)
                .theme(theme.clone())
                .format(TimeFormat::new(
                    HourCycle::H24,
                    TimePrecision::Second,
                    DateLocale::En,
                )),
        )),
        sapid::group().theme(theme.clone()).gap(4.).fill((
            sapid::text().theme(theme.clone()).content("12h format:"),
            sapid::time_value(value)
                .theme(theme.clone())
                .format(TimeFormat::new(
                    HourCycle::H12,
                    TimePrecision::Second,
                    DateLocale::En,
                )),
        )),
    ))
}
