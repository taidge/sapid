use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::color_detail(theme, "color-input-format-example", true, "Format")
}
