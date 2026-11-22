use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut lime = theme.clone();
    lime.accent = Color(169, 227, 75, 255);
    sapid::group()
        .theme(theme)
        .class("site-reference-actions")
        .fill(Vec::from([false, true].map(|contrast| {
            let ink = if contrast {
                lime.contrasting_foreground(lime.accent, lime.on_accent)
            } else {
                lime.on_accent
            };
            sapid::action_icon(if contrast {
                "autoContrast action icon"
            } else {
                "default action icon"
            })
            .theme(lime.clone())
            .variant(sapid::ActionIconVariant::Filled)
            .size(sapid::Size::Large)
            .auto_contrast(contrast)
            .icon(
                glory::ui::graphic()
                    .drawing(illustration(ink))
                    .style(Style::new().width(Length::Px(20.)).height(Length::Px(20.))),
            )
        })))
}

fn illustration(ink: Color) -> glory::render::GraphicProps {
    use glory::render::{GraphicPoint as P, PathCommand as Path};
    glory::render::GraphicProps {
        width: 256.,
        height: 256.,
        fill: Some(ink),
        commands: vec![
            Path::Move(P::new(72.0, 128.0)),
            Path::Cubic {
                first: P::new(72.057, 148.985),
                second: P::new(67.208, 169.692),
                end: P::new(57.84, 188.47),
            },
            Path::Cubic {
                first: P::new(56.622, 191.121),
                second: P::new(54.059, 192.899),
                end: P::new(51.149, 193.11),
            },
            Path::Cubic {
                first: P::new(48.24, 193.321),
                second: P::new(45.446, 191.932),
                end: P::new(43.859, 189.485),
            },
            Path::Cubic {
                first: P::new(42.271, 187.038),
                second: P::new(42.141, 183.921),
                end: P::new(43.52, 181.35),
            },
            Path::Cubic {
                first: P::new(51.78, 164.782),
                second: P::new(56.054, 146.513),
                end: P::new(56.0, 128.0),
            },
            Path::Cubic {
                first: P::new(55.962, 106.13),
                second: P::new(65.903, 85.438),
                end: P::new(83.0, 71.8),
            },
            Path::Cubic {
                first: P::new(86.449, 69.039),
                second: P::new(91.484, 69.596),
                end: P::new(94.245, 73.045),
            },
            Path::Cubic {
                first: P::new(97.006, 76.494),
                second: P::new(96.449, 81.529),
                end: P::new(93.0, 84.29),
            },
            Path::Cubic {
                first: P::new(79.699, 94.894),
                second: P::new(71.966, 110.99),
                end: P::new(72.0, 128.0),
            },
            Path::Close,
            Path::Move(P::new(128.0, 120.0)),
            Path::Cubic {
                first: P::new(123.582, 120.0),
                second: P::new(120.0, 123.582),
                end: P::new(120.0, 128.0),
            },
            Path::Cubic {
                first: P::new(119.998, 159.168),
                second: P::new(112.085, 189.825),
                end: P::new(97.0, 217.1),
            },
            Path::Cubic {
                first: P::new(94.857, 220.966),
                second: P::new(96.254, 225.837),
                end: P::new(100.12, 227.98),
            },
            Path::Cubic {
                first: P::new(103.986, 230.123),
                second: P::new(108.857, 228.726),
                end: P::new(111.0, 224.86),
            },
            Path::Cubic {
                first: P::new(127.396, 195.209),
                second: P::new(135.998, 161.882),
                end: P::new(136.0, 128.0),
            },
            Path::Cubic {
                first: P::new(136.0, 123.582),
                second: P::new(132.418, 120.0),
                end: P::new(128.0, 120.0),
            },
            Path::Close,
            Path::Move(P::new(128.0, 88.0)),
            Path::Cubic {
                first: P::new(105.909, 88.0),
                second: P::new(88.0, 105.909),
                end: P::new(88.0, 128.0),
            },
            Path::Cubic {
                first: P::new(88.0, 132.418),
                second: P::new(91.582, 136.0),
                end: P::new(96.0, 136.0),
            },
            Path::Cubic {
                first: P::new(100.418, 136.0),
                second: P::new(104.0, 132.418),
                end: P::new(104.0, 128.0),
            },
            Path::Cubic {
                first: P::new(104.0, 114.745),
                second: P::new(114.745, 104.0),
                end: P::new(128.0, 104.0),
            },
            Path::Cubic {
                first: P::new(141.255, 104.0),
                second: P::new(152.0, 114.745),
                end: P::new(152.0, 128.0),
            },
            Path::Cubic {
                first: P::new(152.083, 159.81),
                second: P::new(145.076, 191.238),
                end: P::new(131.49, 220.0),
            },
            Path::Cubic {
                first: P::new(129.604, 224.007),
                second: P::new(131.323, 228.784),
                end: P::new(135.33, 230.67),
            },
            Path::Cubic {
                first: P::new(139.337, 232.556),
                second: P::new(144.114, 230.837),
                end: P::new(146.0, 226.83),
            },
            Path::Cubic {
                first: P::new(160.583, 195.93),
                second: P::new(168.099, 162.169),
                end: P::new(168.0, 128.0),
            },
            Path::Cubic {
                first: P::new(168.0, 105.909),
                second: P::new(150.091, 88.0),
                end: P::new(128.0, 88.0),
            },
            Path::Close,
            Path::Move(P::new(128.0, 24.0)),
            Path::Cubic {
                first: P::new(70.588, 24.061),
                second: P::new(24.061, 70.588),
                end: P::new(24.0, 128.0),
            },
            Path::Cubic {
                first: P::new(24.015, 137.99),
                second: P::new(22.324, 147.909),
                end: P::new(19.0, 157.33),
            },
            Path::Cubic {
                first: P::new(17.528, 161.497),
                second: P::new(19.713, 166.068),
                end: P::new(23.88, 167.54),
            },
            Path::Cubic {
                first: P::new(28.047, 169.012),
                second: P::new(32.618, 166.827),
                end: P::new(34.09, 162.66),
            },
            Path::Cubic {
                first: P::new(38.015, 151.526),
                second: P::new(40.013, 139.805),
                end: P::new(40.0, 128.0),
            },
            Path::Cubic {
                first: P::new(40.0, 79.399),
                second: P::new(79.399, 40.0),
                end: P::new(128.0, 40.0),
            },
            Path::Cubic {
                first: P::new(176.601, 40.0),
                second: P::new(216.0, 79.399),
                end: P::new(216.0, 128.0),
            },
            Path::Cubic {
                first: P::new(216.004, 146.278),
                second: P::new(214.232, 164.514),
                end: P::new(210.71, 182.45),
            },
            Path::Cubic {
                first: P::new(209.853, 186.786),
                second: P::new(212.674, 190.995),
                end: P::new(217.01, 191.85),
            },
            Path::Cubic {
                first: P::new(217.521, 191.949),
                second: P::new(218.04, 191.999),
                end: P::new(218.56, 192.0),
            },
            Path::Cubic {
                first: P::new(222.378, 191.996),
                second: P::new(225.66, 189.295),
                end: P::new(226.4, 185.55),
            },
            Path::Cubic {
                first: P::new(230.125, 166.593),
                second: P::new(232.001, 147.319),
                end: P::new(232.0, 128.0),
            },
            Path::Cubic {
                first: P::new(231.934, 70.59),
                second: P::new(185.41, 24.066),
                end: P::new(128.0, 24.0),
            },
            Path::Close,
            Path::Move(P::new(94.4, 152.17)),
            Path::Cubic {
                first: P::new(90.08, 151.311),
                second: P::new(85.879, 154.104),
                end: P::new(85.0, 158.42),
            },
            Path::Cubic {
                first: P::new(81.754, 174.396),
                second: P::new(75.942, 189.742),
                end: P::new(67.79, 203.86),
            },
            Path::Cubic {
                first: P::new(65.581, 207.687),
                second: P::new(66.893, 212.581),
                end: P::new(70.72, 214.79),
            },
            Path::Cubic {
                first: P::new(74.547, 216.999),
                second: P::new(79.441, 215.687),
                end: P::new(81.65, 211.86),
            },
            Path::Cubic {
                first: P::new(90.659, 196.248),
                second: P::new(97.076, 179.277),
                end: P::new(100.65, 161.61),
            },
            Path::Cubic {
                first: P::new(101.075, 159.529),
                second: P::new(100.654, 157.365),
                end: P::new(99.482, 155.594),
            },
            Path::Cubic {
                first: P::new(98.31, 153.824),
                second: P::new(96.481, 152.592),
                end: P::new(94.4, 152.17),
            },
            Path::Close,
            Path::Move(P::new(128.0, 56.0)),
            Path::Cubic {
                first: P::new(124.991, 56.001),
                second: P::new(121.986, 56.188),
                end: P::new(119.0, 56.56),
            },
            Path::Cubic {
                first: P::new(114.704, 57.205),
                second: P::new(111.707, 61.161),
                end: P::new(112.25, 65.472),
            },
            Path::Cubic {
                first: P::new(112.794, 69.782),
                second: P::new(116.678, 72.871),
                end: P::new(121.0, 72.43),
            },
            Path::Cubic {
                first: P::new(136.942, 70.448),
                second: P::new(152.969, 75.397),
                end: P::new(165.017, 86.024),
            },
            Path::Cubic {
                first: P::new(177.065, 96.651),
                second: P::new(183.977, 111.935),
                end: P::new(184.0, 128.0),
            },
            Path::Cubic {
                first: P::new(183.998, 138.363),
                second: P::new(183.356, 148.716),
                end: P::new(182.08, 159.0),
            },
            Path::Cubic {
                first: P::new(181.797, 161.112),
                second: P::new(182.368, 163.251),
                end: P::new(183.667, 164.94),
            },
            Path::Cubic {
                first: P::new(184.966, 166.63),
                second: P::new(186.886, 167.731),
                end: P::new(189.0, 168.0),
            },
            Path::Cubic {
                first: P::new(189.332, 168.04),
                second: P::new(189.666, 168.06),
                end: P::new(190.0, 168.06),
            },
            Path::Cubic {
                first: P::new(194.025, 168.051),
                second: P::new(197.417, 165.053),
                end: P::new(197.92, 161.06),
            },
            Path::Cubic {
                first: P::new(199.269, 150.112),
                second: P::new(199.937, 139.091),
                end: P::new(199.92, 128.06),
            },
            Path::Cubic {
                first: P::new(199.909, 88.322),
                second: P::new(167.738, 56.088),
                end: P::new(128.0, 56.0),
            },
            Path::Close,
            Path::Move(P::new(185.93, 184.25)),
            Path::Cubic {
                first: P::new(181.649, 183.143),
                second: P::new(177.283, 185.718),
                end: P::new(176.18, 190.0),
            },
            Path::Cubic {
                first: P::new(174.72, 195.69),
                second: P::new(173.03, 201.4),
                end: P::new(171.18, 207.0),
            },
            Path::Cubic {
                first: P::new(169.771, 211.178),
                second: P::new(172.007, 215.708),
                end: P::new(176.18, 217.13),
            },
            Path::Cubic {
                first: P::new(177.001, 217.409),
                second: P::new(177.863, 217.551),
                end: P::new(178.73, 217.55),
            },
            Path::Cubic {
                first: P::new(182.167, 217.547),
                second: P::new(185.219, 215.349),
                end: P::new(186.31, 212.09),
            },
            Path::Cubic {
                first: P::new(188.31, 206.17),
                second: P::new(190.1, 200.09),
                end: P::new(191.66, 194.04),
            },
            Path::Cubic {
                first: P::new(192.202, 191.984),
                second: P::new(191.903, 189.798),
                end: P::new(190.83, 187.963),
            },
            Path::Cubic {
                first: P::new(189.756, 186.128),
                second: P::new(187.997, 184.796),
                end: P::new(185.94, 184.26),
            },
            Path::Line(P::new(185.93, 184.25)),
            Path::Close,
        ],
        ..Default::default()
    }
}
