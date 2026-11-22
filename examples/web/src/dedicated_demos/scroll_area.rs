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
        .name("Studio reading region").scrollbars(kind.map(|s|match s.as_deref(){Some("always")=>sapid::Scrollbars::Always,Some("hover")=>sapid::Scrollbars::Hover,Some("scroll")=>sapid::Scrollbars::Scroll,Some("never")=>sapid::Scrollbars::Never,_=>sapid::Scrollbars::Auto})).offset_scrollbars(offset.map(|is_on| if *is_on {sapid::ScrollbarOffset::Both} else {sapid::ScrollbarOffset::None})).overscroll_behavior(overscroll.map(|s|match s.as_deref(){Some("none")=>sapid::OverscrollBehavior::None,Some("contain")=>sapid::OverscrollBehavior::Contain,_=>sapid::OverscrollBehavior::Auto})).scrollbar_size(size).scroll_hide_delay(delay.map(|n|std::time::Duration::from_millis(*n as u64)))
        .style(
            Style::new()
                .width(Length::Percent(100.)).height(Length::Px(250.))
                .max_width(glory::style::Limit::Px(400.)),
        )
        .delay(|duration| host_delay(duration.as_millis() as i32))
        .content(move |_| display_sample::reading_article(theme.clone(), false))
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
