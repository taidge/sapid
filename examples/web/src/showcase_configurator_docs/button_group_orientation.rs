use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let axis = scope.cage("horizontal".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![axis.get()]))
            .key(Clone::clone)
            .children(move |entry| {
                sapid::button_group()
                    .theme(palette.clone())
                    .name("Example group")
                    .orientation(if entry.value.get() == "vertical" {
                        glory::render::Orientation::Vertical
                    } else {
                        glory::render::Orientation::Horizontal
                    })
                    .button(
                        sapid::button()
                            .variant(sapid::ButtonVariant::Secondary)
                            .label("First"),
                    )
                    .button(
                        sapid::button()
                            .variant(sapid::ButtonVariant::Secondary)
                            .label("Second"),
                    )
                    .button(
                        sapid::button()
                            .variant(sapid::ButtonVariant::Secondary)
                            .label("Third"),
                    )
            });
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

    pub fn segmented_choice(
        key: &'static str,
        label: &'static str,
        value: Cage<String>,
        options: &'static [(&str, &str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        Box::new(
            sapid::segmented_control(key)
                .theme(theme.clone())
                .size(Size::Small)
                .label(label)
                .full_width(true)
                .value(value)
                .segments(
                    options
                        .iter()
                        .map(|&(key, caption)| sapid::segment(key).label(caption)),
                )
                .on_value_change(move |next| value.revise(|current| *current = next)),
        )
    }
}
