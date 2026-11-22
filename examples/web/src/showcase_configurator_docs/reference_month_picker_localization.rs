use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = CivilDate::new(2026, 11, 4).unwrap();
    let provider = DatesProvider::new(date).locale(DateLocale::ZhCn);
    sapid::month_picker("reference-month-picker-localization", provider).theme(theme)
}
