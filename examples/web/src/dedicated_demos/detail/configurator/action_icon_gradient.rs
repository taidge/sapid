use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let from = scope.cage(Some("Blue".to_owned()));
        let to = scope.cage(Some("Cyan".to_owned()));
        let angle = scope.cage(90.);
        let style = scope.bond(move || {
            Style::new().background_image(controls::gradient_style(
                from.get().as_deref(),
                to.get().as_deref(),
                angle.get(),
            ))
        });
        let preview = sapid::action_icon("Gradient action")
            .theme(theme.clone())
            .variant(sapid::ActionIconVariant::Filled)
            .style(style)
            .icon(
                glory::ui::graphic()
                    .drawing(sapid::Icon::Check.drawing(theme.on_accent))
                    .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
            );
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::gradient_controls(from, to, angle, &theme)],
            ));
        panel.show_in(scope)
    })
}
