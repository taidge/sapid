use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .gap(16.)
        .fill(
            [true, false]
                .into_iter()
                .enumerate()
                .map(|(i, value)| {
                    sapid::indicator()
                        .theme(theme.clone())
                        .count(0)
                        .show_zero(value)
                        .diameter(16.)
                        .with_border(false)
                        .fill(display_sample::portrait(theme.clone(), i % 2 == 1, true))
                })
                .collect::<Vec<_>>(),
        )
}
