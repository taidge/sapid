use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::schedule_example(ScheduleView::Week));
        sapid::week_view("reference-recurring-events-weekview-demo", data)
            .theme(theme.clone())
            .viewport_height(340.)
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
