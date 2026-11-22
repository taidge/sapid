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
            sapid::radio_group("disabled-framework")
                .theme(theme.clone())
                .enabled(false)
                .default_value("React")
                .items(["React", "Svelte", "Angular", "Vue"].map(|v| sapid::radio(v).label(v))),
        )
}
