use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(0.).fill(
        (1..=6)
            .map(|level| {
                sapid::title(level)
                    .theme(theme.clone())
                    .weight(700.)
                    .content(format!("Heading level {level}"))
            })
            .collect::<Vec<_>>(),
    )
}
