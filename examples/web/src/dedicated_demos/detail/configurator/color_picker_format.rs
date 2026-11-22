use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::color_detail(theme, "picker-format-example", false, "Format")
}
