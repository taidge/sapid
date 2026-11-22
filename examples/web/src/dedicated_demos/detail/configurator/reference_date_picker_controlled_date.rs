use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme:sapid::Theme) -> impl Widget {
 let today=CivilDate::new(2026,11,22).unwrap();
 let provider=DatesProvider::new(today);
widget(move|scope|{
 let cursor=scope.cage(today.year_month());
 let range=scope.cage(DateSelection::Range(DateRangeSelection::Empty));
 sapid::date_picker("date-picker-controlled-date",provider).theme(theme.clone()).mode(DatePickerMode::Range).month(cursor).value(range).on_month_change(move|v|cursor.revise(|date|*date=v))
 .on_value_change(move|value|{
   if matches!(value,DateSelection::Range(DateRangeSelection::Start(_))) && let Ok(next)=cursor.get().checked_add(1) {cursor.revise(|date|*date=next);}
   range.revise(|selection|*selection=value);
 }).show_in(scope)
 })
}
