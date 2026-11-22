use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    display_sample::focus_form(theme, "focus-first", false)
}
