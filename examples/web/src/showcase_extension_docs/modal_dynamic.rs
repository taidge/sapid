use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let handle = scope.cage(None::<sapid::ModalsHandle>);
        let weak = scope.downgrade();
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Open updating modal")
                    .on_press(move || {
                        let (Some(h), Some(owner)) = (handle.get(), weak.upgrade()) else {
                            return;
                        };
                        let result = (|| -> glory::ui::Result {
                            let token = h.open(sapid::modal_request(
                                "Initial modal title",
                                "This text updates after two seconds.",
                            ))?;
                            owner.spawn(async move {
                                super::notifications_system_timeouts::delay(1600).await;
                                let _ = h.update(
                                    &token,
                                    "Updated modal title",
                                    "This is the updated content of the modal.",
                                );
                            })?;
                            Ok(())
                        })();
                        if let Err(e) = result {
                            owner.report_error(e)
                        }
                    }),
                sapid::modals_manager("modal-dynamic")
                    .theme(theme.clone())
                    .stacked(false)
                    .on_ready(move |h| handle.revise(|v| *v = Some(h))),
            ))
            .show_in(scope)
    })
}
