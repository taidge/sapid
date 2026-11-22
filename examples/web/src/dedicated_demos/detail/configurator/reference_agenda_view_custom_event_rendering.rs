use glory::prelude::*;
use sapid::prelude::*;
fn events() -> ScheduleData {
    let start = CivilDate::new(2026, 11, 22).unwrap();
    let events = [
        (
            "standup",
            "Team Standup",
            "2026-11-22T09:00",
            "2026-11-22T09:30",
            Color(34, 139, 230, 255),
            false,
        ),
        (
            "workshop",
            "Design Workshop",
            "2026-11-22T10:00",
            "2026-11-22T12:00",
            Color(190, 75, 219, 255),
            false,
        ),
        (
            "review",
            "Code Review",
            "2026-11-23T14:00",
            "2026-11-23T15:00",
            Color(64, 192, 87, 255),
            false,
        ),
        (
            "conference",
            "Tech Conference",
            "2026-11-24T00:00",
            "2026-11-25T00:00",
            Color(250, 82, 82, 255),
            true,
        ),
    ]
    .map(|(key, title, start, end, color, all_day)| {
        let event = ScheduleEvent::new(
            ScheduleKey::new(key).unwrap(),
            title,
            ScheduleInterval::new(
                parse_local_date_time(start).unwrap(),
                parse_local_date_time(end).unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
        .color(color);
        if all_day {
            event.all_day().unwrap()
        } else {
            event
        }
    });
    ScheduleData::new(
        ScheduleZone::Civil,
        ScheduleWindow::days(start, 7).unwrap(),
        events,
    )
    .unwrap()
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(events());
        sapid::agenda_view("agenda-custom-events", data)
            .theme(theme.clone())
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(500.)),
            )
            .agenda_event_content(move |key, title, interval| {
                let (category, location) = match key.as_str() {
                    "standup" => ("meeting", "Room A"),
                    "workshop" => ("workshop", "Creative Space"),
                    "review" => ("dev", "Zoom"),
                    _ => ("event", "Convention Center"),
                };
                widget(move |scope| {
                    let interval = interval.clone();
                    let label = scope.bond(move || {
                        let span = interval.get();
                        format!(
                            "{:02}:{:02} – {:02}:{:02}",
                            span.start().time().hour(),
                            span.start().time().minute(),
                            span.end().time().hour(),
                            span.end().time().minute()
                        )
                    });
                    glory::ui::group()

                        .fill((
                            glory::ui::text()
                                .class("site-agenda-custom-title")
                                .content(title.clone()),
                            glory::ui::text()
                                .class("site-agenda-custom-category")
                                .content(category),
                            glory::ui::text()
                                .class("site-agenda-custom-time")
                                .content(label),
                            glory::ui::text()
                                .class("site-agenda-custom-location")
                                .content(format!("📍 {location}")),
                        ))
                        .show_in(scope)
                })
            })
            .on_window_request(move |request| {
                data.revise(|v| {
                    *v = ScheduleData::new(request.zone, request.window, v.events().iter().cloned())
                        .unwrap()
                })
            })
            .show_in(scope)
    })
}
