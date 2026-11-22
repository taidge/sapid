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
        .fill(
            sapid::slider("hidden-marks")
                .theme(theme.clone())
                .name("Hidden marks")
                .restrict_to_marks(true)
                .marks([
                    sapid::SliderMark::new(0., "0%"),
                    sapid::SliderMark::unlabeled(25.).hidden(true),
                    sapid::SliderMark::new(50., "50%"),
                    sapid::SliderMark::unlabeled(75.).hidden(true),
                    sapid::SliderMark::new(100., "100%"),
                ]),
        )
}
