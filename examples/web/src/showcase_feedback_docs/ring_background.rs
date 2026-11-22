use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .fill(
            sapid::ring_progress(40.)
                .theme(theme)
                .size(120.)
                .thickness(12.)
                .root_color(Color(255, 224, 102, 255))
                .sections([sapid::ProgressSection::new(40., Color(250, 82, 82, 255))]),
        )
}
