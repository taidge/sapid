use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::group().theme(theme.clone()).fill(
        sapid::indicator()
            .theme(theme.clone())
            .diameter(16.)
            .offset(7.)
            .with_border(true)
            .color(Color(250, 82, 82, 255))
            .position(sapid::IndicatorPosition::BottomRight)
            .fill(display_sample::portrait(theme.clone(), false, true)),
    )
}
