use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut accordion = sapid::accordion("styled-fruits")
        .theme(theme.clone())
        .class("sapid-demo-styled-fruits")
        .default_open(["apples"]);
    for (id, emoji, label, body) in [
        (
            "apples",
            "🍎",
            "Apples",
            "A crisp apple can be enjoyed fresh or baked. Varieties range from tart green apples to sweet red fruit.",
        ),
        (
            "bananas",
            "🍌",
            "Bananas",
            "Bananas have a sweet, soft flesh and a bright yellow peel.",
        ),
        (
            "broccoli",
            "🥦",
            "Broccoli",
            "Broccoli is a green vegetable with closely packed edible florets.",
        ),
    ] {
        accordion = accordion.item(
            sapid::accordion_item(id)
                .label(label)
                .trigger_content(move || text().content(format!("{emoji}    {label}")))
                .content(move || text().content(body)),
        );
    }
    sapid::group()
        .theme(theme)
        .justify(Justify::Center)
        .fill(accordion)
}
