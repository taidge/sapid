use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let thickness = scope.cage(2.);
        let preview = sapid::splitter("line-splitter")
            .theme(theme.clone())
            .name("Resize example panes")
            .class("site-splitter-colors")
            .grip_size(52., 10.)
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .height(Length::Px(200.)),
            )
            .thickness(12.)
            .line_size(thickness)
            .first(|| {
                view()
                    .class("site-splitter-first")
                    .fill(text().content("First pane"))
            })
            .second(|| {
                view()
                    .class("site-splitter-second")
                    .fill(text().content("Second pane"))
            });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::range(
                    "splitter-line-size",
                    "Line size",
                    thickness,
                    1.,
                    10.,
                    1.,
                    &theme,
                )],
            ));
        panel.show_in(scope)
    })
}
