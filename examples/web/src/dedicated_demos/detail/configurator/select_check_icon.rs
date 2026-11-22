use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::check_icon_playground(theme, "select_check_icon", false)
}
