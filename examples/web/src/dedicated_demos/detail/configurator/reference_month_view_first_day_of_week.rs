use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::schedule_example(ScheduleView::Month));
        sapid::month_view("month-view-first-day-of-week", data)
            .theme(theme.clone())
            .viewport_height(340.)
            .week_start(Weekday::Sunday)
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
