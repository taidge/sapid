use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let presses = scope.cage(0);
        let color = scope.cage("Blue".to_owned());
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(8.);
        let variant = scope.cage(Some("filled".to_owned()));
        let palette = theme.clone();
        let action = each(scope.bond(move || vec![(color.get(), size.get(), variant.get())]))
            .key(Clone::clone)
            .children(move |_| {
                let mut theme = controls::accent(palette.clone(), &color.get());
                let c = theme.accent;
                theme.accent_soft = glory::style::Color(c.0, c.1, c.2, 28);
                let choice = variant.get();
                let variant = match choice.as_deref() {
                    Some("filled") => sapid::ActionIconVariant::Filled,
                    Some("outline") => sapid::ActionIconVariant::Outline,
                    Some("default") => sapid::ActionIconVariant::Default,
                    Some("subtle" | "transparent" | "white") => sapid::ActionIconVariant::Subtle,
                    _ => sapid::ActionIconVariant::Light,
                };
                let ink = if variant == sapid::ActionIconVariant::Filled {
                    theme.on_accent
                } else {
                    theme.accent_text
                };
                let white = choice.as_deref() == Some("white");
                let transparent = choice.as_deref() == Some("transparent");
                sapid::action_icon("Confirm selection")
                    .theme(theme)
                    .size(controls::size_value(size.get().as_deref().unwrap_or("md")))
                    .variant(variant)
                    .auto_contrast(false)
                    .class(if transparent {
                        "site-action-transparent"
                    } else {
                        ""
                    })
                    .style(radius.map(move |r| {
                        let mut style = Style::new().radius(*r);
                        if white {
                            style.background = Some(Color(255, 255, 255, 255))
                        }
                        style
                    }))
                    .icon(
                        glory::ui::graphic().drawing(sliders(ink)).style(
                            Style::new()
                                .width(Length::Percent(70.))
                                .height(Length::Percent(70.)),
                        ),
                    )
                    .on_press(move || presses.revise(|n| *n += 1))
            });
        let response_theme = theme.clone();
        let preview = sapid::stack().theme(theme.clone()).gap(8.).fill((
            action,
            when(presses.map(|n| *n > 0), move || {
                sapid::text()
                    .theme(response_theme.clone())
                    .content(presses.map(|n| format!("Confirmed selections: {n}")))
            }),
        ));
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::select(
                        "action-variant",
                        "Variant",
                        variant,
                        &[
                            ("filled", "Filled"),
                            ("light", "Light"),
                            ("default", "Default"),
                            ("subtle", "Subtle"),
                            ("transparent", "Transparent"),
                            ("white", "White"),
                            ("outline", "Outline"),
                        ],
                        &theme,
                    ),
                    controls::color_choices(color, &theme),
                    controls::size_slider(size, &theme),
                    controls::pixel_scale(
                        "action_icon-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
fn sliders(color: Color) -> glory::render::GraphicProps {
    use glory::render::{GraphicPoint, PathCommand as Path};
    let p = GraphicPoint::new;
    let mut drawing = sapid::Icon::Check.drawing(color);
    drawing.commands = vec![
        Path::Move(p(2., 7.)),
        Path::Line(p(7., 7.)),
        Path::Move(p(11., 7.)),
        Path::Line(p(22., 7.)),
        Path::Move(p(2., 17.)),
        Path::Line(p(13., 17.)),
        Path::Move(p(17., 17.)),
        Path::Line(p(22., 17.)),
    ];
    for (x, y) in [(9., 7.), (15., 17.)] {
        drawing.commands.push(Path::Move(p(x + 2., y)));
        for i in 1..=12 {
            let a = f64::from(i) * std::f64::consts::TAU / 12.;
            drawing
                .commands
                .push(Path::Line(p(x + 2. * a.cos(), y + 2. * a.sin())))
        }
        drawing.commands.push(Path::Close);
    }
    drawing
}
