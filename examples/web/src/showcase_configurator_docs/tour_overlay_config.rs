use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::tour_context(scope, "overlay-tour")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);
        let overlay = scope.cage(true);
        let interactive = scope.cage(false);
        let outside = scope.cage(false);
        let escape = scope.cage(true);
        let close = scope.cage(true);
        let actions = scope.cage(0usize);
        let targets = ["First target", "Second target"]
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                let key = format!("target-{i}");
                let class = scope.bond(move || {
                    if open.get() && current.get() == i {
                        "site-tour-current"
                    } else {
                        ""
                    }
                    .to_owned()
                });
                view().class(class).fill(
                    sapid::tour_target(&context, key).fill(
                        sapid::button()
                            .theme(theme.clone())
                            .variant(if i == 0 {
                                sapid::ButtonVariant::Primary
                            } else {
                                sapid::ButtonVariant::Light
                            })
                            .label(label)
                            .on_press(move || actions.revise(|n| *n += 1)),
                    ),
                )
            })
            .collect::<Vec<_>>();
        let shade = ["top", "right", "bottom", "left", "target"]
            .into_iter()
            .map(|side| {
                glory::ui::button()
                    .name("Tour overlay")
                    .label("")
                    .focus(glory::render::FocusMode::Programmatic)
                    .on(glory::render::EventKind::Activate, move |_| {
                        if interactive.get() && outside.get() {
                            open.revise(|value| *value = false);
                        }
                    })
                    .class(scope.bond(move || {
                        let enabled =
                            open.get() && overlay.get() && (side != "target" || !interactive.get());
                        if enabled {
                            format!("site-tour-shade site-tour-shade-{side}")
                        } else {
                            "site-tour-shade".to_owned()
                        }
                    }))
            })
            .collect::<Vec<_>>();
        let preview = sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::group().theme(theme.clone()).fill(targets),
            sapid::button()
                .theme(theme.clone())
                .label("Start tour")
                .on_press(move || {
                    current.revise(|v| *v = 0);
                    open.revise(|v| *v = true)
                }),
            text()
                .class("site-tour-activation-count")
                .content(actions.map(|n| {
                    if *n == 0 {
                        String::new()
                    } else {
                        format!("Target activations: {n}")
                    }
                })),
            group().fill(shade),
            sapid::tour(&context, "overlay-config")
                .theme(theme.clone())
                .open(open)
                .current(current)
                .with_close_button(close)
                .close_on_escape(escape)
                .close_on_outside_press(
                    scope.bond(move || overlay.get() && outside.get() && !interactive.get()),
                )
                .steps([
                    sapid::tour_step(
                        "target-0",
                        "First step",
                        "This is the first step of the tour.",
                    ),
                    sapid::tour_step(
                        "target-1",
                        "Second step",
                        "This is the second step of the tour.",
                    ),
                ])
                .on_open_change(move |next| open.revise(|v| *v = next))
                .on_current_change(move |next| current.revise(|v| *v = next)),
        ));
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("With overlay", overlay, &theme),
                    controls::switch("With overlay interaction", interactive, &theme),
                    controls::switch("Close on overlay click", outside, &theme),
                    controls::switch("Close on escape", escape, &theme),
                    controls::switch("With close button", close, &theme),
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
