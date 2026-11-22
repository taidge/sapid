use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        super::framework_popover::picker(
            theme.clone(),
            "framework-large",
            scope.cage(None),
            "Select option",
            false,
            super::framework_popover::Options::Scrollable,
            None,
        )
        .show_in(scope)
    })
}
