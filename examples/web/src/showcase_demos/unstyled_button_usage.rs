use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(0);
        let feedback_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .class("site-standalone-action")
            .gap(8.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .unstyled()
                    .class("site-bare-native")
                    .label("Button without styles")
                    .on_press(move || count.revise(|v| *v += 1)),
                when(count.map(|v| *v > 0), move || {
                    sapid::text()
                        .theme(feedback_theme.clone())
                        .content(count.map(|v| format!("Button presses: {v}")))
                }),
            ))
            .show_in(scope)
    })
}
