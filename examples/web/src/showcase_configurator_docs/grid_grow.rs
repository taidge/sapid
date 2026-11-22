use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let grow = scope.cage(true);
        let gap = scope.cage(Some("md".to_owned()));
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![grow.get()]))
            .key(Clone::clone)
            .children(move |row| {
                let grow = row.value.get();
                let theme = palette.clone();
                let colors = theme.clone();
                sapid::grid("growing-grid")
                    .theme(theme.clone())
                    .columns(12)
                    .style(gap.map(move |v| {
                        Style::new()
                            .width(Length::Percent(100.))
                            .gap(colors.spacing(controls::size_value(v.as_deref().unwrap_or("md"))))
                    }))
                    .items((0..5).map(|i| {
                        sapid::grid_item(i.to_string())
                            .span(if i < 3 || !grow { 4 } else { 6 })
                            .fill(controls::grid_demo_cell((i + 1).to_string(), &theme))
                    }))
            });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Grow", grow, &theme),
                    controls::scale_slider("grow-grid-gap", "Gap", gap, &theme),
                ],
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

    pub fn grid_demo_cell(label: String, theme: &Theme) -> impl Widget {
        let (background, color) = if theme.scheme == sapid::ColorScheme::Dark {
            (theme.surface, theme.muted)
        } else {
            (
                glory::style::Color(231, 245, 255, 255),
                glory::style::Color(34, 139, 230, 255),
            )
        };
        view()
            .style(Style {
                justify: Some(Justify::Center),
                align: Some(Align::Center),
                background: Some(background),
                ..Style::new()
                    .display(Display::Flex)
                    .padding(glory::style::Edges::all(16.))
                    .height(Length::Percent(100.))
            })
            .fill(
                text()
                    .style(Style {
                        font_size: Some(20.),
                        color: Some(color),
                        ..Style::new().font_weight(700.)
                    })
                    .content(label),
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

    pub fn scale_slider(
        key: &'static str,
        label: &'static str,
        value: Cage<Option<String>>,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let current = value.map(|current| match current.as_deref() {
            Some("xs") => 0.0,
            Some("sm") => 1.0,
            Some("lg") => 3.0,
            Some("xl") => 4.0,
            _ => 2.0,
        });
        let value_text = value.map(|current| {
            Some(
                match current.as_deref() {
                    Some("xs") => "Extra small",
                    Some("sm") => "Small",
                    Some("lg") => "Large",
                    Some("xl") => "Extra large",
                    _ => "Medium",
                }
                .to_owned(),
            )
        });
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.0).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(label),
                sapid::slider(key)
                    .theme(theme.clone())
                    .name(label)
                    .min(0.0)
                    .max(4.0)
                    .step(1.0)
                    .value(current)
                    .value_text(value_text)
                    .marks((0..5).map(|i| sapid::SliderMark::unlabeled(f64::from(i))))
                    .on_value_change(move |next| {
                        value.revise(|current| {
                            *current = Some(
                                match next.round() as u8 {
                                    0 => "xs",
                                    1 => "sm",
                                    3 => "lg",
                                    4 => "xl",
                                    _ => "md",
                                }
                                .to_owned(),
                            )
                        })
                    }),
            )),
        )
    }

    pub fn size_value(value: &str) -> Size {
        match value {
            "xs" => Size::ExtraSmall,
            "sm" => Size::Small,
            "lg" => Size::Large,
            "xl" => Size::ExtraLarge,
            _ => Size::Medium,
        }
    }
}
