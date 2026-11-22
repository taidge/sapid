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
                    .label("Open content modal")
                    .on_press(move || {
                        let (Some(h), Some(owner)) = (handle.get(), weak.upgrade()) else {
                            return;
                        };
                        let result = (|| -> glory::ui::Result {
                            let theme = palette.clone();
                            h.open(
                                sapid::modal_request("Subscribe", "Enter your email address.")
                                    .content(move || {
                                        sapid::text_field()
                                            .theme(theme.clone())
                                            .name("Email address")
                                            .placeholder("you@example.org")
                                    }),
                            )?;
                            Ok(())
                        })();
                        if let Err(e) = result {
                            owner.report_error(e)
                        }
                    }),
                sapid::modals_manager("modal-content")
                    .theme(theme.clone())
                    .stacked(false)
                    .on_ready(move |h| handle.revise(|v| *v = Some(h))),
            ))
            .show_in(scope)
    })
}
