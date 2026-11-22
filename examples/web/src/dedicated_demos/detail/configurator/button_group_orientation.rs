use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let axis = scope.cage("horizontal".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![axis.get()]))
            .key(Clone::clone)
            .children(move |entry| {
                sapid::button_group()
                    .theme(palette.clone())
                    .name("Example group")
                    .orientation(if entry.value.get() == "vertical" {
                        glory::render::Orientation::Vertical
                    } else {
                        glory::render::Orientation::Horizontal
                    })
                    .button(
                        sapid::button()
                            .variant(sapid::ButtonVariant::Secondary)
                            .label("First"),
                    )
                    .button(
                        sapid::button()
                            .variant(sapid::ButtonVariant::Secondary)
                            .label("Second"),
                    )
                    .button(
                        sapid::button()
                            .variant(sapid::ButtonVariant::Secondary)
                            .label("Third"),
                    )
            });
        let panel = view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::segmented_choice(
                    "group-orientation",
                    "Orientation",
                    axis,
                    &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                    &theme,
                )],
            ));
        panel.show_in(scope)
    })
}
