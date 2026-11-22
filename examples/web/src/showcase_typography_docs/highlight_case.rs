use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill(
        [true, false]
            .map(|ignore| {
                sapid::stack().theme(theme.clone()).gap(4.).fill((
                    sapid::text()
                        .theme(theme.clone())
                        .weight(600.)
                        .content(if ignore {
                            "With case-insensitive matching (default)"
                        } else {
                            "With case-sensitive matching"
                        }),
                    view().fill(
                        sapid::highlight()
                            .theme(theme.clone())
                            .content("Highlight This, definitely THIS and also this!")
                            .query("this")
                            .ignore_ascii_case(ignore),
                    ),
                ))
            })
            .into_iter()
            .collect::<Vec<_>>(),
    )
}
