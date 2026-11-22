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
        .fill(widget(move |scope| {
            let selected = scope.cage(Vec::<String>::new());
            let switches = ["React", "Svelte", "Angular", "Vue"].map(|name| {
                sapid::switch()
                    .theme(theme.clone())
                    .size(sapid::Size::Small)
                    .label(name)
                    .checked(selected.map(move |v| v.iter().any(|s| s == name)))
                    .enabled(selected.map(move |v| v.len() < 2 || v.iter().any(|s| s == name)))
                    .on_checked_change(move |checked| {
                        selected.revise(|values| {
                            values.retain(|v| v != name);
                            if checked && values.len() < 2 {
                                values.push(name.to_owned());
                            }
                        })
                    })
            });
            sapid::group()
                .theme(theme.clone())
                .fill(switches.into_iter().collect::<Vec<_>>())
                .show_in(scope)
        }))
}
