use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let blue = theme.accent;
    let preview = sapid::carousel()
        .theme(theme)
        .name("Carousel styles")
        .class("showcase-number-carousel showcase-carousel-wide")
        .style(Style::new().width(Length::Percent(100.)))
        .control_size(26.)
        .controls_offset(12.)
        .count(2usize)
        .looping(true)
        .viewport_height(200.)
        .with_indicators(true)
        .render_slide(move |i| {
            view()
                .class("site-carousel-number")
                .style(Style {
                    background: Some(blue),
                    color: Some(glory::style::Color(255, 255, 255, 255)),
                    ..Style::new().height(Length::Px(200.))
                })
                .fill(text().content((i + 1).to_string()))
        });
    crate::showcase_style_explorer::explorer(
        "carousel-styles",
        Box::new(preview),
        &[
            ("root", "Root element", ".sapid-carousel"),
            (
                "slide",
                "Individual carousel slide",
                ".sapid-carousel-slide",
            ),
            ("container", "Slides container", ".sapid-carousel-container"),
            (
                "viewport",
                "Clipped slide viewport",
                ".sapid-carousel-viewport",
            ),
            (
                "controls",
                "Previous and next controls container",
                ".sapid-carousel-navigation",
            ),
            (
                "control",
                "Previous or next slide button",
                ".sapid-carousel-previous, .sapid-carousel-next",
            ),
            (
                "indicators",
                "Indicators container",
                ".sapid-carousel-indicators",
            ),
            ("indicator", "Indicator button", ".sapid-carousel-indicator"),
        ],
    )
}
