use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::color_detail(
        theme,
        "color-input-swatches-example",
        true,
        "Swatches per row",
    )
}
