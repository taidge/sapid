use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().class("site-default-color-swatches").fill(
        sapid::color_picker("palette-with-picker")
            .theme(theme)
            .size(sapid::Size::Small)
            .plane_size(240.)
            .plane_height(110.)
            .show_alpha(false)
            .default_value(sapid::Hsva::from_rgba(glory::style::Color(
                50, 151, 194, 255,
            )))
            .swatches([
                "#2e2e2e", "#868e96", "#fa5252", "#e64980", "#be4bdb", "#7950f2", "#4c6ef5",
                "#228be6", "#15aabf", "#12b886", "#40c057", "#82c91e", "#fab005", "#fd7e14",
            ])
            .swatches_per_row(7),
    )
}
