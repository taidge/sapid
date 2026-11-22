use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let body = theme.clone();
        view()
            .class("site-reference-dialog-buttons")
            .fill(
                sapid::popover("reference-focus-fields")
                    .theme(theme.clone())
                    .open(open)
                    .on_open_change(move |v| open.revise(|s| *s = v))
                    .label("Toggle popover")
                    .title("Contact details")
                    .with_arrow(true)
                    .style(Style::new().width(Length::Px(300.)))
                    .content(move || {
                        sapid::focus_scope()
                            .active(open)
                            .loop_tab(true)
                            .contain_focus(true)
                            .auto_focus(true)
                            .fill(
                                sapid::stack().theme(body.clone()).fill((
                                    sapid::field("popover-reference-name")
                                        .theme(body.clone())
                                        .label("Name")
                                        .control(|field| {
                                            sapid::text_field().field(field).placeholder("Name")
                                        }),
                                    sapid::field("popover-reference-email")
                                        .theme(body.clone())
                                        .label("Email")
                                        .control(|field| {
                                            sapid::text_field()
                                                .field(field)
                                                .placeholder("john@doe.com")
                                        }),
                                )),
                            )
                    }),
            )
            .show_in(scope)
    })
}
