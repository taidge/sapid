use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let owner = scope.downgrade();
        let current = scope.cage(None::<sapid::NotificationsHandle>);
        let mut buttons = Vec::new();
        for (index, label) in ["Progress at the top", "Progress at the bottom"]
            .into_iter()
            .enumerate()
        {
            let weak = owner.clone();
            buttons.push(
                sapid::button()
                    .theme(theme.clone())
                    .label(label)
                    .on_press(move || {
                        let Some(h) = current.get() else { return };
                        let Some(owner) = weak.upgrade() else { return };
                        let result = (|| -> glory::ui::Result {
                            h.show(
                                sapid::notification_request(
                                    "timed",
                                    "Time remaining",
                                    "This notice closes after four seconds.",
                                )
                                .timeout(std::time::Duration::from_secs(4))
                                .auto_close_progress(
                                    if index == 0 {
                                        sapid::NotificationProgressPosition::Top
                                    } else {
                                        sapid::NotificationProgressPosition::Bottom
                                    },
                                ),
                            )?;
                            Ok(())
                        })();
                        if let Err(error) = result {
                            owner.report_error(error)
                        }
                    }),
            );
        }
        view()
            .class("showcase-notice-example notice-progress-position")
            .fill((
                sapid::group().theme(theme.clone()).fill(buttons),
                sapid::toaster("notice-progress-position")
                    .theme(theme.clone())
                    .visible_limit(1)
                    .clock(super::notifications_system_timeouts::clock())
                    .on_ready(move |h| current.revise(|v| *v = Some(h))),
            ))
            .show_in(scope)
    })
}
