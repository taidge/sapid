use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let owner = scope.downgrade();
        let current = scope.cage(None::<sapid::NotificationsHandle>);
        let sequence = scope.cage(0usize);
        let mut buttons = Vec::new();
        for (index, label) in ["Show low priority", "Show high priority"]
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
                        let n = sequence.get() + 1;
                        sequence.revise(|v| *v = n);
                        let result = (|| -> glory::ui::Result {
                            let high = index == 1;
                            h.show(
                                sapid::notification_request(
                                    format!("m{n}"),
                                    if high {
                                        "High priority"
                                    } else {
                                        "Low priority"
                                    },
                                    if high {
                                        "This notice takes the visible slot."
                                    } else {
                                        "Queued while urgent work is visible."
                                    },
                                )
                                .priority(if high { 10 } else { 0 })
                                .severity(if high {
                                    sapid::AlertSeverity::Danger
                                } else {
                                    sapid::AlertSeverity::Info
                                }),
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
            .class("showcase-notice-example notice-priority")
            .fill((
                sapid::group().theme(theme.clone()).fill(buttons),
                sapid::toaster("notice-priority")
                    .theme(theme.clone())
                    .visible_limit(1)
                    .on_ready(move |h| current.revise(|v| *v = Some(h))),
            ))
            .show_in(scope)
    })
}
