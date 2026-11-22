use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(true);
        let draft = scope.cage("Personal workspace".to_owned());
        let field_theme = theme.clone();

        let animate = scope.cage(true);
        let preview = {
            let theme = theme.clone();

            sapid::stack().theme(theme.clone()).gap(16.).fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Loading workspace card")
                    .checked(loading)
                    .on_checked_change(move |v| loading.revise(|old| *old = v)),
                sapid::skeleton()
                    .theme(theme.clone())
                    .name("Workspace card")
                    .loading(loading)
                    .animated(animate)
                    .fill(
                        sapid::field("skeleton-workspace-field")
                            .theme(theme.clone())
                            .label("Workspace name")
                            .description("Your draft stays mounted while the placeholder is shown.")
                            .control(move |field| {
                                sapid::text_field()
                                    .theme(field_theme.clone())
                                    .field(field)
                                    .value(draft)
                                    .on_value_change(move |v| draft.revise(|old| *old = v))
                            }),
                    ),
            ))
        };
        view()
            .class("site-configurator site-display-configurator site-skeleton-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::switch("Animate", animate, &theme)],
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
                .size(Size::Small)
                .with_thumb_indicator(true)
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }
}
