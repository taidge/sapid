use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(135i32);
        sapid::group()
            .theme(theme.clone())
            .class("site-ref-counter-group site-ref-icon-counter")
            .gap(0.)
            .fill((
                sapid::action_icon("Decrement value")
                    .theme(theme.clone())
                    .variant(sapid::ActionIconVariant::Default)
                    .icon(
                        glory::ui::graphic()
                            .drawing(sapid::Icon::ChevronDown.drawing(Color(224, 49, 49, 255)))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                    )
                    .on_press(move || value.revise(|v| *v = (*v - 1).max(0))),
                text()
                    .class("site-ref-counter-number")
                    .content(value.map(i32::to_string)),
                sapid::action_icon("Increment value")
                    .theme(theme.clone())
                    .variant(sapid::ActionIconVariant::Default)
                    .icon(
                        glory::ui::graphic()
                            .drawing(sapid::Icon::ChevronUp.drawing(Color(18, 184, 134, 255)))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                    )
                    .on_press(move || value.revise(|v| *v += 1)),
            ))
            .show_in(scope)
    })
}
