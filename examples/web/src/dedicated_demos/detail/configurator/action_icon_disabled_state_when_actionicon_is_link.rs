use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::anchor()
        .theme(theme.clone())
        .href("?widget=action-icon")
        .name("Open in a new tab")
        .label("Open in a new tab")
        .class("site-disabled-button-link site-disabled-icon-link")
        .fill(
            glory::ui::graphic()
                .drawing(illustration(theme.disabled_text))
                .style(Style::new().width(Length::Px(24.)).height(Length::Px(24.))),
        )
        .on_navigate(|_| {})
}

fn illustration(ink: Color) -> glory::render::GraphicProps {
    use glory::render::{GraphicPoint as P, PathCommand as Path};
    glory::render::GraphicProps {
        width: 256.,
        height: 256.,
        fill: Some(ink),
        commands: vec![
            Path::Move(P::new(224.0, 104.0)),
            Path::Cubic {
                first: P::new(224.0, 108.418),
                second: P::new(220.418, 112.0),
                end: P::new(216.0, 112.0),
            },
            Path::Cubic {
                first: P::new(211.582, 112.0),
                second: P::new(208.0, 108.418),
                end: P::new(208.0, 104.0),
            },
            Path::Line(P::new(208.0, 59.32)),
            Path::Line(P::new(141.67, 125.66)),
            Path::Cubic {
                first: P::new(138.544, 128.786),
                second: P::new(133.476, 128.786),
                end: P::new(130.35, 125.66),
            },
            Path::Cubic {
                first: P::new(127.224, 122.534),
                second: P::new(127.224, 117.466),
                end: P::new(130.35, 114.34),
            },
            Path::Line(P::new(196.68, 48.0)),
            Path::Line(P::new(152.0, 48.0)),
            Path::Cubic {
                first: P::new(147.582, 48.0),
                second: P::new(144.0, 44.418),
                end: P::new(144.0, 40.0),
            },
            Path::Cubic {
                first: P::new(144.0, 35.582),
                second: P::new(147.582, 32.0),
                end: P::new(152.0, 32.0),
            },
            Path::Line(P::new(216.0, 32.0)),
            Path::Cubic {
                first: P::new(220.418, 32.0),
                second: P::new(224.0, 35.582),
                end: P::new(224.0, 40.0),
            },
            Path::Line(P::new(224.0, 104.0)),
            Path::Close,
            Path::Move(P::new(184.0, 128.0)),
            Path::Cubic {
                first: P::new(179.582, 128.0),
                second: P::new(176.0, 131.582),
                end: P::new(176.0, 136.0),
            },
            Path::Line(P::new(176.0, 208.0)),
            Path::Line(P::new(48.0, 208.0)),
            Path::Line(P::new(48.0, 80.0)),
            Path::Line(P::new(120.0, 80.0)),
            Path::Cubic {
                first: P::new(124.418, 80.0),
                second: P::new(128.0, 76.418),
                end: P::new(128.0, 72.0),
            },
            Path::Cubic {
                first: P::new(128.0, 67.582),
                second: P::new(124.418, 64.0),
                end: P::new(120.0, 64.0),
            },
            Path::Line(P::new(48.0, 64.0)),
            Path::Cubic {
                first: P::new(39.163, 64.0),
                second: P::new(32.0, 71.163),
                end: P::new(32.0, 80.0),
            },
            Path::Line(P::new(32.0, 208.0)),
            Path::Cubic {
                first: P::new(32.0, 216.837),
                second: P::new(39.163, 224.0),
                end: P::new(48.0, 224.0),
            },
            Path::Line(P::new(176.0, 224.0)),
            Path::Cubic {
                first: P::new(184.837, 224.0),
                second: P::new(192.0, 216.837),
                end: P::new(192.0, 208.0),
            },
            Path::Line(P::new(192.0, 136.0)),
            Path::Cubic {
                first: P::new(192.0, 131.582),
                second: P::new(188.418, 128.0),
                end: P::new(184.0, 128.0),
            },
            Path::Close,
        ],
        ..Default::default()
    }
}
