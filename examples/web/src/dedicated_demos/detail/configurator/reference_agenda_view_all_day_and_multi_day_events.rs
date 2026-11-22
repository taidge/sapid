use glory::prelude::*;
use sapid::prelude::*;
fn events() -> ScheduleData {
 let start = CivilDate::new(2026, 11, 22).unwrap();
 let events = [
("timed","Team Meeting","2026-11-22T10:00","2026-11-22T11:30",Color(34,139,230,255),false),
("allday","Company Holiday","2026-11-23T00:00","2026-11-24T00:00",Color(250,82,82,255),true),
("multi","Sales Conference","2026-11-24T00:00","2026-11-27T00:00",Color(190,75,219,255),true),
("client","Client Call","2026-11-25T14:00","2026-11-25T15:00",Color(64,192,87,255),false),
("dinner","Dinner Event","2026-11-26T19:00","2026-11-26T21:00",Color(253,126,20,255),false),
 ].map(|(key,title,start,end,color,all_day)|{
 let event=ScheduleEvent::new(ScheduleKey::new(key).unwrap(),title,ScheduleInterval::new(parse_local_date_time(start).unwrap(),parse_local_date_time(end).unwrap()).unwrap()).unwrap().color(color);
 if all_day {event.all_day().unwrap()} else {event}
 });
 ScheduleData::new(ScheduleZone::Civil,ScheduleWindow::days(start,7).unwrap(),events).unwrap()
}
pub fn demo(theme:sapid::Theme) -> impl Widget {
 widget(move |scope| {
 let data=scope.cage(events());
let agenda=sapid::agenda_view("all-day-and-multi-day-events",data).theme(theme.clone())
 .style(Style::new().width(Length::Percent(100.)).max_width(Limit::Px(500.)))
 .on_window_request(move |request| data.revise(|v| *v=ScheduleData::new(request.zone,request.window,v.events().iter().cloned()).unwrap()));
agenda.show_in(scope)
 })
}
