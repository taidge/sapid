use glory::prelude::*;
use sapid::{SemiCircleLabelPosition, semi_circle_progress, stack};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    stack()
        .theme(theme.clone())
        .align(Align::Center)
        .gap(16.)
        .fill((
            semi_circle_progress(30.)
                .theme(theme.clone())
                .size(180.)
                .label("Bottom"),
            semi_circle_progress(30.)
                .theme(theme.clone())
                .size(180.)
                .label("Center")
                .label_position(SemiCircleLabelPosition::Center),
        ))
}
