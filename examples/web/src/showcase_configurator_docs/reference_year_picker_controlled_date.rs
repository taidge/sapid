use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let today = CivilDate::new(2026, 11, 22).unwrap();
    let provider = DatesProvider::new(today);
    widget(move |scope| {
        let cursor = scope.cage(CivilYear::new(today.year()).unwrap());
        let range = scope.cage(PeriodSelection::Range(PeriodRangeSelection::Empty));
        sapid::year_picker("year-picker-controlled-date", provider)
            .theme(theme.clone())
            .mode(DatePickerMode::Range)
            .year(cursor)
            .value(range)
            .on_year_change(move |v| cursor.revise(|date| *date = v))
            .on_value_change(move |value| {
                if matches!(
                    value,
                    PeriodSelection::Range(PeriodRangeSelection::Start(_))
                ) && let Ok(next) = cursor.get().checked_add(10)
                {
                    cursor.revise(|date| *date = next);
                }
                range.revise(|selection| *selection = value);
            })
            .show_in(scope)
    })
}
