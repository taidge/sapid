use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme:sapid::Theme) -> impl Widget {
 let today=CivilDate::new(2026,11,22).unwrap();
 let provider=DatesProvider::new(today);
sapid::year_picker("year-picker-single-date-in-range",provider).theme(theme.clone()).mode(DatePickerMode::Range)
}
