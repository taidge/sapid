use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let content = theme.clone();
    sapid::collapsible("collapse-motion-example")
        .theme(theme)
        .class("site-reference-collapse site-collapse-center site-collapse-linear")
        .label("Toggle with linear transition")
        .height_transition(500)
        .content(move || collapse_paragraph(content.clone()))
}

fn collapse_paragraph(theme: sapid::Theme) -> impl Widget {
    sapid::text().theme(theme).content("Bulbasaur is a small Pokémon with a plant bulb on its back. The plant grows with its host, storing energy from sunlight. Two slender vines beneath the bulb help it grasp nearby objects.")
}
