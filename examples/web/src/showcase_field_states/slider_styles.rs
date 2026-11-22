use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::slider("styles-slider")
        .theme(theme)
        .name("Value")
        .default_value(40.)
        .marks([
            sapid::SliderMark::new(20., "20%"),
            sapid::SliderMark::new(80., "80%"),
        ]);
    crate::showcase_style_explorer::explorer(
        "slider-styles",
        Box::new(
            sapid::stack()
                .class("site-field-state-example")
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(340.)),
                )
                .fill(preview),
        ),
        &[
            ("root", "Root element", ".sapid-slider"),
            ("label", "Thumb label", ".sapid-slider-mark-label"),
            ("thumb", "Thumb element", ".sapid-slider-thumb"),
            ("trackContainer", "Wraps track element", ".sapid-slider"),
            ("track", "Slider track", ".sapid-slider-track"),
            ("bar", "Track filled part", ".sapid-slider-fill"),
            (
                "markWrapper",
                "Contains `mark` and `markLabel` elements",
                ".sapid-slider-mark",
            ),
            ("mark", "Mark displayed on track", ".sapid-slider-mark"),
            (
                "markLabel",
                "Label of the associated mark, displayed below track",
                ".sapid-slider-mark-label",
            ),
        ],
    )
}
