use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            sapid::number_formatter()
                .theme(theme.clone())
                .value(5. / 3.)
                .max_fraction_digits(2),
        )
}
