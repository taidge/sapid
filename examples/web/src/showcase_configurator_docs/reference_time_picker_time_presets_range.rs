use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let times = (6 * 60..=18 * 60)
        .step_by(90)
        .map(|minutes| LocalTime::new((minutes / 60) as u8, (minutes % 60) as u8, 0).unwrap())
        .collect::<Vec<_>>();
    sapid::field("time-3-field")
        .theme(theme.clone())
        .label("Enter time")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::time_picker("time-3")
                .field(field)
                .with_dropdown(true)
                .presets(times.clone())
                .close_on_preset_select(false)
        })
}
