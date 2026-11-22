use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let owner = scope.downgrade();
        let current = scope.cage(None::<sapid::NotificationsHandle>);
        let label = "Show custom notification";
        let weak = owner.clone();
        let palette = theme.clone();
        let trigger = sapid::button()
            .theme(theme.clone())
            .label(label)
            .on_press(move || {
                let Some(h) = current.get() else { return };
                let Some(owner) = weak.upgrade() else { return };
                let result = (|| -> glory::ui::Result {
                    let theme = palette.clone();
                    h.show(
                        sapid::notification_request(
                            "dm",
                            "New message",
                            "A colleague sent you a message.",
                        )
                        .render(move |token, h| {
                            let reply = token.clone();
                            let first = h.clone();
                            sapid::card()
                                .theme(theme.clone())
                                .title("Dan sent you a message")
                                .description("Are you free for a quick call?")
                                .fill(
                                    sapid::group().theme(theme.clone()).fill((
                                        sapid::button()
                                            .theme(theme.clone())
                                            .label("Reply")
                                            .on_press(move || {
                                                let _ = first.dismiss(&reply);
                                            }),
                                        sapid::button()
                                            .theme(theme.clone())
                                            .label("Dismiss")
                                            .on_press(move || {
                                                let _ = h.dismiss(&token);
                                            }),
                                    )),
                                )
                        }),
                    )?;
                    Ok(())
                })();
                if let Err(error) = result {
                    owner.report_error(error)
                }
            });
        view()
            .class("showcase-notice-example notice-custom-render")
            .fill((
                sapid::group().theme(theme.clone()).fill(trigger),
                sapid::toaster("notice-custom-render")
                    .theme(theme.clone())
                    .visible_limit(1)
                    .on_ready(move |h| current.revise(|v| *v = Some(h))),
            ))
            .show_in(scope)
    })
}
