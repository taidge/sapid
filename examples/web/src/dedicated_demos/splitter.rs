use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let axis = scope.cage("horizontal".to_owned());
        let pane_theme = theme.clone();
        let preview = sapid::splitter("workspace-splitter")
            .theme(theme.clone())
            .name("Resize editor and preview")
            .class("site-splitter-colors")
            .axis(axis.map(|v| {
                if v == "vertical" {
                    glory::render::Orientation::Vertical
                } else {
                    glory::render::Orientation::Horizontal
                }
            }))
            .grip_size(52., 10.)
            .default_value(50.)
            .min(20.)
            .max(80.)
            .style(Style::new().height(Length::Px(200.)))
            .first({
                let theme = theme.clone();
                move || {
                    view().class("site-splitter-first").fill(
                        sapid::text_field()
                            .theme(theme.clone())
                            .name("Editor draft")
                            .placeholder("First pane")
                            .class("site-splitter-draft"),
                    )
                }
            })
            .second(move || {
                view().class("site-splitter-second").fill(
                    sapid::text()
                        .theme(pane_theme.clone())
                        .content("Second pane"),
                )
            });
        view()
            .class("site-configurator site-layout-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::segmented_choice(
                    "splitter-direction",
                    "Orientation",
                    axis,
                    &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
