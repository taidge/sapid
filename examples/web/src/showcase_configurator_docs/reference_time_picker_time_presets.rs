use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |h, m| LocalTime::new(h, m, 0).unwrap();
    sapid::field("time-1-field")
        .theme(theme.clone())
        .label("Enter time")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::time_picker("time-1")
                .field(field)
                .with_dropdown(true)
                .presets(vec![
                    at(12, 30),
                    at(15, 45),
                    at(18, 0),
                    at(20, 15),
                    at(22, 30),
                ])
                .close_on_preset_select(false)
        })
}
