use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let handle = scope.cage(None::<sapid::ModalsHandle>);
        let weak = scope.downgrade();
        let palette = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Open first modal")
                    .on_press(move || {
                        let (Some(h), Some(owner)) = (handle.get(), weak.upgrade()) else {
                            return;
                        };
                        let result = (|| -> glory::ui::Result {
                            let command = h.clone();
                            let theme = palette.clone();
                            h.open(
                                sapid::modal_request(
                                    "First modal",
                                    "Open the next step while retaining this dialog.",
                                )
                                .content(move || {
                                    let h = command.clone();
                                    sapid::button()
                                        .theme(theme.clone())
                                        .label("Open second modal")
                                        .on_press(move || {
                                            let _ = h.open(
                                                sapid::modal_request(
                                                    "Second modal",
                                                    "The previous dialog is still open.",
                                                )
                                                .close_label("Back to first modal"),
                                            );
                                        })
                                }),
                            )?;
                            Ok(())
                        })();
                        if let Err(e) = result {
                            owner.report_error(e)
                        }
                    }),
                sapid::modals_manager("modal-stacked")
                    .theme(theme.clone())
                    .stacked(true)
                    .on_ready(move |h| handle.revise(|v| *v = Some(h))),
            ))
            .show_in(scope)
    })
}
