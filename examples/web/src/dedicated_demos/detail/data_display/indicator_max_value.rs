use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .gap(16.)
        .fill(
            [(50, 99), (100, 99), (1000, 999)]
                .into_iter()
                .enumerate()
                .map(|(i, value)| {
                    sapid::indicator()
                        .theme(theme.clone())
                        .count(value.0)
                        .max_value(value.1)
                        .diameter(16.)
                        .with_border(false)
                        .fill(display_sample::portrait(theme.clone(), i % 2 == 1, true))
                })
                .collect::<Vec<_>>(),
        )
}
