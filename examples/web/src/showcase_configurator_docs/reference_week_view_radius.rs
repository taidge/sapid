use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::schedule_example(ScheduleView::Week));
        sapid::week_view("reference-week-view-radius", data)
            .theme(theme.clone())
            .with_slot_text(false)
            .viewport_height(340.)
            .style(Style::new().radius(0.))
            .on_window_request(move |request| {
                data.revise(|old| {
                    *old = ScheduleData::new(
                        request.zone,
                        request.window,
                        old.events().iter().cloned(),
                    )
                    .unwrap()
                })
            })
            .show_in(scope)
    })
}

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn schedule_date(value: &str) -> sapid::CivilDate {
        use sapid::prelude::*;
        CivilDate::parse_iso(value).unwrap()
    }
    pub fn schedule_time(value: &str) -> sapid::LocalDateTime {
        use sapid::prelude::*;
        parse_local_date_time(value).unwrap()
    }
    pub fn schedule_period(view: sapid::ScheduleView) -> sapid::ScheduleWindow {
        use sapid::prelude::*;
        match view {
            ScheduleView::Day => ScheduleWindow::days(schedule_date("2026-11-04"), 1),
            ScheduleView::Week | ScheduleView::Agenda => {
                ScheduleWindow::days(schedule_date("2026-11-02"), 7)
            }
            ScheduleView::Month | ScheduleView::MobileMonth => {
                ScheduleWindow::month(YearMonth::new(2026, 11).unwrap())
            }
            ScheduleView::Year => ScheduleWindow::year(CivilYear::new(2026).unwrap()),
        }
        .unwrap()
    }
    pub fn schedule_example(view: sapid::ScheduleView) -> sapid::ScheduleData {
        use sapid::prelude::*;
        ScheduleData::new(
            ScheduleZone::Civil,
            schedule_period(view),
            [
                ScheduleEvent::new(
                    ScheduleKey::new("planning").unwrap(),
                    "Project planning",
                    ScheduleInterval::new(
                        schedule_time("2026-11-04T09:00"),
                        schedule_time("2026-11-04T10:30"),
                    )
                    .unwrap(),
                )
                .unwrap()
                .color(glory::style::Color(34, 139, 230, 255)),
                ScheduleEvent::new(
                    ScheduleKey::new("review").unwrap(),
                    "Design review",
                    ScheduleInterval::new(
                        schedule_time("2026-11-04T09:30"),
                        schedule_time("2026-11-04T11:00"),
                    )
                    .unwrap(),
                )
                .unwrap()
                .color(glory::style::Color(121, 80, 242, 255)),
                ScheduleEvent::new(
                    ScheduleKey::new("standup").unwrap(),
                    "Daily standup",
                    ScheduleInterval::new(
                        schedule_time("2026-11-02T11:30"),
                        schedule_time("2026-11-02T12:00"),
                    )
                    .unwrap(),
                )
                .unwrap()
                .color(glory::style::Color(64, 192, 87, 255))
                .recurring(
                    ScheduleRecurrence::new(RecurrenceFrequency::Daily, 1, RecurrenceEnd::Count(7))
                        .unwrap(),
                )
                .unwrap()
                .except_dates([schedule_date("2026-11-05")])
                .unwrap(),
                ScheduleEvent::new(
                    ScheduleKey::new("release").unwrap(),
                    "Release day",
                    ScheduleInterval::all_day(
                        schedule_date("2026-11-04"),
                        schedule_date("2026-11-05"),
                    )
                    .unwrap(),
                )
                .unwrap()
                .color(glory::style::Color(250, 82, 82, 255))
                .all_day()
                .unwrap(),
            ],
        )
        .unwrap()
    }
}
