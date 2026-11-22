use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::meeting_rooms(
            ResourceScheduleView::Week,
            false,
            false,
        ));
        let selected = scope.cage(None::<ScheduleSelection>);
        let minutes = scope.cage("240".to_string());
        let switch = controls::segmented_choice(
            "rooms-week-multi-hour-intervals-interval",
            "Interval",
            minutes,
            &[("60", "1 hour"), ("120", "2 hours"), ("240", "4 hours")],
            &theme,
        );
        let theme = theme.clone();
        let chart = each(scope.bond(move || vec![minutes.get()]))
            .key(Clone::clone)
            .children(move |entry| {
                let step = entry.value.get().parse().unwrap_or(60);
                sapid::resources_week_view(
                    "rooms-week-multi-hour-intervals",
                    data,
                )
                .theme(theme.clone())
                .axis(ResourceAxis::Rows)
                .resource_page_size(4)
                .column_width(96.)
                .viewport_height(320.)
                .time_window(ScheduleTimeWindow::new(8 * 60, 20 * 60, step).unwrap())
                .selection(selected)
                .on_selection_change(move |next| selected.revise(|v| *v = Some(next)))
                .on_window_request(move |request| controls::change_booking_window(data, request))
            });
        view().fill((switch, chart)).show_in(scope)
    })
}
