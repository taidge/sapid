use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let color = scope.cage("Blue".to_owned());
        let selection = scope.cage("React".to_owned());
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![color.get()]))
            .key(Clone::clone)
            .children(move |item| {
                let shade = controls::accent(palette.clone(), &item.value.get()).accent;
                sapid::segmented_control("colored-segments")
                    .theme(palette.clone())
                    .name("Framework")
                    .value(selection)
                    .segments(["React", "Angular", "Vue"].map(|v| {
                        sapid::segment(v)
                            .label(v)
                            .style(selection.map(move |current| {
                                if current == v {
                                    Style {
                                        background: Some(shade),
                                        color: Some(glory::style::Color(255, 255, 255, 255)),
                                        ..Style::new()
                                    }
                                } else {
                                    Style::new()
                                }
                            }))
                    }))
                    .on_value_change(move |v| selection.revise(|old| *old = v))
            });
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::color_choices(color, &theme)],
            ))
            .show_in(scope)
    })
}
