use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let justify = scope.cage("center".to_owned());
        let row_style = justify.map(|v| Style {
            justify: Some(controls::layout_justify(v)),
            ..Style::new().width(Length::Percent(100.))
        });
        let button = || {
            sapid::button()
                .theme(theme.clone())
                .variant(sapid::ButtonVariant::Secondary)
                .label("Button label")
                .style(row_style)
        };
        let icon = || {
            use glory::render::{GraphicPoint as P, PathCommand as Path};
            let mut image = sapid::Icon::Check.drawing(theme.foreground);
            image.commands = vec![
                Path::Move(P::new(2., 4.)),
                Path::Line(P::new(22., 4.)),
                Path::Line(P::new(22., 20.)),
                Path::Line(P::new(2., 20.)),
                Path::Close,
                Path::Move(P::new(2., 16.)),
                Path::Line(P::new(8., 10.)),
                Path::Line(P::new(18., 20.)),
                Path::Move(P::new(14., 16.)),
                Path::Line(P::new(17., 13.)),
                Path::Line(P::new(22., 18.)),
                Path::Move(P::new(15., 8.)),
                Path::Line(P::new(16., 8.)),
            ];
            glory::ui::graphic()
                .drawing(image)
                .style(Style::new().width(Length::Px(14.)).height(Length::Px(14.)))
        };
        let preview = sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .style(Style::new().width(Length::Percent(100.)))
            .fill((
                button().left_section(icon()).right_section(icon()),
                button().left_section(icon()),
                button().right_section(icon()),
                button()
                    .left_section(text().content(""))
                    .right_section(icon()),
            ));
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::segmented_choice(
                    "sections-position",
                    "Justify",
                    justify,
                    &[("center", "Center"), ("between", "Space-between")],
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

    pub fn layout_justify(value: &str) -> Justify {
        match value {
            "center" => Justify::Center,
            "end" => Justify::End,
            "between" => Justify::Between,
            "around" => Justify::Around,
            "evenly" => Justify::Evenly,
            _ => Justify::Start,
        }
    }
}
