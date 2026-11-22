use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill((
            sapid::slider("plain-marks")
                .theme(theme.clone())
                .name("Unlabeled marks")
                .default_value(40.)
                .marks([10., 40., 95.].map(sapid::SliderMark::unlabeled)),
            sapid::slider("labeled-marks")
                .theme(theme.clone())
                .name("Labeled marks")
                .default_value(40.)
                .marks(
                    [20., 50., 80.].map(|value| sapid::SliderMark::new(value, format!("{value}%"))),
                ),
        ))
}
