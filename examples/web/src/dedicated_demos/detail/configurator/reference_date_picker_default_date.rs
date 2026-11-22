use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::date_picker("reference-date-picker-default-date", provider)
        .theme(theme)
        .default_month(YearMonth::new(2024, 2).unwrap())
}
