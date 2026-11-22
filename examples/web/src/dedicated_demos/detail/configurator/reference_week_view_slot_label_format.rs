use glory::prelude::*;
use sapid::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::sample_timeline_events(true));
        sapid::week_view("reference-week-view-slot-label-format", data)
            .theme(theme.clone())
            .with_slot_text(false)
            .timeline_gutter_width(80.)
            .fit_timeline_columns(81.)
            .slot_height(64.)
            .viewport_height(672.)
            .time_window(ScheduleTimeWindow::new(8 * 60, 18 * 60, 60).unwrap())
            .slot_label_format(|hour, minute| {let display = (hour + 11) % 12 + 1; format!("{display}:{minute:02} {}", if hour < 12 { "AM" } else { "PM" })})
            .on_window_request(move |request| {
                data.revise(|current| {
                    *current = ScheduleData::new(request.zone, request.window, current.events().iter().cloned()).unwrap();
                });
            })
            .show_in(scope)
    })
}
