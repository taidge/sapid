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
            sapid::checkbox_group("disabled-frameworks")
                .theme(theme.clone())
                .label("Select your favorite frameworks/libraries")
                .enabled(false)
                .default_selected(["React".to_owned()])
                .items(["React", "Svelte", "Angular", "Vue"].map(|v| sapid::checkbox_item(v, v))),
        )
}
