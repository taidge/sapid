use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date);
    sapid::year_picker("reference-year-picker-allow-deselect", provider)
        .theme(theme)
        .allow_deselect(true)
        .default_value(PeriodSelection::Single(Some(
            CivilYear::new(date.year()).unwrap(),
        )))
}
