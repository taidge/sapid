use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::mini_calendar(
        "demo-mini-calendar-1",
        DatesProvider::new(CivilDate::parse_iso("2025-04-15").unwrap()),
    )
    .theme(theme)
    .name("Choose date")
    .number_of_days(5)
}
