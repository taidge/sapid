use glory::prelude::*;
use glory::render::Orientation;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let placement = scope.cage("left".to_owned());
        let selected = scope.cage("gallery".to_owned());
        let preview = sapid::tabs("placed-tabs")
            .theme(theme.clone())
            .name("Vertical tab placement")
            .orientation(Orientation::Vertical)
            .class(placement.map(|v| format!("site-tabs-placement-{v}")))
            .value(selected)
            .on_value_change(move |s| selected.revise(|v| *v = s))
            .tab(
                sapid::tab("gallery")
                    .label("Gallery")
                    .content(|| text().content("Gallery tab content")),
            )
            .tab(
                sapid::tab("messages")
                    .label("Messages")
                    .content(|| text().content("Messages tab content")),
            )
            .tab(
                sapid::tab("settings")
                    .label("Settings")
                    .content(|| text().content("Settings tab content")),
            );
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::segmented_choice(
                    "tabs-placement",
                    "Placement",
                    placement,
                    &[("left", "Left"), ("right", "Right")],
                    &theme,
                )],
            ));
        panel.show_in(scope)
    })
}
