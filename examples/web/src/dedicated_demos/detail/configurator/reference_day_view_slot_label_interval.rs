use glory::prelude::*;
use sapid::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::sample_timeline_events(false));
        sapid::day_view("reference-day-view-slot-label-interval", data)
            .theme(theme.clone())
            .with_slot_text(false)
            .timeline_gutter_width(80.)
            .fit_timeline_columns(28.)
            .slot_height(40.)
            .viewport_height(1632.)
            .time_window(ScheduleTimeWindow::new(8 * 60, 18 * 60, 15).unwrap())
            .slot_label_interval(std::num::NonZeroU16::new(15).unwrap())
            .on_window_request(move |request| {
                data.revise(|current| {
                    *current = ScheduleData::new(request.zone, request.window, current.events().iter().cloned()).unwrap();
                });
            })
            .show_in(scope)
    })
}
