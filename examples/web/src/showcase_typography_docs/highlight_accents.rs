use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(12.).fill(
        [false, true]
            .into_iter()
            .map(|ignore| {
                view().fill(
                    sapid::highlight()
                        .theme(theme.clone())
                        .content("We visited café and cafe, then cafe\u{301}.")
                        .query("cafe")
                        .ignore_accents(ignore),
                )
            })
            .collect::<Vec<_>>(),
    )
}
