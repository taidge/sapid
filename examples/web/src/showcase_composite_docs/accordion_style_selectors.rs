use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = {
        let mut accordion = sapid::accordion("style-accordion")
            .theme(theme.clone())
            .default_open(["apple"]);
        for (id, name, emoji, desc) in [
            ("apple", "Apples", "🍎", "Crisp and refreshing fruit."),
            ("banana", "Bananas", "🍌", "Soft tropical fruit."),
            ("broccoli", "Broccoli", "🥦", "A green vegetable."),
        ] {
            accordion = accordion.item(
                sapid::accordion_item(id)
                    .label(name)
                    .trigger_content(move || {
                        glory::ui::group().fill((
                            text().class("style-fruit-icon").content(emoji),
                            text().class("style-fruit-label").content(name),
                        ))
                    })
                    .content(move || text().class("style-fruit-content").content(desc)),
            );
        }
        accordion
    };
    crate::showcase_style_explorer::explorer(
        "accordion-styles",
        Box::new(preview),
        &[
            ("root", "Root element", ".sapid-accordion"),
            (
                "item",
                "`Accordion.Item` root element",
                ".sapid-accordion-item",
            ),
            (
                "control",
                "`Accordion.Control` root element",
                ".sapid-accordion-trigger",
            ),
            (
                "chevron",
                "`Accordion.Control` chevron container element",
                ".sapid-disclosure-chevron",
            ),
            ("label", "`Accordion.Control` label", ".style-fruit-label"),
            ("icon", "`Accordion.Control` icon", ".style-fruit-icon"),
            (
                "itemTitle",
                "`Accordion.Control` title (h2-h6) tag",
                ".sapid-accordion-item > :first-child",
            ),
            (
                "panel",
                "`Accordion.Panel` root element",
                ".sapid-accordion-panel",
            ),
            (
                "content",
                "Wrapper element of `Accordion.Panel` `children`",
                ".style-fruit-content",
            ),
        ],
    )
}
