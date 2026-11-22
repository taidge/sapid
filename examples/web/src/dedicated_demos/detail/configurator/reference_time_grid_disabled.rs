use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::time_grid("demo-time-grid-3")
        .theme(theme)
        .name("Choose time")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(360.)),
        )
        .spec(
            TimeGridSpec::new(
                LocalTime::new(9, 0, 0).unwrap(),
                LocalTime::new(22, 0, 0).unwrap(),
                3600,
                3,
            )
            .unwrap(),
        )
        .enabled(false)
}
