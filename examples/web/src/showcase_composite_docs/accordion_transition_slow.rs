use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let accordion = sapid::accordion("fruit-transition-slow")
        .theme(theme.clone())
        .transition_duration(500);
    fruit_preview(theme, accordion, None, 380.)
}

const FRUIT: [(&str, &str, &str, &str); 3] = [
    (
        "apples",
        "🍎",
        "Apples",
        "Apples are crisp fruit that range from tart to sweet. Eat them fresh, bake them into pies, or slice them into salads. Several thousand varieties grow around the world.",
    ),
    (
        "bananas",
        "🍌",
        "Bananas",
        "Bananas have sweet flesh beneath a bright yellow peel. They are a convenient snack, blend well in smoothies, and soften as they ripen.",
    ),
    (
        "broccoli",
        "🥦",
        "Broccoli",
        "Broccoli has green florets and firm, edible stems. Steam it, roast it, or add it to a stir-fry to preserve its texture and color.",
    ),
];
fn fruit_items(mut accordion: sapid::Accordion, disabled: Option<&str>) -> sapid::Accordion {
    for (key, glyph, label, description) in FRUIT {
        accordion = accordion.item(
            sapid::accordion_item(key)
                .label(label)
                .enabled(disabled != Some(key))
                .trigger_content(move || text().content(format!("{glyph}　{label}")))
                .content(move || text().content(description)),
        );
    }
    accordion
}
fn fruit_preview(
    theme: sapid::Theme,
    accordion: sapid::Accordion,
    disabled: Option<&str>,
    width: f64,
) -> impl Widget {
    sapid::group().theme(theme).justify(Justify::Center).fill(
        fruit_items(accordion, disabled).style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(width)),
        ),
    )
}
