use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    controls::choice_playground(theme, "switch")
}
