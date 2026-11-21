use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(0);
        let compact = scope.cage(false);
        let loading = scope.cage(false);
        let preview = sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::button()
                .theme(theme.clone())
                .label("Filled button")
                .compact(compact)
                .loading(loading)
                .on_press(move || count.revise(|v| *v += 1)),
            sapid::text()
                .theme(theme.clone())
                .content(count.map(|v| format!("Button presses: {v}"))),
        ));
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Compact", compact, &theme),
                    controls::switch("Loading", loading, &theme),
                ],
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

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }
}
