use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let wrap = scope.cage(Some("wrap".to_owned()));
        let preview = {
            let theme = theme.clone();

            sapid::text().theme(theme.clone()).class(wrap.map(|v| if v.as_deref()==Some("balance") {"sapid-text-balanced".to_owned()}else{"".to_owned()})).style(Style::new().max_width(Limit::Px(320.))).content("A thoughtful review connects decisions to the people using the project. Give the team time to explore alternatives, test assumptions and clarify the details together.")
        };
        view()
            .class("site-configurator site-display-configurator site-text-wrap-play-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::option_segments(
                    "text_wrap_play-wrap",
                    "Text wrap",
                    wrap,
                    &[("wrap", "Wrap"), ("balance", "Balance")],
                    &theme,
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

    pub fn option_segments(
        key: &'static str,
        label: &'static str,
        value: Cage<Option<String>>,
        options: &'static [(&str, &str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::stack().theme(theme.clone()).gap(4.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(label),
                sapid::segmented_control(key)
                    .theme(theme.clone())
                    .size(Size::Small)
                    .name(label)
                    .full_width(true)
                    .value(value.map(|selected| selected.clone().unwrap_or_default()))
                    .segments(
                        options
                            .iter()
                            .map(|&(key, caption)| sapid::segment(key).label(caption)),
                    )
                    .on_value_change(move |next| value.revise(|current| *current = Some(next))),
            )),
        )
    }
}
