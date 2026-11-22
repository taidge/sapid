use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme:sapid::Theme) -> impl Widget {
 let today=CivilDate::new(2026,11,22).unwrap();
 let provider=DatesProvider::new(today);
sapid::month_picker("month-picker-single-date-in-range",provider).theme(theme.clone()).mode(DatePickerMode::Range)
}
