use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut accordion = sapid::accordion("with-leading-icons")
        .theme(theme.clone())
        .variant(sapid::AccordionVariant::Filled)
        .default_open(["photos"])
        .class("site-accordion-media-icons");
    for (key, label) in [
        ("photos", "Recent photos"),
        ("print", "Print photos"),
        ("camera", "CameraIcon settings"),
    ] {
        let ink = theme.muted;
        accordion = accordion.item(
            sapid::accordion_item(key)
                .label(label)
                .trigger_content(move || {
                    glory::ui::group().fill((
                        glory::ui::graphic()
                            .drawing(media_icon(key, ink))
                            .style(Style::new().width(Length::Px(22.)).height(Length::Px(22.))),
                        text().content(label),
                    ))
                })
                .content(|| text().content("Content")),
        );
    }
    accordion
}
fn media_icon(key: &str, color: Color) -> glory::render::GraphicProps {
    use glory::render::{GraphicPoint, PathCommand};
    let paths: &[&[(f64, f64)]] = match key {
        "photos" => &[
            &[(3., 4.), (21., 4.), (21., 20.), (3., 20.), (3., 4.)],
            &[(3., 17.), (8., 12.), (12., 16.), (15., 13.), (21., 19.)],
            &[(15., 8.), (16., 8.)],
        ],
        "print" => &[
            &[(6., 8.), (6., 3.), (18., 3.), (18., 8.)],
            &[
                (6., 17.),
                (3., 17.),
                (3., 8.),
                (21., 8.),
                (21., 17.),
                (18., 17.),
            ],
            &[(6., 14.), (18., 14.), (18., 21.), (6., 21.), (6., 14.)],
            &[(17., 11.), (18., 11.)],
        ],
        _ => &[
            &[
                (3., 7.),
                (7., 7.),
                (9., 4.),
                (15., 4.),
                (17., 7.),
                (21., 7.),
                (21., 20.),
                (3., 20.),
                (3., 7.),
            ],
            &[
                (12., 9.),
                (15., 10.),
                (16., 13.),
                (15., 16.),
                (12., 17.),
                (9., 16.),
                (8., 13.),
                (9., 10.),
                (12., 9.),
            ],
        ],
    };
    let mut drawing = sapid::Icon::Circle.drawing(color);
    drawing.commands = paths
        .iter()
        .flat_map(|line| {
            line.iter().enumerate().map(|(index, &(x, y))| {
                let point = GraphicPoint::new(x, y);
                if index == 0 {
                    PathCommand::Move(point)
                } else {
                    PathCommand::Line(point)
                }
            })
        })
        .collect();
    drawing
}
