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
            sapid::field("framework-children")
                .theme(theme)
                .label("With children options")
                .control(|field| {
                    sapid::native_select("framework-child-options")
                        .field(field)
                        .group(
                            sapid::select_group("Frontend libraries")
                                .option(sapid::select_option("react", "React"))
                                .option(sapid::select_option("angular", "Angular"))
                                .option(sapid::select_option("vue", "Vue").disabled(true)),
                        )
                        .group(
                            sapid::select_group("Backend libraries").options(
                                ["Express", "Koa", "Django"]
                                    .map(|name| sapid::select_option(name.to_lowercase(), name)),
                            ),
                        )
                        .default_value(Some("react".into()))
                }),
        )
}
