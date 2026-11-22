use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = theme.clone();
    sapid::carousel()
        .theme(theme)
        .class("showcase-number-carousel")
        .name("Custom arrow controls")
        .viewport_height(180.)
        .count(5usize)
        .control_size(26.)
        .controls_offset(12.)
        .control_icons(sapid::Icon::ArrowLeft, sapid::Icon::ArrowRight)
        .render_slide(move |index| {
            sapid::box_()
                .theme(palette.clone())
                .style(Style {
                    background: Some(palette.accent),
                    color: Some(palette.on_accent),
                    font_size: Some(50.),
                    font_weight: Some(700.),
                    align: Some(Align::Center),
                    justify: Some(Justify::Center),
                    ..Style::new().display(Display::Flex).height(Length::Px(180.))
                })
                .fill(text().content((index + 1).to_string()))
        })
}
