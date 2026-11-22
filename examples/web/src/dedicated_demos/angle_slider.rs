use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let angle = scope.cage(0.);
        let diameter = scope.cage(60.);
        let thumb = scope.cage(3.);
        let label = scope.cage(true);
        let preview = view()
            .style(diameter.map(|px| {
                Style::new()
                    .position(Position::Relative)
                    .width(Length::Px(*px))
                    .height(Length::Px(*px))
            }))
            .fill((
                sapid::angle_slider("rotation-angle")
                    .theme(theme.clone())
                    .name("Rotation angle")
                    .thumb_size(thumb)
                    .style(
                        diameter
                            .map(|px| Style::new().width(Length::Px(*px)).height(Length::Px(*px))),
                    )
                    .value(angle)
                    .on_value_change(move |next| angle.revise(|v| *v = next)),
                when(label, move || {
                    text()
                        .class("site-angle-value")
                        .style(Style {
                            display: Some(Display::Flex),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .position(Position::Absolute)
                                .left(Length::Px(0.))
                                .top(Length::Px(0.))
                                .width(Length::Percent(100.))
                                .height(Length::Percent(100.))
                                .pointer_events(glory::style::PointerEvents::None)
                        })
                        .content(angle.map(|v| format!("{v:.0}")))
                }),
            ));
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::range("angle-size", "Size", diameter, 40., 240., 1., &theme),
                    controls::range("angle-thumb-size", "Thumb size", thumb, 1., 18., 1., &theme),
                    controls::switch("With label", label, &theme),
                ],
            ))
            .show_in(scope)
    })
}
