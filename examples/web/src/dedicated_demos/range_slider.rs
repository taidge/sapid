use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::slider_playground(theme, "range-slider")
}
