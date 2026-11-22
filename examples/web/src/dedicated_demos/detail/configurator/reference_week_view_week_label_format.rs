use glory::prelude::*;
use sapid::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let data = scope.cage(controls::sample_timeline_events(true));
        sapid::week_view("reference-week-view-week-label-format", data)
            .theme(theme.clone())
            .with_slot_text(false)
            .timeline_gutter_width(80.)
            .fit_timeline_columns(81.)
            .slot_height(64.)
            .viewport_height(672.)
            .time_window(ScheduleTimeWindow::new(8 * 60, 18 * 60, 60).unwrap())
            .window_date_label(|date| format!("{} {}, {}",DateLocale::En.month_name(date.year_month()),date.day(),date.year()))
            .on_window_request(move |request| {
                data.revise(|current| {
                    *current = ScheduleData::new(request.zone, request.window, current.events().iter().cloned()).unwrap();
                });
            })
            .show_in(scope)
    })
}
