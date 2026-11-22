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
        let chart = sapid::resources_day_view("rooms-day-localization", data)
            .theme(theme.clone())
            .axis(ResourceAxis::Rows)
            .resource_page_size(4)
            .column_width(96.)
            .viewport_height(320.)
            .time_window(ScheduleTimeWindow::new(8 * 60, 18 * 60, 60).unwrap())
            .locale(DateLocale::ZhCn)
            .selection(selected)
            .on_selection_change(move |next| selected.revise(|v| *v = Some(next)))
            .on_window_request(move |request| controls::change_booking_window(data, request));
        chart.show_in(scope)
    })
}
