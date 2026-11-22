use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let thumb = scope.cage(14.);
        let preview = sapid::slider("configured-thumb")
            .theme(theme.clone())
            .name("Value")
            .value_text(Some("Slider value".to_owned()))
            .default_value(20.)
            .thumb_size(thumb)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(400.)),
            );
        view()
            .class("site-configurator site-field-configurator site-slider-thumb-panel")
            .fill(controls::playground(
                Box::new(preview),
                vec![Box::new(
                    sapid::stack().class("site-thumb-setting").gap(3.).fill((
                        sapid::text()
                            .class("site-control-label")
                            .content("Thumb size"),
                        sapid::slider("slider-config-thumb-size")
                            .theme(theme.clone())
                            .name("Thumb size")
                            .min(16.)
                            .max(32.)
                            .value(thumb.map(|v| v.max(16.)))
                            .on_value_change(move |next| thumb.revise(|current| *current = next)),
                    )),
                )],
            ))
            .show_in(scope)
    })
}
