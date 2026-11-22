use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let owner = scope.downgrade();
        let current = scope.cage(None::<sapid::NotificationsHandle>);
        let sequence = scope.cage(0usize);
        let snapshot = scope.bond(move || {
            current
                .get()
                .and_then(|h| h.visible_and_queued().ok())
                .unwrap_or_default()
        });
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Show notification")
                    .on_press(move || {
                        if let Some(h) = current.get() {
                            let n = sequence.get() + 1;
                            sequence.revise(|v| *v = n);
                            if let Err(e) = h.show(sapid::notification_request(
                                format!("notice-{n}"),
                                format!("Notification {n}"),
                                "Most notifications go to the queue.",
                            )) && let Some(o) = owner.upgrade()
                            {
                                o.report_error(e)
                            }
                        }
                    }),
                sapid::toaster("notice-store")
                    .theme(theme.clone())
                    .visible_limit(1)
                    .on_ready(move |h| current.revise(|v| *v = Some(h))),
                text().content(
                    scope.bond(move || {
                        snapshot.with(|(visible, _)| format!("Visible: {visible:?}"))
                    }),
                ),
                text().content(
                    scope.bond(move || snapshot.with(|(_, queued)| format!("Queued: {queued:?}"))),
                ),
            ))
            .show_in(scope)
    })
}
