use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let page = scope.cage(0usize);
        let mut previews = Vec::new();
        for (columns, class) in [
            (1, "showcase-carousel-one"),
            (2, "showcase-carousel-two"),
            (3, "showcase-carousel-three"),
        ] {
            let palette = theme.clone();
            previews.push(
                sapid::carousel()
                    .theme(theme.clone())
                    .class(format!(
                        "showcase-number-carousel showcase-carousel-wide {class}"
                    ))
                    .name("Responsive slides")
                    .count(6usize)
                    .current(page)
                    .on_current_change(move |v| page.revise(|old| *old = v))
                    .viewport_height(200.)
                    .control_size(26.)
                    .controls_offset(12.)
                    .overscan(3)
                    .looping(true)
                    .with_indicators(true)
                    .slide_fraction(1. / f64::from(columns))
                    .slide_gap(if columns == 1 { 0. } else { 16. })
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
                    }),
            );
        }
        view()
            .class("showcase-container-carousel")
            .fill(previews)
            .show_in(scope)
    })
}
