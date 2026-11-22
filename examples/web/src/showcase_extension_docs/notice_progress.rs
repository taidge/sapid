use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let owner = scope.downgrade();
        let current = scope.cage(None::<sapid::NotificationsHandle>);
        let sequence = scope.cage(0usize);
        let label = "Show notification with progress";
        let weak = owner.clone();
        let trigger = sapid::button()
            .theme(theme.clone())
            .label(label)
            .on_press(move || {
                let Some(h) = current.get() else { return };
                let Some(owner) = weak.upgrade() else { return };
                let n = sequence.get() + 1;
                sequence.revise(|v| *v = n);
                let result = (|| -> glory::ui::Result {
                    h.show(
                        sapid::notification_request(
                            format!("timed{n}"),
                            "Auto close progress",
                            "This notice closes after four seconds.",
                        )
                        .timeout(std::time::Duration::from_secs(4))
                        .auto_close_progress(sapid::NotificationProgressPosition::Bottom),
                    )?;
                    Ok(())
                })();
                if let Err(error) = result {
                    owner.report_error(error)
                }
            });
        view()
            .class("showcase-notice-example notice-progress")
            .fill((
                sapid::group().theme(theme.clone()).fill(trigger),
                sapid::toaster("notice-progress")
                    .theme(theme.clone())
                    .visible_limit(1)
                    .clock(super::notifications_system_timeouts::clock())
                    .on_ready(move |h| current.revise(|v| *v = Some(h))),
            ))
            .show_in(scope)
    })
}
