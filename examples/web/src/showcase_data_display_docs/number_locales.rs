use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            sapid::stack().theme(theme.clone()).gap(0.).fill((
                sapid::group().theme(theme.clone()).gap(4.).fill((
                    sapid::text()
                        .theme(theme.clone())
                        .content("With default separator:"),
                    sapid::number_formatter()
                        .theme(theme.clone())
                        .value(1_000_000.)
                        .grouping(true),
                )),
                sapid::group().theme(theme.clone()).gap(4.).fill((
                    sapid::text()
                        .theme(theme.clone())
                        .content("With custom separator:"),
                    sapid::number_formatter()
                        .theme(theme.clone())
                        .value(1_000_000.)
                        .grouping(true)
                        .locale(sapid::NumberLocale::DeDe),
                )),
            )),
        )
}
