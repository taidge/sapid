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
        let selected = scope.cage(None::<ScheduleKey>);
        let details = scope.bond(move || {
            selected.with(|key| match key.as_ref().map(ScheduleKey::as_str) {
                Some("standup") => (
                    "Team Standup",
                    "Nov 22, 2026 09:00 – 09:30",
                    "Daily team sync",
                    "Room A",
                ),
                Some("workshop") => (
                    "Design Workshop",
                    "Nov 22, 2026 10:00 – 12:00",
                    "Collaborative design session",
                    "Creative Space",
                ),
                Some("review") => (
                    "Code Review",
                    "Nov 23, 2026 14:00 – 15:00",
                    "Review pull requests",
                    "Zoom",
                ),
                _ => ("Event details", "", "", ""),
            })
        });
        let event_title=scope.bond(move||details.get().0.to_owned());
        let interval=scope.bond(move||details.get().1.to_owned());
        let description=scope.bond(move||details.get().2.to_owned());
        let location=scope.bond(move||format!("Location: {}",details.get().3));
        let agenda = sapid::agenda_view("event-click", data)
            .theme(theme.clone())
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(500.)),
            )
            .on_window_request(move |request| {
                data.revise(|v| {
                    *v = ScheduleData::new(request.zone, request.window, v.events().iter().cloned())
                        .unwrap()
                })
            });
        agenda
            .on_selection_change(move |value| {
                if let ScheduleSelection::Event { key, .. } = value {
                    selected.revise(|v| *v = Some(key));
                }
            })
            .show_in(scope)?;
        sapid::dialog("agenda-event-details")
            .theme(theme.clone())
            .show_trigger(false)
            .open(selected.map(|v| v.is_some()))
            .title(event_title)
            .on_open_change(move |open| {
                if !open {
                    selected.revise(|v| *v = None);
                }
            })
            .content(move || {
                sapid::stack().gap(8.).fill((
                    sapid::text().content(interval),
                    sapid::text().content(description),
                    sapid::text().content(location),
                ))
            })
            .show_in(scope)
    })
}
