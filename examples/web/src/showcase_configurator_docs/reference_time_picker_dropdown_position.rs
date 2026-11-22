use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("time-5-field")
        .theme(theme.clone())
        .label("Enter time")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .control(move |field| {
            sapid::time_picker("time-5")
                .field(field)
                .with_dropdown(true)
                .popup_placement(sapid::AnchorPlacement {
                    side: sapid::AnchorSide::Top,
                    align: sapid::AnchorAlign::Start,
                    flip: false,
                    shift: false,
                    ..Default::default()
                })
        })
}
