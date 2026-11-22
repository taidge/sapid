use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("autocomplete_disabled-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Your favorite library")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .enabled(false)
        .control(move |field| {
            sapid::autocomplete("autocomplete_disabled-example")
                .options(["React", "Angular", "Svelte", "Vue"].map(|v| sapid::select_option(v, v)))
                .field(field)
                .placeholder("Your favorite library")
        })
}
