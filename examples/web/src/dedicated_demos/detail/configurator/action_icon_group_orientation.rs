use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let axis = scope.cage("horizontal".to_owned());
        let preview = sapid::flex()
            .theme(theme.clone())
            .gap(0.)
            .style(axis.map(|v| {
                Style::new().flex_direction(if v == "vertical" {
                    FlexDirection::Column
                } else {
                    FlexDirection::Row
                })
            }))
            .fill(Vec::from(
                [
                    (sapid::Icon::Check, "Accept"),
                    (sapid::Icon::Info, "Information"),
                    (sapid::Icon::Close, "Dismiss"),
                ]
                .map(|(icon, name)| {
                    sapid::action_icon(name).theme(theme.clone()).icon(
                        glory::ui::graphic()
                            .drawing(icon.drawing(theme.foreground))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                    )
                }),
            ));
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::segmented_choice(
                    "group-orientation",
                    "Orientation",
                    axis,
                    &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                    &theme,
                )],
            ));
        panel.show_in(scope)
    })
}
