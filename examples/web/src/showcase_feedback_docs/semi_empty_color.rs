use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::semi_circle_progress(40.)
        .theme(theme)
        .name("Completion")
        .size(190.)
        .thickness(12.)
        .empty_segment_color(Color(134, 142, 150, 255))
        .label("40%")
}
