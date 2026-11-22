use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().class("sapid-demo-native-table-scroll").fill(
        element_table(&theme, "periodic-native", false, false, false, false)
            .style(Style::new().min_width(Length::Px(500.))),
    )
}
