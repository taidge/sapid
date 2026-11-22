use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::tooltip("disabled-icon-hint")
        .theme(theme.clone())
        .text("Tooltip for disabled button")
        .target(move |target| {
            target.button(
                glory::ui::button()
                    .name("Hover to see tooltip")
                    .label("")
                    .class("site-disabled-button-link site-disabled-icon-link")
                    .fill(
                        glory::ui::graphic()
                            .drawing(illustration(theme.disabled_text))
                            .style(Style::new().width(Length::Px(20.)).height(Length::Px(20.))),
                    ),
            )
        })
}

fn illustration(ink: Color) -> glory::render::GraphicProps {
    use glory::render::{GraphicPoint as P, PathCommand as Path};
    glory::render::GraphicProps {
        width: 256.,
        height: 256.,
        fill: Some(ink),
        commands: vec![
            Path::Move(P::new(178.0, 40.0)),
            Path::Cubic {
                first: P::new(157.35, 40.0),
                second: P::new(139.27, 48.88),
                end: P::new(128.0, 63.89),
            },
            Path::Cubic {
                first: P::new(116.73, 48.88),
                second: P::new(98.65, 40.0),
                end: P::new(78.0, 40.0),
            },
            Path::Cubic {
                first: P::new(43.774, 40.039),
                second: P::new(16.039, 67.774),
                end: P::new(16.0, 102.0),
            },
            Path::Cubic {
                first: P::new(16.0, 172.0),
                second: P::new(119.79, 228.66),
                end: P::new(124.21, 231.0),
            },
            Path::Cubic {
                first: P::new(126.576, 232.273),
                second: P::new(129.424, 232.273),
                end: P::new(131.79, 231.0),
            },
            Path::Cubic {
                first: P::new(136.21, 228.66),
                second: P::new(240.0, 172.0),
                end: P::new(240.0, 102.0),
            },
            Path::Cubic {
                first: P::new(239.961, 67.774),
                second: P::new(212.226, 40.039),
                end: P::new(178.0, 40.0),
            },
            Path::Close,
            Path::Move(P::new(128.0, 214.8)),
            Path::Cubic {
                first: P::new(109.74, 204.16),
                second: P::new(32.0, 155.69),
                end: P::new(32.0, 102.0),
            },
            Path::Cubic {
                first: P::new(32.033, 76.609),
                second: P::new(52.609, 56.033),
                end: P::new(78.0, 56.0),
            },
            Path::Cubic {
                first: P::new(97.45, 56.0),
                second: P::new(113.78, 66.36),
                end: P::new(120.6, 83.0),
            },
            Path::Cubic {
                first: P::new(121.833, 86.001),
                second: P::new(124.756, 87.96),
                end: P::new(128.0, 87.96),
            },
            Path::Cubic {
                first: P::new(131.244, 87.96),
                second: P::new(134.167, 86.001),
                end: P::new(135.4, 83.0),
            },
            Path::Cubic {
                first: P::new(142.22, 66.33),
                second: P::new(158.55, 56.0),
                end: P::new(178.0, 56.0),
            },
            Path::Cubic {
                first: P::new(203.391, 56.033),
                second: P::new(223.967, 76.609),
                end: P::new(224.0, 102.0),
            },
            Path::Cubic {
                first: P::new(224.0, 155.61),
                second: P::new(146.24, 204.15),
                end: P::new(128.0, 214.8),
            },
            Path::Close,
        ],
        ..Default::default()
    }
}
