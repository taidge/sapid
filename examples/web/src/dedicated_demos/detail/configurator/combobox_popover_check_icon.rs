use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(true);
        let aligned = scope.cage(false);
        let position = scope.cage("left".to_owned());
        let picked = scope.cage(Some("React".to_owned()));
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(active.get(), aligned.get(), position.get())]))
            .key(Clone::clone)
            .children(move |row| {
                let (with_icon, aligned, side) = row.value.get();
                let check = if !with_icon {
                    sapid::SelectCheckPosition::Hidden
                } else if side == "left" {
                    sapid::SelectCheckPosition::Left
                } else {
                    sapid::SelectCheckPosition::Right
                };
                let palette = palette.clone();
                view().class("site-framework-check-target").fill(
                    sapid::popover("framework-check-popup")
                        .theme(palette.clone())
                        .label("Select framework")
                        .show_close(false)
                        .open(true)
                        .width(sapid::PopoverWidth::Px(200.))
                        .content(move || {
                            view().class("site-framework-scroll-list").fill(
                                sapid::combobox("check-framework-option")
                                    .name("Framework")
                                    .theme(palette.clone())
                                    .inline_list(true)
                                    .open(true)
                                    .value(picked)
                                    .filter(sapid::ComboboxFilter::None)
                                    .check_position(check)
                                    .with_aligned_labels(aligned)
                                    .options(
                                        ["React", "Angular", "Svelte", "Vue"]
                                            .map(|v| sapid::select_option(v, v)),
                                    )
                                    .on_value_change(move |next| picked.revise(|v| *v = next)),
                            )
                        }),
                )
            });
        view()
            .class("site-configurator site-field-configurator site-framework-check")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("With check icon", active, &theme),
                    controls::switch("With aligned labels", aligned, &theme),
                    controls::segment_setting(
                        "check-icon-side",
                        "Check icon position",
                        position,
                        &[("left", "Left"), ("right", "Right")],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
