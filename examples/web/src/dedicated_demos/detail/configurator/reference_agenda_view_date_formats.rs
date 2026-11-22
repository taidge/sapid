use glory::prelude::*;
use sapid::prelude::*;
fn events() -> ScheduleData {
 let start = CivilDate::new(2026, 11, 22).unwrap();
 let events = [
("standup","Morning Standup","2026-11-22T09:00","2026-11-22T09:30",Color(34,139,230,255),false),
("meeting","Team Meeting","2026-11-23T10:00","2026-11-23T11:30",Color(64,192,87,255),false),
("review","Code Review","2026-11-24T14:00","2026-11-24T15:00",Color(121,80,242,255),false),
 ].map(|(key,title,start,end,color,all_day)|{
 let event=ScheduleEvent::new(ScheduleKey::new(key).unwrap(),title,ScheduleInterval::new(parse_local_date_time(start).unwrap(),parse_local_date_time(end).unwrap()).unwrap()).unwrap().color(color);
 if all_day {event.all_day().unwrap()} else {event}
 });
 ScheduleData::new(ScheduleZone::Civil,ScheduleWindow::days(start,7).unwrap(),events).unwrap()
}
pub fn demo(theme:sapid::Theme) -> impl Widget {
 widget(move |scope| {
 let data=scope.cage(events());
let agenda=sapid::agenda_view("date-formats",data).theme(theme.clone())
 .style(Style::new().width(Length::Percent(100.)).max_width(Limit::Px(500.)))
 .on_window_request(move |request| data.revise(|v| *v=ScheduleData::new(request.zone,request.window,v.events().iter().cloned()).unwrap()));
agenda.window_date_label(|d| format!("{} {}", &DateLocale::En.month_name(d.year_month())[..3], d.day()))
 .agenda_date_label(|d| format!("{}, {} {}", DateLocale::En.weekday_short(d.weekday()), d.day(), &DateLocale::En.month_name(d.year_month())[..3])).show_in(scope)
 })
}
