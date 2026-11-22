use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage(sapid::Hsva::from_rgba(glory::style::Color(
            255, 255, 255, 255,
        )));
        let caption = color.map(|value| {
            let hex = sapid::format_hex_color(value.to_rgba().expect("valid picker value"), false);
            if hex == "#ffffff" {
                "#fff".to_owned()
            } else {
                hex
            }
        });
        sapid::stack()
            .theme(theme.clone())
            .class("site-reference-palette")
            .gap(5.)
            .fill((
                sapid::color_picker("docs-picker-palette")
                    .theme(theme.clone())
                    .value(color)
                    .show_picker(false)
                    .full_width(true)
                    .swatches([
                        "#fff5f5", "#ffe3e3", "#ffc9c9", "#ffa8a8", "#ff8787", "#ff6b6b",
                        "#fa5252", "#ebfbee", "#d3f9d8", "#b2f2bb", "#8ce99a", "#69db7c",
                        "#51cf66", "#40c057", "#e7f5ff", "#d0ebff", "#a5d8ff", "#74c0fc",
                        "#4dabf7", "#339af0", "#228be6",
                    ])
                    .swatches_per_row(7)
                    .on_value_change(move |next| color.revise(|current| *current = next)),
                text()
                    .class("site-reference-palette-value")
                    .content(caption),
            ))
            .show_in(scope)
    })
}
