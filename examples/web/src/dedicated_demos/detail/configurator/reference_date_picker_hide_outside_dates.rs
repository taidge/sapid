use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::date_picker("reference-date-picker-hide-outside-dates", provider)
        .theme(theme)
        .hide_outside_dates(true)
}
