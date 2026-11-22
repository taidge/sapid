use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::schedule_example(ScheduleView::Day));
        sapid::day_view("day-view-interval-minutes", data)
            .theme(theme.clone())
            .with_slot_text(false)
            .viewport_height(340.)
            .time_window(ScheduleTimeWindow::new(9 * 60, 17 * 60, 15).unwrap())
            .on_window_request(move |request| {
                data.revise(|current| {
                    *current = ScheduleData::new(
                        request.zone,
                        request.window,
                        current.events().iter().cloned(),
                    )
                    .unwrap()
                });
            })
            .show_in(scope)
    })
}
