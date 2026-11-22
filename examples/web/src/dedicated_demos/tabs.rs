use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage("gallery".to_owned());
        let variant = scope.cage("default".to_owned());
        let axis = scope.cage("horizontal".to_owned());
        let radius = scope.cage(8.);
        let color = scope.cage("Blue".to_owned());
        let palette = theme.clone();
        let preview = each(
            scope.bond(move || vec![(variant.get(), axis.get(), radius.get() as u8, color.get())]),
        )
        .key(Clone::clone)
        .children(move |entry| {
            let (variant, axis, radius, color) = entry.value.get();
            let mut theme = controls::accent(palette.clone(), &color);
            theme.radius = f64::from(radius);
            sapid::tabs("showcase-tabs")
                .theme(theme.clone())
                .name("Account settings")
                .variant(match variant.as_str() {
                    "outline" => sapid::TabsVariant::Outline,
                    "pills" => sapid::TabsVariant::Pills,
                    _ => sapid::TabsVariant::Default,
                })
                .orientation(if axis == "vertical" {
                    glory::render::Orientation::Vertical
                } else {
                    glory::render::Orientation::Horizontal
                })
                .value(selected)
                .on_value_change(move |next| selected.revise(|v| *v = next))
                .tab(
                    sapid::tab("gallery")
                        .label("Gallery")
                        .trigger_content({
                            let ink = theme.foreground;
                            move || {
                                glory::ui::group().fill((
                                    glory::ui::graphic()
                                        .drawing(sapid::Icon::Calendar.drawing(ink))
                                        .style(
                                            Style::new()
                                                .width(Length::Px(14.))
                                                .height(Length::Px(14.)),
                                        ),
                                    text().content("Gallery"),
                                ))
                            }
                        })
                        .content({
                            let theme = theme.clone();
                            move || {
                                sapid::text()
                                    .theme(theme.clone())
                                    .content("Gallery tab content")
                            }
                        }),
                )
                .tab(
                    sapid::tab("messages")
                        .label("Messages")
                        .trigger_content({
                            let ink = theme.foreground;
                            move || {
                                glory::ui::group().fill((
                                    glory::ui::graphic()
                                        .drawing(sapid::Icon::Info.drawing(ink))
                                        .style(
                                            Style::new()
                                                .width(Length::Px(14.))
                                                .height(Length::Px(14.)),
                                        ),
                                    text().content("Messages"),
                                ))
                            }
                        })
                        .content({
                            let theme = theme.clone();
                            move || {
                                sapid::text()
                                    .theme(theme.clone())
                                    .content("Messages tab content")
                            }
                        }),
                )
                .tab(
                    sapid::tab("settings")
                        .label("Settings")
                        .trigger_content({
                            let ink = theme.foreground;
                            move || {
                                glory::ui::group().fill((
                                    glory::ui::graphic()
                                        .drawing(sapid::Icon::More.drawing(ink))
                                        .style(
                                            Style::new()
                                                .width(Length::Px(14.))
                                                .height(Length::Px(14.)),
                                        ),
                                    text().content("Settings"),
                                ))
                            }
                        })
                        .content(move || {
                            sapid::text()
                                .theme(theme.clone())
                                .content("Settings tab content")
                        }),
                )
        });
        view()
            .class("site-configurator site-layout-configurator site-tabs-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::color_choices(color, &theme),
                    controls::segmented_choice(
                        "tab-variant",
                        "Variant",
                        variant,
                        &[
                            ("default", "Default"),
                            ("outline", "Outline"),
                            ("pills", "Pills"),
                        ],
                        &theme,
                    ),
                    controls::pixel_scale(
                        "tabs-radius",
                        "Radius",
                        radius,
                        [2., 4., 8., 16., 32.],
                        &theme,
                    ),
                    controls::segmented_choice(
                        "tab-axis",
                        "Orientation",
                        axis,
                        &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
