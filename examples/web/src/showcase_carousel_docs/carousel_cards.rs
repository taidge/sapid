use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let palette = theme.clone();
    sapid::carousel()
        .theme(theme)
        .name("Cards carousel")
        .count(4usize)
        .slide_fraction(0.65)
        .slide_gap(14.)
        .render_slide(move |index| {
            sapid::card()
                .theme(palette.clone())
                .title(
                    [
                        "Mountain trails",
                        "Design notes",
                        "Travel journal",
                        "New projects",
                    ][index],
                )
                .description("Explore this collection")
                .fill(
                    sapid::text()
                        .theme(palette.clone())
                        .content("Read the latest updates and ideas from the team."),
                )
        })
}
