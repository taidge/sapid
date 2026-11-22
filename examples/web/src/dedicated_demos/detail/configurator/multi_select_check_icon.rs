use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::check_icon_playground(theme, "multi_select_check_icon", true)
}
