use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("select_readonly-example")
        .theme(theme.clone())
        .class("site-field-state-example")
        .label("Your favorite library")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .readonly(true)
        .control(move |field| {
            sapid::select("select_readonly-example")
                .options(["React", "Angular", "Svelte", "Vue"].map(|v| sapid::select_option(v, v)))
                .field(field)
                .placeholder("Your favorite library")
                .default_value(Some("React".to_owned()))
        })
}
