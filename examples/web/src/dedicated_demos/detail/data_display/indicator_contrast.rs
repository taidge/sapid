use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .gap(16.)
        .fill(
            [false, true, false, true]
                .into_iter()
                .enumerate()
                .map(|(i, value)| {
                    sapid::indicator()
                        .theme(theme.clone())
                        .count(99)
                        .color(if i < 2 {
                            Color(192, 235, 117, 255)
                        } else {
                            Color(11, 114, 133, 255)
                        })
                        .auto_contrast(value)
                        .diameter(16.)
                        .with_border(false)
                        .fill(display_sample::portrait(theme.clone(), i % 2 == 1, true))
                })
                .collect::<Vec<_>>(),
        )
}
