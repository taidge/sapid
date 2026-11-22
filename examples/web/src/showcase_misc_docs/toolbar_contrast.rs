use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .align(Align::Center)
        .gap(12.)
        .fill((
            format_toolbar(
                theme.clone(),
                "contrast-white",
                ToolbarExample::Contrast(false),
            ),
            format_toolbar(theme, "contrast-auto", ToolbarExample::Contrast(true)),
        ))
}
