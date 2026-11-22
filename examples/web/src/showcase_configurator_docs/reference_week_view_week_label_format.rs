use glory::prelude::*;
use sapid::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::sample_timeline_events(true));
        sapid::week_view("reference-week-view-week-label-format", data)
            .theme(theme.clone())
            .with_slot_text(false)
            .timeline_gutter_width(80.)
            .fit_timeline_columns(81.)
            .slot_height(64.)
            .viewport_height(672.)
            .time_window(ScheduleTimeWindow::new(8 * 60, 18 * 60, 60).unwrap())
            .window_date_label(|date| {
                format!(
                    "{} {}, {}",
                    DateLocale::En.month_name(date.year_month()),
                    date.day(),
                    date.year()
                )
            })
            .on_window_request(move |request| {
                data.revise(|current| {
                    *current = ScheduleData::new(
                        request.zone,
                        request.window,
                        current.events().iter().cloned(),
                    )
                    .unwrap();
                });
            })
            .show_in(scope)
    })
}

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn sample_timeline_events(week: bool) -> sapid::ScheduleData {
        use sapid::prelude::*;
        let start = CivilDate::new(2026, 11, if week { 16 } else { 22 }).unwrap();
        let mut events = Vec::new();
        for (key, title, day_offset, from, to, color) in [
            (
                "standup",
                "Morning Standup",
                0,
                "09:00",
                "09:30",
                Color(34, 139, 230, 255),
            ),
            (
                "meeting",
                "Team Meeting",
                0,
                "10:00",
                "11:30",
                Color(64, 192, 87, 255),
            ),
            (
                "lunch",
                "Lunch Break",
                0,
                "12:00",
                "13:00",
                Color(253, 126, 20, 255),
            ),
            (
                "review",
                "Code Review",
                1,
                "14:00",
                "15:00",
                Color(121, 80, 242, 255),
            ),
            (
                "client",
                "Client Call",
                1,
                "15:30",
                "16:30",
                Color(21, 170, 191, 255),
            ),
            (
                "planning",
                "Project Planning",
                2,
                "10:00",
                "12:00",
                Color(230, 73, 128, 255),
            ),
        ] {
            if !week && day_offset == 2 {
                continue;
            }
            let date = start
                .checked_add_days(if week { day_offset } else { 0 })
                .unwrap();
            let begin = parse_local_date_time(&format!("{date}T{from}")).unwrap();
            let end = parse_local_date_time(&format!("{date}T{to}")).unwrap();
            events.push(
                ScheduleEvent::new(
                    ScheduleKey::new(key).unwrap(),
                    title,
                    ScheduleInterval::new(begin, end).unwrap(),
                )
                .unwrap()
                .color(color),
            );
        }
        let all_day = ScheduleInterval::all_day(start, start.checked_add_days(1).unwrap()).unwrap();
        events.push(
            ScheduleEvent::new(
                ScheduleKey::new("conference").unwrap(),
                "All Day Conference",
                all_day,
            )
            .unwrap()
            .color(Color(250, 82, 82, 255))
            .all_day()
            .unwrap(),
        );
        ScheduleData::new(
            ScheduleZone::Civil,
            ScheduleWindow::days(start, if week { 7 } else { 1 }).unwrap(),
            events,
        )
        .unwrap()
    }
}
