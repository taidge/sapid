use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::schedule_example(ScheduleView::Week));
        sapid::week_view("week-view-static-mode", data)
            .theme(theme.clone())
            .with_slot_text(false)
            .viewport_height(340.)
            .readonly(true)
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
