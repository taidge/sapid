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
            view().class("site-dd-scroll site-combo-scroll-area").fill(
                sapid::field("combo-area-field")
                    .theme(theme.clone())
                    .label("Pick value")
                    .control(move |field| {
                        sapid::combobox("combo-area")
                            .placeholder("Pick value")
                            .options(
                                [
                                    "🍎 Apples",
                                    "🍌 Bananas",
                                    "🥦 Broccoli",
                                    "🥕 Carrots",
                                    "🍫 Chocolate",
                                    "🍇 Grapes",
                                    "🍋 Lemon",
                                    "🥬 Lettuce",
                                    "🍄 Mushrooms",
                                    "🍊 Oranges",
                                    "🥔 Potatoes",
                                    "🍅 Tomatoes",
                                    "🥚 Eggs",
                                    "🥛 Milk",
                                    "🍞 Bread",
                                    "🍗 Chicken",
                                    "🍔 Hamburger",
                                    "🧀 Cheese",
                                    "🥩 Steak",
                                    "🍟 French Fries",
                                    "🍕 Pizza",
                                    "🥦 Cauliflower",
                                    "🥜 Peanuts",
                                    "🍦 Ice Cream",
                                    "🍯 Honey",
                                    "🥖 Baguette",
                                    "🍣 Sushi",
                                    "🥝 Kiwi",
                                    "🍓 Strawberries",
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
                            .field(field)
                    }),
            ),
        )
}
