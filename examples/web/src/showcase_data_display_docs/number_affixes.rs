use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            sapid::stack().theme(theme.clone()).gap(0.).fill((
                sapid::group().theme(theme.clone()).gap(4.).fill((
                    sapid::text().theme(theme.clone()).content("With prefix:"),
                    sapid::number_formatter()
                        .theme(theme.clone())
                        .value(100.)
                        .prefix("$ "),
                )),
                sapid::group().theme(theme.clone()).gap(4.).fill((
                    sapid::text().theme(theme.clone()).content("With suffix:"),
                    sapid::number_formatter()
                        .theme(theme.clone())
                        .value(100.)
                        .suffix(" RUB"),
                )),
            )),
        )
}
