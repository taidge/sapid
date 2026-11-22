use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = theme.clone();
    sapid::carousel()
        .theme(theme)
        .name("Drag free")
        .class("showcase-number-carousel ")
        .count(5usize)
        .viewport_height(200.)
        .control_size(26.)
        .controls_offset(12.)
        .drag_free(true)
        .slide_gap(16.)
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
