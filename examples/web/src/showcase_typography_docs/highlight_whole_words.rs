use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [true, false]
            .map(|whole| {
                sapid::stack().theme(theme.clone()).gap(4.).fill((
                    sapid::text()
                        .theme(theme.clone())
                        .weight(600.)
                        .content(if whole {
                            "With whole word matching (wholeWords={true})"
                        } else {
                            "Without whole word matching (default)"
                        }),
                    view().fill(
                        sapid::highlight()
                            .theme(theme.clone())
                            .content("The theme is there")
                            .query("the")
                            .ignore_ascii_case(true)
                            .whole_words(whole),
                    ),
                ))
            })
            .into_iter()
            .collect::<Vec<_>>(),
    )
}
