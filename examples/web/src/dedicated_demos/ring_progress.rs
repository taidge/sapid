use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::ring_progress(70.)
        .theme(theme.clone())
        .name("Application data usage")
        .size(120.)
        .thickness(12.)
        .label("Application data usage")
        .style(Style {
            font_size: Some(12.),
            ..Style::new()
        })
        .sections([
            sapid::ProgressSection::new(40., Color(21, 170, 191, 255)),
            sapid::ProgressSection::new(15., Color(253, 126, 20, 255)),
            sapid::ProgressSection::new(15., Color(190, 75, 219, 255)),
        ])
}
