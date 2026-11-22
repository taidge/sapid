use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::typography().theme(theme.clone()).content(move |cx| {
        view()
            .style(
                Style::new()
                    .display(glory::style::Display::Flex)
                    .flex_direction(glory::style::FlexDirection::Column)
                    .gap(12.),
            )
            .fill(
                (1..=6)
                    .map(|n| cx.title(n).content(format!("Heading {n}")))
                    .collect::<Vec<_>>(),
            )
            .fill((
                cx.text().content(
                    "A paragraph of text can contain links, emphasis, and other inline elements.",
                ),
                cx.anchor().label("Example link").href("#typography-styles"),
                sapid::text_list().theme(cx.theme.clone()).items(
                    ["First item", "Second item", "Third item"]
                        .into_iter()
                        .map(|s| sapid::text_list_item().fill(cx.text().content(s))),
                ),
                sapid::blockquote().theme(cx.theme.clone()).fill(
                    cx.text()
                        .content("A blockquote stands apart from the surrounding paragraph."),
                ),
                cx.title(2).content("Code"),
                sapid::text()
                    .theme(cx.theme.clone())
                    .content("let example = true;"),
            ))
    })
}
