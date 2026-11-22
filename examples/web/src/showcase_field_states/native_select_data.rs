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
            sapid::native_select("framework-data")
                .theme(theme)
                .label("Libraries")
                .group(
                    sapid::select_group("Frontend libraries").options(
                        [
                            ("react", "React", false),
                            ("angular", "Angular", false),
                            ("vue", "Vue", true),
                        ]
                        .map(|(value, label, disabled)| {
                            sapid::select_option(value, label).disabled(disabled)
                        }),
                    ),
                )
                .group(
                    sapid::select_group("Backend libraries").options(
                        ["Express", "Koa", "Django"]
                            .map(|name| sapid::select_option(name.to_lowercase(), name)),
                    ),
                )
                .default_value(Some("react".into())),
        )
}
