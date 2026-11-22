use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::year_picker_input("reference-year-picker-input-disabled-state", provider)
        .theme(theme)
        .placeholder("Pick a year")
        .enabled(false)
}
