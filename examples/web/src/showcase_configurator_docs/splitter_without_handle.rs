use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::splitter("reference-no-handle")
        .theme(theme)
        .name("Resize panes")
        .class("site-splitter-colors")
        .line_size(2.)
        .with_grip(false)
        .min(20.)
        .max(80.)
        .default_value(50.)
        .style(
            Style::new()
                .height(Length::Px(200.))
                .width(Length::Percent(100.)),
        )
        .first(|| {
            view()
                .class("site-splitter-first")
                .fill(text().content("First pane"))
        })
        .second(|| {
            view()
                .class("site-splitter-second")
                .fill(text().content("Second pane"))
        })
}
