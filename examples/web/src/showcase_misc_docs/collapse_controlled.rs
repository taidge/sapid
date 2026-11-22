use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let content = theme.clone();
        sapid::collapsible("collapse-hook")
            .theme(theme.clone())
            .class("site-reference-collapse site-collapse-hook")
            .label(open.map(|v| {
                if *v {
                    String::from("Collapse")
                } else {
                    String::from("Expand")
                }
            }))
            .open(open)
            .on_open_change(move |proposed| open.revise(|current| *current = proposed))
            .height_transition(200)
            .content(move || collapse_paragraph(content.clone()))
            .show_in(scope)
    })
}

fn collapse_paragraph(theme: sapid::Theme) -> impl Widget {
    sapid::text().theme(theme).content("Bulbasaur is a small Pokémon with a plant bulb on its back. The plant grows with its host, storing energy from sunlight. Two slender vines beneath the bulb help it grasp nearby objects.")
}
