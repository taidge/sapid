use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill(
        [1., 2., 3., 4., 5.]
            .into_iter()
            .map(|px| {
                sapid::separator()
                    .theme(theme.clone())
                    .name(format!("{px} pixel divider"))
                    .thickness(px)
            })
            .collect::<Vec<_>>(),
    )
}
