use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::month_picker("reference-month-picker-multiple-dates", provider)
        .theme(theme)
        .mode(DatePickerMode::Multiple)
}
