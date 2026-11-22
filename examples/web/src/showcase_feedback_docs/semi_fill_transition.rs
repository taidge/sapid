use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(30_f64);
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Center)
            .gap(14.)
            .fill((
                sapid::semi_circle_progress(value)
                    .theme(theme.clone())
                    .name("Completion")
                    .class("sapid-demo-progress-transition")
                    .size(120.)
                    .thickness(12.)
                    .label(value.map(|n| format!("{n:.0}%"))),
                sapid::button()
                    .theme(theme.clone())
                    .label("Set next value")
                    .on_press(move || value.revise(|n| *n = (*n + 37.) % 101.)),
            ))
            .show_in(scope)
    })
}
