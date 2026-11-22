use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let kind = scope.cage(Some("hover".to_owned()));
        let offset = scope.cage(false);
        let overscroll = scope.cage(Some("auto".to_owned()));
        let size = scope.cage(10_f64);
        let delay = scope.cage(1000_f64);
        let preview = {
            let theme = theme.clone();

            sapid::scroll_area("studio-reading")
                .theme(theme.clone())
                .name("Studio reading region")
                .scrollbars(kind.map(|s| match s.as_deref() {
                    Some("always") => sapid::Scrollbars::Always,
                    Some("hover") => sapid::Scrollbars::Hover,
                    Some("scroll") => sapid::Scrollbars::Scroll,
                    Some("never") => sapid::Scrollbars::Never,
                    _ => sapid::Scrollbars::Auto,
                }))
                .offset_scrollbars(offset.map(|is_on| {
                    if *is_on {
                        sapid::ScrollbarOffset::Both
                    } else {
                        sapid::ScrollbarOffset::None
                    }
                }))
                .overscroll_behavior(overscroll.map(|s| match s.as_deref() {
                    Some("none") => sapid::OverscrollBehavior::None,
                    Some("contain") => sapid::OverscrollBehavior::Contain,
                    _ => sapid::OverscrollBehavior::Auto,
                }))
                .scrollbar_size(size)
                .scroll_hide_delay(delay.map(|n| std::time::Duration::from_millis(*n as u64)))
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .height(Length::Px(250.))
                        .max_width(glory::style::Limit::Px(400.)),
                )
                .delay(|duration| host_delay(duration.as_millis() as i32))
                .content(move |_| reading_article(theme.clone(), false))
        };
        view()
            .class("site-configurator site-display-configurator site-scroll-area-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "scroll_area-kind",
                        "Type",
                        kind,
                        &[
                            ("hover", "Hover"),
                            ("auto", "Auto"),
                            ("always", "Always"),
                            ("scroll", "Scroll"),
                            ("never", "Never"),
                        ],
                        &theme,
                    ),
                    controls::switch("Offset scrollbars", offset, &theme),
                    controls::option_segments(
                        "scroll_area-overscroll",
                        "Overscroll behavior",
                        overscroll,
                        &[("auto", "Auto"), ("contain", "Contain"), ("none", "None")],
                        &theme,
                    ),
                    controls::range(
                        "scroll_area-size",
                        "Scrollbar size",
                        size,
                        2.,
                        20.,
                        2.,
                        &theme,
                    ),
                    controls::range(
                        "scroll_area-delay",
                        "Scroll hide delay",
                        delay,
                        0.,
                        6000.,
                        500.,
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}

fn host_delay(millis: i32) -> glory::core::LocalFuture {
    #[cfg(target_arch = "wasm32")]
    {
        Box::pin(timer::Delay::new(millis))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = millis;
        Box::pin(std::future::ready(()))
    }
}
#[cfg(target_arch = "wasm32")]
mod timer {
    use std::{
        cell::{Cell, RefCell},
        future::Future,
        pin::Pin,
        rc::Rc,
        task::{Context, Poll, Waker},
    };
    use wasm_bindgen::{JsCast, prelude::*};
    struct TimerState {
        done: Cell<bool>,
        waker: RefCell<Option<Waker>>,
    }
    pub struct Delay {
        millis: i32,
        state: Rc<TimerState>,
        timer: Option<i32>,
        callback: Option<Closure<dyn FnMut()>>,
    }
    impl Delay {
        pub fn new(millis: i32) -> Self {
            Self {
                millis,
                state: Rc::new(TimerState {
                    done: Cell::new(false),
                    waker: RefCell::new(None),
                }),
                timer: None,
                callback: None,
            }
        }
    }
    impl Future for Delay {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.state.done.get() {
                return Poll::Ready(());
            }
            let previous = self.state.waker.borrow_mut().replace(cx.waker().clone());
            drop(previous);
            if self.timer.is_none() {
                let state = self.state.clone();
                let callback = Closure::new(move || {
                    state.done.set(true);
                    let waker = state.waker.borrow_mut().take();
                    if let Some(waker) = waker {
                        waker.wake();
                    }
                });
                self.timer = Some(
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            callback.as_ref().unchecked_ref(),
                            self.millis,
                        )
                        .unwrap(),
                );
                self.callback = Some(callback);
            }
            Poll::Pending
        }
    }
    impl Drop for Delay {
        fn drop(&mut self) {
            if let Some(timer) = self.timer.take()
                && let Some(window) = web_sys::window()
            {
                window.clear_timeout_with_handle(timer);
            }
        }
    }
}

fn reading_article(theme: sapid::Theme, wide: bool) -> impl Widget {
    let width = if wide {
        Length::Px(600.)
    } else {
        Length::Percent(100.)
    };
    let heading = sapid::text()
        .theme(theme.clone())
        .weight(700.)
        .style(Style {
            font_size: Some(20.),
            ..Style::new()
        })
        .content("Charizard (Pokémon)");
    let attribution = sapid::text()
        .theme(theme.clone())
        .style(Style {
            color: Some(theme.muted),
            ..Style::new()
        })
        .content("Charizard description from Bulbapedia");
    let paragraphs = [
        "Charizard is an orange, two-legged Pokémon with a long tail tipped by a flame. Its wings have blue-green undersides, and its light-colored belly stretches from its chest to the base of its tail. Two horns rise from the back of its head. It flies in search of strong opponents, breathing fire that can melt rock and carrying itself high above mountain ridges. Each foot has three claws, and two pointed teeth show above its lower jaw. Its narrow arms contrast with a broad chest and powerful back legs. When it rests on the ground, its folded wings rise above its shoulders. The fire at the tip of its tail burns on land and in flight.",
            "Mega Charizard X has dark skin, a blue underside, and blue flames that trail from its mouth. Its broad shoulders carry a pair of spikes, and its wings and horns change shape. Its claws and brow grow larger as its flames intensify. Its horns curve upward, and small spines appear near the neck. The fire at the sides of its mouth glows blue. The lower edge of each wing follows a rounded outline, while the pointed tips reach beyond its shoulders. From the ground its dark silhouette looks quite different from its familiar orange form."
    ].into_iter().map(|paragraph| {
        sapid::text().theme(theme.clone()).style(Style {
            font_size: Some(14.),
            margin: Some(glory::style::Edges { top:16., right:0., bottom:0., left:0. }),
            ..Style::new()
        }).content(paragraph)
    }).collect::<Vec<_>>();
    sapid::stack()
        .theme(theme.clone())
        .gap(0.)
        .style(Style::new().width(width))
        .fill((
            heading,
            attribution,
            sapid::stack().theme(theme).gap(0.).fill(paragraphs),
        ))
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
