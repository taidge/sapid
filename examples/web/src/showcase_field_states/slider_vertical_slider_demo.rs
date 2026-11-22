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
            sapid::group().gap(40.).fill((
                sapid::slider("vertical-value")
                    .theme(theme.clone())
                    .name("Vertical slider")
                    .orientation(glory::render::Orientation::Vertical)
                    .default_value(45.)
                    .style(Style::new().height(Length::Px(170.)))
                    .marks([
                        sapid::SliderMark::new(20., "20%"),
                        sapid::SliderMark::new(80., "80%"),
                    ]),
                sapid::range_slider("vertical-range")
                    .theme(theme.clone())
                    .name("Vertical range")
                    .orientation(glory::render::Orientation::Vertical)
                    .default_value([25., 65.])
                    .style(Style::new().height(Length::Px(170.))),
            )),
        )
}
