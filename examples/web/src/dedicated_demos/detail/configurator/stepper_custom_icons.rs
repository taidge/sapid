use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(1usize);
        let ink = theme.foreground;
        sapid::stepper("custom-icons")
            .theme(theme.clone())
            .class("site-ref-icon-step site-ref-custom-step")
            .name("Registration progress")
            .active(current)
            .allow_skip(true)
            .selectable(true)
            .on_step_change(move |n| current.revise(|v| *v = n))
            .steps(
                [
                    ("account", "Step 1", "Create an account", 0usize),
                    ("verify", "Step 2", "Verify email", 1usize),
                    ("secure", "Step 3", "Get full access", 2usize),
                ]
                .map(|(key, label, description, icon)| {
                    sapid::step(key)
                        .label(label)
                        .description(description)
                        .icon(move || {
                            glory::ui::graphic()
                                .drawing(drawing(icon, ink))
                                .style(Style::new().width(Length::Px(18.)).height(Length::Px(18.)))
                        })
                }),
            )
            .show_in(scope)
    })
}

// Regular Phosphor Icons paths are used by the reference example.
fn drawing(index: usize, ink: Color) -> glory::render::GraphicProps {
    use glory::render::{GraphicPoint as P, PathCommand as C};
    let commands = match index {
        0 => vec![
            C::Move(P::new(144.0, 157.68)),
            C::Cubic {
                first: P::new(169.766, 141.632),
                second: P::new(181.775, 110.45),
                end: P::new(173.43, 81.265),
            },
            C::Cubic {
                first: P::new(165.084, 52.079),
                second: P::new(138.405, 31.96),
                end: P::new(108.05, 31.96),
            },
            C::Cubic {
                first: P::new(77.695, 31.96),
                second: P::new(51.016, 52.079),
                end: P::new(42.67, 81.265),
            },
            C::Cubic {
                first: P::new(34.325, 110.45),
                second: P::new(46.334, 141.632),
                end: P::new(72.1, 157.68),
            },
            C::Cubic {
                first: P::new(51.45, 164.44),
                second: P::new(32.87, 177.07),
                end: P::new(17.93, 194.85),
            },
            C::Cubic {
                first: P::new(15.086, 198.233),
                second: P::new(15.522, 203.281),
                end: P::new(18.905, 206.125),
            },
            C::Cubic {
                first: P::new(22.288, 208.969),
                second: P::new(27.336, 208.533),
                end: P::new(30.18, 205.15),
            },
            C::Cubic {
                first: P::new(50.25, 181.19),
                second: P::new(77.91, 168.0),
                end: P::new(108.0, 168.0),
            },
            C::Cubic {
                first: P::new(138.09, 168.0),
                second: P::new(165.75, 181.19),
                end: P::new(185.87, 205.15),
            },
            C::Cubic {
                first: P::new(188.714, 208.533),
                second: P::new(193.762, 208.969),
                end: P::new(197.145, 206.125),
            },
            C::Cubic {
                first: P::new(200.528, 203.281),
                second: P::new(200.964, 198.233),
                end: P::new(198.12, 194.85),
            },
            C::Cubic {
                first: P::new(183.18, 177.07),
                second: P::new(164.6, 164.44),
                end: P::new(144.0, 157.68),
            },
            C::Close,
            C::Move(P::new(56.0, 100.0)),
            C::Cubic {
                first: P::new(56.0, 71.281),
                second: P::new(79.281, 48.0),
                end: P::new(108.0, 48.0),
            },
            C::Cubic {
                first: P::new(136.719, 48.0),
                second: P::new(160.0, 71.281),
                end: P::new(160.0, 100.0),
            },
            C::Cubic {
                first: P::new(160.0, 128.719),
                second: P::new(136.719, 152.0),
                end: P::new(108.0, 152.0),
            },
            C::Cubic {
                first: P::new(79.295, 151.967),
                second: P::new(56.033, 128.705),
                end: P::new(56.0, 100.0),
            },
            C::Close,
            C::Move(P::new(253.66, 133.66)),
            C::Line(P::new(221.66, 165.66)),
            C::Cubic {
                first: P::new(220.159, 167.162),
                second: P::new(218.123, 168.006),
                end: P::new(216.0, 168.006),
            },
            C::Cubic {
                first: P::new(213.877, 168.006),
                second: P::new(211.841, 167.162),
                end: P::new(210.34, 165.66),
            },
            C::Line(P::new(194.34, 149.66)),
            C::Cubic {
                first: P::new(191.214, 146.534),
                second: P::new(191.214, 141.466),
                end: P::new(194.34, 138.34),
            },
            C::Cubic {
                first: P::new(197.466, 135.214),
                second: P::new(202.534, 135.214),
                end: P::new(205.66, 138.34),
            },
            C::Line(P::new(216.0, 148.69)),
            C::Line(P::new(242.34, 122.34)),
            C::Cubic {
                first: P::new(245.466, 119.214),
                second: P::new(250.534, 119.214),
                end: P::new(253.66, 122.34),
            },
            C::Cubic {
                first: P::new(256.786, 125.466),
                second: P::new(256.786, 130.534),
                end: P::new(253.66, 133.66),
            },
            C::Close,
        ],
        1 => vec![
            C::Move(P::new(228.44, 89.34)),
            C::Line(P::new(132.44, 25.34)),
            C::Cubic {
                first: P::new(129.752, 23.546),
                second: P::new(126.248, 23.546),
                end: P::new(123.56, 25.34),
            },
            C::Line(P::new(27.56, 89.34)),
            C::Cubic {
                first: P::new(25.334, 90.825),
                second: P::new(23.998, 93.324),
                end: P::new(24.0, 96.0),
            },
            C::Line(P::new(24.0, 200.0)),
            C::Cubic {
                first: P::new(24.0, 208.837),
                second: P::new(31.163, 216.0),
                end: P::new(40.0, 216.0),
            },
            C::Line(P::new(216.0, 216.0)),
            C::Cubic {
                first: P::new(224.837, 216.0),
                second: P::new(232.0, 208.837),
                end: P::new(232.0, 200.0),
            },
            C::Line(P::new(232.0, 96.0)),
            C::Cubic {
                first: P::new(232.002, 93.324),
                second: P::new(230.666, 90.825),
                end: P::new(228.44, 89.34),
            },
            C::Close,
            C::Move(P::new(96.72, 152.0)),
            C::Line(P::new(40.0, 192.0)),
            C::Line(P::new(40.0, 111.53)),
            C::Line(P::new(96.72, 152.0)),
            C::Close,
            C::Move(P::new(113.09, 160.0)),
            C::Line(P::new(142.91, 160.0)),
            C::Line(P::new(199.54, 200.0)),
            C::Line(P::new(56.46, 200.0)),
            C::Line(P::new(113.09, 160.0)),
            C::Close,
            C::Move(P::new(159.28, 152.0)),
            C::Line(P::new(216.0, 111.53)),
            C::Line(P::new(216.0, 192.0)),
            C::Line(P::new(159.28, 152.0)),
            C::Close,
            C::Move(P::new(128.0, 41.61)),
            C::Line(P::new(209.91, 96.22)),
            C::Line(P::new(142.91, 144.0)),
            C::Line(P::new(113.11, 144.0)),
            C::Line(P::new(46.11, 96.22)),
            C::Line(P::new(128.0, 41.61)),
            C::Close,
        ],
        _ => vec![
            C::Move(P::new(208.0, 40.0)),
            C::Line(P::new(48.0, 40.0)),
            C::Cubic {
                first: P::new(39.163, 40.0),
                second: P::new(32.0, 47.163),
                end: P::new(32.0, 56.0),
            },
            C::Line(P::new(32.0, 112.0)),
            C::Cubic {
                first: P::new(32.0, 164.72),
                second: P::new(57.52, 196.67),
                end: P::new(78.93, 214.19),
            },
            C::Cubic {
                first: P::new(101.99, 233.05),
                second: P::new(124.93, 239.45),
                end: P::new(125.93, 239.72),
            },
            C::Cubic {
                first: P::new(127.305, 240.094),
                second: P::new(128.755, 240.094),
                end: P::new(130.13, 239.72),
            },
            C::Cubic {
                first: P::new(131.13, 239.45),
                second: P::new(154.04, 233.05),
                end: P::new(177.13, 214.19),
            },
            C::Cubic {
                first: P::new(198.48, 196.67),
                second: P::new(224.0, 164.72),
                end: P::new(224.0, 112.0),
            },
            C::Line(P::new(224.0, 56.0)),
            C::Cubic {
                first: P::new(224.0, 47.163),
                second: P::new(216.837, 40.0),
                end: P::new(208.0, 40.0),
            },
            C::Close,
            C::Move(P::new(208.0, 112.0)),
            C::Cubic {
                first: P::new(208.0, 149.07),
                second: P::new(194.34, 179.16),
                end: P::new(167.4, 201.42),
            },
            C::Cubic {
                first: P::new(155.673, 211.077),
                second: P::new(142.336, 218.592),
                end: P::new(128.0, 223.62),
            },
            C::Cubic {
                first: P::new(113.852, 218.68),
                second: P::new(100.679, 211.298),
                end: P::new(89.08, 201.81),
            },
            C::Cubic {
                first: P::new(61.82, 179.51),
                second: P::new(48.0, 149.3),
                end: P::new(48.0, 112.0),
            },
            C::Line(P::new(48.0, 56.0)),
            C::Line(P::new(208.0, 56.0)),
            C::Line(P::new(208.0, 112.0)),
            C::Close,
            C::Move(P::new(82.34, 141.66)),
            C::Cubic {
                first: P::new(79.214, 138.534),
                second: P::new(79.214, 133.466),
                end: P::new(82.34, 130.34),
            },
            C::Cubic {
                first: P::new(85.466, 127.214),
                second: P::new(90.534, 127.214),
                end: P::new(93.66, 130.34),
            },
            C::Line(P::new(112.0, 148.69)),
            C::Line(P::new(162.34, 98.34)),
            C::Cubic {
                first: P::new(165.466, 95.214),
                second: P::new(170.534, 95.214),
                end: P::new(173.66, 98.34),
            },
            C::Cubic {
                first: P::new(176.786, 101.466),
                second: P::new(176.786, 106.534),
                end: P::new(173.66, 109.66),
            },
            C::Line(P::new(117.66, 165.66)),
            C::Cubic {
                first: P::new(116.159, 167.162),
                second: P::new(114.123, 168.006),
                end: P::new(112.0, 168.006),
            },
            C::Cubic {
                first: P::new(109.877, 168.006),
                second: P::new(107.841, 167.162),
                end: P::new(106.34, 165.66),
            },
            C::Line(P::new(82.34, 141.66)),
            C::Close,
        ],
    };
    glory::render::GraphicProps {
        width: 256.,
        height: 256.,
        fill: Some(ink),
        commands,
        ..Default::default()
    }
}
