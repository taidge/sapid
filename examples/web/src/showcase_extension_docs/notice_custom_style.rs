use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let owner = scope.downgrade();
        let current = scope.cage(None::<sapid::NotificationsHandle>);
        let mut buttons = Vec::new();
        for (index, label) in ["Default notification", "Error notification"]
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
                                    "styled",
                                    "Notification with custom styles",
                                    if index == 0 {
                                        "The default notification is blue."
                                    } else {
                                        "The error notification is red."
                                    },
                                )
                                .severity(if index == 0 {
                                    sapid::AlertSeverity::Info
                                } else {
                                    sapid::AlertSeverity::Danger
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
            .class("showcase-notice-example notice-custom-style")
            .fill((
                sapid::group().theme(theme.clone()).fill(buttons),
                sapid::toaster("notice-custom-style")
                    .theme(theme.clone())
                    .visible_limit(1)
                    .on_ready(move |h| current.revise(|v| *v = Some(h))),
            ))
            .show_in(scope)
    })
}
