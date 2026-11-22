use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::date_picker("reference-date-picker-presets", provider)
        .theme(theme)
        .presets([
            DatePickerPreset::new("Today", DateSelection::Single(Some(date))),
            DatePickerPreset::new(
                "Tomorrow",
                DateSelection::Single(Some(date.checked_add_days(1).unwrap())),
            ),
        ])
}
