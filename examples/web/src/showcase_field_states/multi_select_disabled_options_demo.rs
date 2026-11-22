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
            sapid::field("multi-disabled-options-field")
                .theme(theme.clone())
                .label("Your favorite libraries")
                .control(move |field| {
                    sapid::multi_select("multi-disabled-options")
                        .placeholder("Pick value")
                        .options([
                            sapid::select_option("react", "React"),
                            sapid::select_option("angular", "Angular"),
                            sapid::select_option("vue", "Vue").disabled(true),
                            sapid::select_option("svelte", "Svelte").disabled(true),
                        ])
                        .field(field)
                }),
        )
}
