use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::time_grid("demo-time-grid-2")
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
                LocalTime::new(11, 45, 0).unwrap(),
                900,
                3,
            )
            .unwrap(),
        )
        .disable_times([
            LocalTime::new(10, 45, 0).unwrap(),
            LocalTime::new(11, 0, 0).unwrap(),
            LocalTime::new(11, 30, 0).unwrap(),
        ])
}
