use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::date_picker("reference-date-picker-exclude-dates", provider)
        .theme(theme)
        .exclude_dates([
            CivilDate::new(2026, 11, 6).unwrap(),
            CivilDate::new(2026, 11, 13).unwrap(),
        ])
}
