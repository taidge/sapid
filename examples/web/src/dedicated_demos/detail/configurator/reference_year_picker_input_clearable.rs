use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::year_picker_input("reference-year-picker-input-clearable", provider)
        .theme(theme)
        .placeholder("Pick a year")
        .clearable(true)
        .default_value(PeriodSelection::Single(Some(
            CivilYear::new(date.year()).unwrap(),
        )))
}
