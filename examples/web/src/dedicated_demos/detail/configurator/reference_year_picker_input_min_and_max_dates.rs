use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date).bounds(
        DateBounds::new(
            Some(CivilDate::new(2022, 1, 1).unwrap()),
            Some(CivilDate::new(2028, 12, 31).unwrap()),
        )
        .unwrap(),
    );
    sapid::year_picker_input("reference-year-picker-input-min-and-max-dates", provider)
        .theme(theme)
        .placeholder("Pick a year")
}
