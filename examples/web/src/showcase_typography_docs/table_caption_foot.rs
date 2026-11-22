use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    element_table(&theme, "periodic-foot", false, true, false, false)
}
