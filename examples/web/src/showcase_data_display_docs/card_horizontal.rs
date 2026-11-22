use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::card().theme(theme.clone()).bordered(true).fill(
        sapid::group().theme(theme.clone()).gap(24.).fill((
            sapid::ring_progress(81.)
                .theme(theme.clone())
                .name("Completed tasks")
                .size(100.)
                .thickness(7.)
                .label("81%"),
            sapid::stack().theme(theme.clone()).gap(4.).fill((
                sapid::title(3)
                    .theme(theme.clone())
                    .content("Project tasks"),
                sapid::text()
                    .theme(theme.clone())
                    .content("Completed  1,887"),
                sapid::text().theme(theme).content("Remaining  447"),
            )),
        )),
    )
}
