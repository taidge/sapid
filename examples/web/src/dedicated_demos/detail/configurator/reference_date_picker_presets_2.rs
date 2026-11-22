use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::date_picker("reference-date-picker-presets-2", provider)
        .theme(theme)
        .presets([
            DatePickerPreset::new(
                "Next week",
                DateSelection::Single(Some(date.checked_add_days(7).unwrap())),
            ),
            DatePickerPreset::new(
                "In two weeks",
                DateSelection::Single(Some(date.checked_add_days(14).unwrap())),
            ),
        ])
}
