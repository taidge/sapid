use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let focused = scope.cage(false);
        let palette = theme.clone();
        sapid::field("tooltip-input-field")
            .theme(theme.clone())
            .class("site-field-state-example")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(400.)),
            )
            .label("TextInput with tooltip")
            .description("Tooltip will be relative to the input")
            .control(move |field| {
                let mut field = Some(field);
                sapid::tooltip("input-container-tip")
                    .theme(palette.clone())
                    .text("Additional information")
                    .open(focused)
                    .placement(glory::render::LayerPlacement::Anchor(
                        glory::render::AnchorPlacement::new(
                            glory::render::AnchorSide::Top,
                            glory::render::AnchorAlign::Start,
                        ),
                    ))
                    .target(move |target| {
                        target.view(
                            view().fill(
                                view()
                                    .on(glory::render::EventKind::FocusWithin, move |e| {
                                        if let glory::render::Event::FocusWithin {
                                            focused: inside,
                                            ..
                                        } = e
                                        {
                                            focused.revise(|v| *v = inside);
                                        }
                                    })
                                    .fill(
                                        sapid::text_field()
                                            .field(field.take().expect("target is mounted once"))
                                            .placeholder("Focus me to see tooltip"),
                                    ),
                            ),
                        )
                    })
            })
            .show_in(scope)
    })
}
