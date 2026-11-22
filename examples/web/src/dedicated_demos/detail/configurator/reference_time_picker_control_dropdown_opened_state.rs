use glory::prelude::*;
use sapid::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let opened = scope.cage(false);
        let value = scope.cage(None::<LocalTime>);
        sapid::time_picker("time-9")
            .theme(theme.clone())
            .name("Enter time")
            .with_dropdown(true)
            .dropdown_icon(sapid::Icon::Clock)
            .class("site-time-clock-picker")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(340.)),
            )
            .open(opened)
            .on_open_change(move |next| opened.revise(|current| *current = next))
            .value(value)
            .on_value_change(move |next| {
                if value.get().is_none() && next.is_some() {
                    opened.revise(|current| *current = false);
                }
                value.revise(|current| *current = next);
            })
            .show_in(scope)
    })
}
