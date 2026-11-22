use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let bounds = scope.cage(sapid::WindowBounds {
            x: 40.,
            y: 200.,
            width: 280.,
            height: 160.,
        });
        let axis = scope.cage(false);
        let controls = sapid::group()
            .theme(theme.clone())
            .class("site-reference-actions")
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .variant(sapid::ButtonVariant::Secondary)
                    .label(open.map(|v| {
                        if *v {
                            "Hide floating window".to_owned()
                        } else {
                            "Show floating window".to_owned()
                        }
                    }))
                    .on_press(move || open.revise(|v| *v = !*v)),
                sapid::segmented_control("stage-axis")
                    .theme(theme.clone())
                    .size(sapid::Size::ExtraSmall)
                    .name("Movement axis")
                    .value(axis.map(|horizontal| {
                        if *horizontal {
                            "x".to_owned()
                        } else {
                            "y".to_owned()
                        }
                    }))
                    .on_value_change(move |key| axis.revise(|v| *v = key == "x"))
                    .segments([
                        sapid::segment("x").label("x"),
                        sapid::segment("y").label("y"),
                    ]),
            ));
        let window = sapid::floating_window("ref-lock-axis")
            .theme(theme.clone())
            .title("Lock axis")
            .bounds(bounds)
            .open(open)
            .on_open_change(move |v| open.revise(|s| *s = v))
            .on_bounds_change(move |next| {
                bounds.revise(|v| *v = next);
            })
            .drag_axis(axis.map(|x| {
                Some(if *x {
                    glory::render::Orientation::Horizontal
                } else {
                    glory::render::Orientation::Vertical
                })
            }))
            .content(|| text().content("This window moves along the selected axis only."));
        group().fill((controls, window)).show_in(scope)
    })
}
