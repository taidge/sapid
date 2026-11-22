use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let content = theme.clone();
    sapid::collapsible("nested-outer")
        .theme(theme)
        .class("site-reference-collapse site-collapse-nested")
        .label("Root collapse")
        .height_transition(200)
        .content(move || {
            let inside = content.clone();
            sapid::stack().theme(content.clone()).gap(12.).fill((
                sapid::text()
                    .theme(content.clone())
                    .weight(700.)
                    .content("This collapse contains another collapse"),
                collapse_paragraph(content.clone()),
                sapid::collapsible("nested-inner")
                    .theme(content.clone())
                    .class("site-reference-collapse site-collapse-inner")
                    .label("Inner collapse")
                    .height_transition(200)
                    .content(move || {
                        sapid::stack().theme(inside.clone()).gap(12.).fill((
                            sapid::text()
                                .theme(inside.clone())
                                .weight(700.)
                                .content("This collapse is inside another collapse"),
                            collapse_paragraph(inside.clone()),
                        ))
                    }),
            ))
        })
}

fn collapse_paragraph(theme: sapid::Theme) -> impl Widget {
    sapid::text().theme(theme).content("Bulbasaur is a small Pokémon with a plant bulb on its back. The plant grows with its host, storing energy from sunlight. Two slender vines beneath the bulb help it grasp nearby objects.")
}
