use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::meeting_rooms(
            ResourceScheduleView::Day,
            false,
            false,
        ));
        let selected = scope.cage(None::<ScheduleSelection>);
        let chart = sapid::resources_day_view("rooms-day-radius", data)
            .theme(theme.clone())
            .axis(ResourceAxis::Rows)
            .resource_page_size(4)
            .column_width(96.)
            .viewport_height(320.)
            .time_window(ScheduleTimeWindow::new(8 * 60, 18 * 60, 60).unwrap())
            .style(
                Style::new()
                    .radius(8.)
                    .overflow_x(glory::style::Overflow::Hidden),
            )
            .selection(selected)
            .on_selection_change(move |next| selected.revise(|v| *v = Some(next)))
            .on_window_request(move |request| controls::change_booking_window(data, request));
        chart.show_in(scope)
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
    pub fn meeting_rooms(
        view: sapid::ResourceScheduleView,
        all_day: bool,
        recurring: bool,
    ) -> sapid::ResourceScheduleData {
        use sapid::prelude::*;
        let view = match view {
            ResourceScheduleView::Day => ScheduleView::Day,
            ResourceScheduleView::Week => ScheduleView::Week,
            ResourceScheduleView::Month => ScheduleView::Month,
        };
        let rooms = ScheduleResources::new(
            [
                ("tokyo", "Meeting room: Tokyo"),
                ("paris", "Meeting room: Paris"),
                ("new-york", "Meeting room: New York"),
                ("london", "Meeting room: London"),
            ]
            .map(|(id, label)| {
                ScheduleResource::new(ScheduleKey::new(id).unwrap(), label).unwrap()
            }),
        )
        .unwrap();
        let mut events = [
            (
                "Team Standup",
                "tokyo",
                "09:00",
                "09:30",
                Color(34, 139, 230, 255),
            ),
            (
                "Sprint Planning",
                "tokyo",
                "10:00",
                "11:30",
                Color(64, 192, 87, 255),
            ),
            (
                "Client Call",
                "paris",
                "09:30",
                "10:30",
                Color(121, 80, 242, 255),
            ),
            (
                "Design Review",
                "paris",
                "13:00",
                "14:00",
                Color(253, 126, 20, 255),
            ),
            (
                "1:1 Meeting",
                "new-york",
                "11:00",
                "11:30",
                Color(21, 170, 191, 255),
            ),
            (
                "Workshop",
                "new-york",
                "14:00",
                "16:00",
                Color(230, 73, 128, 255),
            ),
            (
                "Architecture Review",
                "london",
                "10:00",
                "11:00",
                Color(250, 82, 82, 255),
            ),
            (
                "Retrospective",
                "london",
                "15:00",
                "16:00",
                Color(190, 75, 219, 255),
            ),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, (title, room, start, end, color))| {
            let interval = ScheduleInterval::new(
                schedule_time(&format!("2026-11-04T{start}")),
                schedule_time(&format!("2026-11-04T{end}")),
            )
            .unwrap();
            let event = ScheduleEvent::new(
                ScheduleKey::new(format!("booking-{i}")).unwrap(),
                title,
                interval,
            )
            .unwrap()
            .color(color)
            .resource(ScheduleKey::new(room).unwrap());
            if recurring && i == 0 {
                event
                    .recurring(
                        ScheduleRecurrence::new(
                            RecurrenceFrequency::Daily,
                            1,
                            RecurrenceEnd::Count(14),
                        )
                        .unwrap(),
                    )
                    .unwrap()
            } else {
                event
            }
        })
        .collect::<Vec<_>>();
        if all_day {
            events.push(
                ScheduleEvent::new(
                    ScheduleKey::new("conference").unwrap(),
                    "Conference",
                    ScheduleInterval::all_day(
                        schedule_date("2026-11-04"),
                        schedule_date("2026-11-06"),
                    )
                    .unwrap(),
                )
                .unwrap()
                .all_day()
                .unwrap()
                .resource(ScheduleKey::new("paris").unwrap())
                .color(Color(253, 126, 20, 255)),
            );
        }
        ResourceScheduleData::new(
            ScheduleData::new(ScheduleZone::Civil, schedule_period(view), events).unwrap(),
            rooms,
            ScheduleResourceWindow::new(0, 4).unwrap(),
        )
        .unwrap()
    }

    pub fn change_booking_window(
        data: Cage<sapid::ResourceScheduleData>,
        request: sapid::ResourceScheduleRequest,
    ) {
        use sapid::prelude::*;
        data.revise(|current| {
            *current = ResourceScheduleData::new(
                ScheduleData::new(
                    request.zone,
                    request.window,
                    current.data().events().iter().cloned(),
                )
                .unwrap(),
                current.resources().clone(),
                request.resource_window,
            )
            .unwrap();
        });
    }
}
