use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let axis = scope.cage("horizontal".to_owned());
        let pane_theme = theme.clone();
        let preview = sapid::splitter("workspace-splitter")
            .theme(theme.clone())
            .name("Resize editor and preview")
            .class("site-splitter-colors")
            .axis(axis.map(|v| {
                if v == "vertical" {
                    glory::render::Orientation::Vertical
                } else {
                    glory::render::Orientation::Horizontal
                }
            }))
            .grip_size(52., 10.)
            .default_value(50.)
            .min(20.)
            .max(80.)
            .style(Style::new().height(Length::Px(200.)))
            .first({
                let theme = theme.clone();
                move || {
                    view().class("site-splitter-first").fill(
                        sapid::text_field()
                            .theme(theme.clone())
                            .name("Editor draft")
                            .placeholder("First pane")
                            .class("site-splitter-draft"),
                    )
                }
            })
            .second(move || {
                view().class("site-splitter-second").fill(
                    sapid::text()
                        .theme(pane_theme.clone())
                        .content("Second pane"),
                )
            });
        view()
            .class("site-configurator site-layout-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::segmented_choice(
                    "splitter-direction",
                    "Orientation",
                    axis,
                    &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
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
