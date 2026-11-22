use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::mini_calendar(
        "demo-mini-calendar-2",
        DatesProvider::new(CivilDate::parse_iso("2025-04-15").unwrap()),
    )
    .theme(theme)
    .name("Choose date")
    .number_of_days(6)
    .day_style(|date| match date.weekday() {
        Weekday::Saturday | Weekday::Sunday => Style {
            color: Some(Color(224, 49, 49, 255)),
            ..Style::new()
        },
        _ => Style::new(),
    })
}
