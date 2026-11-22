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
            sapid::checkbox_group("maximum-frameworks")
                .theme(theme.clone())
                .label("Select up to two frameworks")
                .max_values(2)
                .items(["React", "Svelte", "Angular", "Vue"].map(|v| sapid::checkbox_item(v, v))),
        )
}
