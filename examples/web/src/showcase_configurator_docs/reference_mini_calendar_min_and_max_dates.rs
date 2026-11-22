use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::mini_calendar(
        "demo-mini-calendar-3",
        DatesProvider::new(CivilDate::parse_iso("2025-04-15").unwrap()).bounds(
            DateBounds::new(
                Some(CivilDate::parse_iso("2025-04-14").unwrap()),
                Some(CivilDate::parse_iso("2025-04-24").unwrap()),
            )
            .unwrap(),
        ),
    )
    .theme(theme)
    .name("Choose date")
    .number_of_days(6)
}
