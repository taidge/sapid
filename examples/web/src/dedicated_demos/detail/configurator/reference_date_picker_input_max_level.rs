use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::date_picker_input("reference-date-picker-input-max-level", provider)
        .theme(theme)
        .placeholder("Pick a date")
        .max_level(DatePickerLevel::Year)
}
