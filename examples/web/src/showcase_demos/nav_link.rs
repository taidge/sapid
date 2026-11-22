use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let ink = theme.foreground;
    let base = |key: &'static str, label: &'static str| {
        sapid::nav_link(key)
            .theme(theme.clone())
            .label(label)
            .href("#usage")
    };
    let heart = |ink| {
        icon(
            &[
                &[
                    (1., 12.),
                    (7., 12.),
                    (9., 8.),
                    (12., 16.),
                    (15., 11.),
                    (22., 11.),
                ],
                &[
                    (3., 8.),
                    (4., 5.),
                    (7., 3.),
                    (10., 3.),
                    (12., 5.),
                    (15., 3.),
                    (18., 3.),
                    (21., 6.),
                    (21., 8.),
                ],
                &[(4., 15.), (12., 22.), (20., 15.)],
            ],
            ink,
            16.,
        )
    };
    let chevron = |ink| {
        glory::ui::graphic()
            .drawing(sapid::Icon::ChevronRight.drawing(ink))
            .style(Style::new().width(Length::Px(12.)).height(Length::Px(12.)))
    };
    let mut links = vec![
        base("icon", "With icon").left_section(icon(
            &[
                &[(2., 11.), (12., 2.), (22., 11.)],
                &[
                    (5., 9.),
                    (5., 21.),
                    (10., 21.),
                    (10., 14.),
                    (14., 14.),
                    (14., 21.),
                    (19., 21.),
                    (19., 9.),
                ],
            ],
            ink,
            16.,
        )),
        base("section", "With right section")
            .left_section(icon(
                &[
                    &[
                        (2., 19.),
                        (2., 14.),
                        (3., 10.),
                        (7., 6.),
                        (12., 4.),
                        (17., 6.),
                        (21., 10.),
                        (22., 15.),
                        (22., 19.),
                        (2., 19.),
                    ],
                    &[(12., 15.), (17., 9.)],
                ],
                ink,
                16.,
            ))
            .right_section(chevron(ink)),
        base("description", "With description")
            .left_section(
                sapid::badge()
                    .theme(theme.clone())
                    .class("site-nav-alert-count")
                    .content("3"),
            )
            .description("Additional information"),
    ];
    for (key, label, variant, ink) in [
        (
            "subtle",
            "Active subtle",
            sapid::NavLinkVariant::Subtle,
            theme.accent,
        ),
        (
            "light",
            "Active light",
            sapid::NavLinkVariant::Light,
            theme.accent,
        ),
        (
            "filled",
            "Active filled",
            sapid::NavLinkVariant::Filled,
            theme.on_accent,
        ),
    ] {
        links.push(
            base(key, label)
                .variant(variant)
                .current(key == "filled")
                .highlighted(true)
                .left_section(heart(ink))
                .right_section(chevron(ink)),
        );
    }
    let mut rows: Vec<Box<dyn Widget>> = links
        .into_iter()
        .map(|link| Box::new(link) as Box<dyn Widget>)
        .collect();
    rows.insert(
        2,
        Box::new(
            sapid::button()
                .theme(theme.clone())
                .class("sapid-nav-link site-nav-disabled")
                .enabled(false)
                .label("Disabled")
                .left_section(
                    glory::ui::graphic()
                        .drawing(sapid::Icon::Circle.drawing(theme.disabled_text))
                        .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                ),
        ),
    );
    sapid::stack()
        .theme(theme)
        .class("site-nav-usage")
        .gap(0.)
        .fill(rows)
}

fn icon(lines: &[&[(f64, f64)]], color: glory::style::Color, size: f64) -> glory::ui::Graphic {
    use glory::render::{GraphicPoint, PathCommand};
    let mut drawing = sapid::Icon::Check.drawing(color);
    drawing.commands.clear();
    for line in lines {
        for (index, &(x, y)) in line.iter().enumerate() {
            let point = GraphicPoint::new(x, y);
            drawing.commands.push(if index == 0 {
                PathCommand::Move(point)
            } else {
                PathCommand::Line(point)
            });
        }
    }
    glory::ui::graphic().drawing(drawing).style(
        Style::new()
            .width(Length::Px(size))
            .height(Length::Px(size)),
    )
}
