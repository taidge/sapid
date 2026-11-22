use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = theme.clone();
    sapid::carousel()
        .theme(theme)
        .name("Size and gap")
        .class("showcase-number-carousel showcase-carousel-wide")
        .count(12usize)
        .viewport_height(200.)
        .control_size(26.)
        .controls_offset(12.)
        .overscan(3)
        .slide_fraction(1. / 3.)
        .slide_gap(16.)
        .looping(true)
        .with_indicators(true)
        .render_slide(move |index| {
            sapid::box_()
                .theme(palette.clone())
                .class("showcase-carousel-number")
                .style(Style {
                    background: Some(palette.accent),
                    color: Some(palette.on_accent),
                    font_size: Some(50.),
                    font_weight: Some(700.),
                    align: Some(Align::Center),
                    justify: Some(Justify::Center),
                    ..Style::new().display(Display::Flex).height(Length::Px(200.))
                })
                .fill(text().content((index + 1).to_string()))
        })
}
