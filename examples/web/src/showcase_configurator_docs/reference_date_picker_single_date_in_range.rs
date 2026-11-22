use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let today = CivilDate::new(2026, 11, 22).unwrap();
    let provider = DatesProvider::new(today);
    sapid::date_picker("date-picker-single-date-in-range", provider)
        .theme(theme.clone())
        .mode(DatePickerMode::Range)
}
