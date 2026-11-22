use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let ticket = scope.cage(0usize);
        let weak = scope.downgrade();
        let demo = sapid::dropdown_menu("reference-menu-click-hover")
            .width(173.3)
            .theme(theme.clone())
            .label("Toggle menu")
            .open(open)
            .on_open_change(move |v| open.revise(|x| *x = v))
            .item(sapid::menu_label("application", "Application"))
            .item(
                sapid::menu_item("settings")
                    .left_section(
                        text()
                            .class("site-ref-menu-glyph site-ref-menu-settings")
                            .content(" "),
                    )
                    .label("Settings"),
            )
            .item(
                sapid::menu_item("messages")
                    .left_section(
                        text()
                            .class("site-ref-menu-glyph site-ref-menu-messages")
                            .content(" "),
                    )
                    .label("Messages"),
            )
            .item(
                sapid::menu_item("gallery")
                    .left_section(
                        text()
                            .class("site-ref-menu-glyph site-ref-menu-gallery")
                            .content(" "),
                    )
                    .label("Gallery"),
            )
            .item(
                sapid::menu_item("search")
                    .left_section(
                        text()
                            .class("site-ref-menu-glyph site-ref-menu-search")
                            .content(" "),
                    )
                    .label("Search")
                    .right_section(text().class("site-ref-menu-shortcut").content("⌘K")),
            )
            .separator("sections-divider")
            .item(sapid::menu_label("danger-zone", "Danger zone"))
            .item(
                sapid::menu_item("transfer")
                    .left_section(
                        text()
                            .class("site-ref-menu-glyph site-ref-menu-transfer")
                            .content(" "),
                    )
                    .label("Transfer my data"),
            )
            .item(
                sapid::menu_item("delete")
                    .left_section(
                        text()
                            .class("site-ref-menu-glyph site-ref-menu-delete")
                            .content(" "),
                    )
                    .label("Delete my account"),
            );
        view()
            .class("site-reference-hover-menu")
            .on(glory::render::EventKind::PointerWithin, move |event| {
                let glory::render::Event::PointerWithin { phase, .. } = event else {
                    return;
                };
                let entered = match phase {
                    glory::render::PointerPhase::Enter => true,
                    glory::render::PointerPhase::Leave => false,
                    _ => return,
                };
                ticket.revise(|n| *n += 1);
                let token = ticket.get();
                if let Some(s) = weak.upgrade() {
                    let _ = s.spawn(async move {
                        timer_delay(if entered { 100 } else { 400 }).await;
                        if ticket.get() == token {
                            open.revise(|x| *x = entered)
                        }
                    });
                }
            })
            .fill(demo)
            .show_in(scope)
    })
}

fn timer_delay(millis: i32) -> glory::core::LocalFuture {
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
