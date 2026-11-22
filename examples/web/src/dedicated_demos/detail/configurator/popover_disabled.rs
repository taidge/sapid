use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let disabled = scope.cage(false);
        let preview = sapid::popover("optional-popover")
            .theme(theme.clone())
            .label("Toggle availability example")
            .title("Popover")
            .description("The trigger can be disabled.")
            .enabled(disabled.map(|v| !*v));
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::switch("Disabled", disabled, &theme)],
            ));
        panel.show_in(scope)
    })
}
