use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date).bounds(
        DateBounds::new(
            Some(CivilDate::new(2026, 3, 1).unwrap()),
            Some(CivilDate::new(2026, 9, 30).unwrap()),
        )
        .unwrap(),
    );
    sapid::month_picker("reference-month-picker-min-and-max-date", provider).theme(theme)
}
