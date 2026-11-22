use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let owner = scope.downgrade();
        let current = scope.cage(None::<sapid::NotificationsHandle>);
        let mut buttons = Vec::new();
        for (index, label) in ["Save document", "Save document (fails)"]
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
                            let token = h.show(sapid::notification_request(
                                "save",
                                "Saving document",
                                "Please wait while the document is being saved.",
                            ))?;
                            owner.spawn(async move {
                                super::notifications_system_timeouts::delay(1200).await;
                                let _ = h.update(
                                    &token,
                                    if index == 0 {
                                        "Document saved"
                                    } else {
                                        "Failed to save document"
                                    },
                                    if index == 0 {
                                        "report.pdf was saved successfully."
                                    } else {
                                        "Network error"
                                    },
                                );
                            })?;
                            Ok(())
                        })();
                        if let Err(error) = result {
                            owner.report_error(error)
                        }
                    }),
            );
        }
        view()
            .class("showcase-notice-example notice-promise")
            .fill((
                sapid::group().theme(theme.clone()).fill(buttons),
                sapid::toaster("notice-promise")
                    .theme(theme.clone())
                    .visible_limit(1)
                    .on_ready(move |h| current.revise(|v| *v = Some(h))),
            ))
            .show_in(scope)
    })
}
