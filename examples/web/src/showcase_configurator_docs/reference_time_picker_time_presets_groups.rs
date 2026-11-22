use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let at = |h, m| LocalTime::new(h, m, 0).unwrap();
    sapid::field("time-2-field")
        .theme(theme.clone())
        .label("Enter time")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::time_picker("time-2")
                .field(field)
                .with_dropdown(true)
                .max_dropdown_content_height(300.)
                .presets(vec![
                    TimePickerPresetGroup::new(
                        "Morning",
                        [
                            at(6, 0),
                            at(6, 30),
                            at(8, 0),
                            at(8, 30),
                            at(10, 0),
                            at(10, 45),
                        ],
                    ),
                    TimePickerPresetGroup::new(
                        "Afternoon",
                        [
                            at(12, 0),
                            at(12, 30),
                            at(14, 0),
                            at(14, 45),
                            at(16, 0),
                            at(17, 30),
                        ],
                    ),
                    TimePickerPresetGroup::new(
                        "Evening",
                        [at(18, 0), at(18, 30), at(20, 0), at(22, 0)],
                    ),
                ])
                .close_on_preset_select(false)
        })
}
