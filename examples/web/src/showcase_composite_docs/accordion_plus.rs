use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let mut accordion = sapid::accordion("fruit-disclosures")
        .theme(theme.clone())
        .with_chevron(false)
        .default_open(["apples"]);
    for (id, label, body) in [
        (
            "apples",
            "Apples",
            "Apples are crisp fruit with red, green and yellow varieties.",
        ),
        (
            "bananas",
            "Bananas",
            "Bananas have a soft texture and a sweet flavor.",
        ),
        ("broccoli", "Broccoli", "Broccoli has edible green florets."),
    ] {
        accordion = accordion.item(
            sapid::accordion_item(id)
                .label(label)
                .trigger_content(move || {
                    glory::ui::group().fill((
                        text().content(format!(
                            "{}    {label}",
                            match id {
                                "apples" => "🍎",
                                "bananas" => "🍌",
                                _ => "🥦",
                            }
                        )),
                        text().class("sapid-demo-accordion-plus").content("+"),
                    ))
                })
                .content(move || text().content(body)),
        );
    }
    sapid::group()
        .theme(theme)
        .justify(Justify::Center)
        .fill(accordion.class("sapid-demo-plus-fruits"))
}
