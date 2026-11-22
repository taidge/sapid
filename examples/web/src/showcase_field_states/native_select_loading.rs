use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::field("framework-loading")
                .theme(theme)
                .label("Your favorite framework")
                .control(|field| {
                    sapid::native_select("framework-loading-select")
                        .field(field)
                        .options(
                            ["React", "Angular", "Vue", "Svelte"]
                                .map(|name| sapid::select_option(name, name)),
                        )
                        .default_value(Some("React".into()))
                        .loading(true)
                }),
        )
}
