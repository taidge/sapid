use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let shadow = scope.cage(Some("small".to_owned()));
        let radius = scope.cage(10_f64);
        let border = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(shadow.get(), border.get())]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();

                sapid::paper()
                    .theme(theme.clone())
                    .bordered(border.get())
                    .shadow(match shadow.get().as_deref() {
                        Some("small") => sapid::ShadowSize::Small,
                        Some("medium") => sapid::ShadowSize::Medium,
                        Some("large") => sapid::ShadowSize::Large,
                        _ => sapid::ShadowSize::None,
                    })
                    .style(radius.map(|r| Style::new().padding(Edges::all(20.)).radius(*r)))
                    .fill(
                        sapid::stack().theme(theme.clone()).gap(10.).fill((
                            sapid::title(3)
                                .theme(theme.clone())
                                .weight(400.)
                                .content("A place for a useful note"),
                            sapid::text().theme(theme.clone()).content(TEXT_1),
                        )),
                    )
            });
        view()
            .class("site-configurator site-display-configurator site-paper-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "paper-shadow",
                        "Shadow",
                        shadow,
                        &[
                            ("none", "None"),
                            ("small", "Small"),
                            ("medium", "Medium"),
                            ("large", "Large"),
                        ],
                        &theme,
                    ),
                    controls::range("paper-radius", "Radius", radius, 0., 24., 1., &theme),
                    controls::switch("With border", border, &theme),
                ],
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str = "Paper supplies the surface. Your own widgets supply the content and actions.";

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

    pub fn select(
        key: &str,
        label: &str,
        value: Cage<Option<String>>,
        options: &'static [(&'static str, &'static str)],
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let field_key = format!("control-{key}");
        let select_key = format!("control-select-{key}");
        Box::new(
            sapid::field(field_key)
                .theme(theme.clone())
                .label(label.to_owned())
                .control(move |field| {
                    let mut select = sapid::native_select(select_key.clone()).field(field);
                    for (value, label) in options {
                        select = select.option(sapid::select_option(*value, *label));
                    }
                    select.value(value).on_value_change(move |next| {
                        if next.is_some() {
                            value.revise(|current| *current = next)
                        }
                    })
                }),
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

    pub fn range(
        key: &str,
        label: &str,
        value: Cage<f64>,
        min: f64,
        max: f64,
        step: f64,
        theme: &Theme,
    ) -> Box<dyn Widget> {
        let label = label.to_owned();
        Box::new(
            sapid::stack().theme(theme.clone()).gap(8.).fill((
                sapid::text()
                    .theme(theme.clone())
                    .class("site-control-label")
                    .content(value.map({
                        let label = label.clone();
                        move |v| {
                            let decimals = if step.fract() == 0. {
                                0
                            } else if (step * 10.).fract() == 0. {
                                1
                            } else {
                                2
                            };
                            format!("{label} · {v:.decimals$}")
                        }
                    })),
                sapid::slider(key.to_owned())
                    .theme(theme.clone())
                    .name(label)
                    .min(min)
                    .max(max)
                    .step(step)
                    .value(value)
                    .on_value_change(move |next| value.revise(|v| *v = next)),
            )),
        )
    }
}
