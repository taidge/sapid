use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            view().class("").fill(
                sapid::field("combo-first-field")
                    .theme(theme.clone())
                    .label("Pick value")
                    .control(move |field| {
                        sapid::combobox("combo-first")
                            .placeholder("Pick value")
                            .options(
                                [
                                    "🍎 Apples",
                                    "🍌 Bananas",
                                    "🥦 Broccoli",
                                    "🥕 Carrots",
                                    "🍫 Chocolate",
                                ]
                                .map(|name| {
                                    sapid::select_option(
                                        name.split_whitespace()
                                            .last()
                                            .unwrap_or(name)
                                            .to_ascii_lowercase(),
                                        name,
                                    )
                                }),
                            )
                            .select_first_on_change(true)
                            .field(field)
                    }),
            ),
        )
}
