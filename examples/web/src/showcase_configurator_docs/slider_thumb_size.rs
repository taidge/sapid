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

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(240.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }
}
