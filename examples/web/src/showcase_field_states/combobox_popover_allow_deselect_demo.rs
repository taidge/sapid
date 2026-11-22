use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let kept = scope.cage(Some("React".to_owned()));
        let cleared = scope.cage(Some("React".to_owned()));
        sapid::stack()
            .gap(16.)
            .fill((
                super::framework_popover::picker(
                    theme.clone(),
                    "framework-keep",
                    kept,
                    "Cannot deselect",
                    false,
                    super::framework_popover::Options::Frameworks,
                    None,
                ),
                super::framework_popover::picker(
                    theme.clone(),
                    "framework-clear",
                    cleared,
                    "Can deselect (default)",
                    true,
                    super::framework_popover::Options::Frameworks,
                    None,
                ),
            ))
            .show_in(scope)
    })
}
