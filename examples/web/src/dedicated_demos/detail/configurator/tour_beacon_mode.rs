use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let context = sapid::tour_context(scope, "reference-beacons")?;
        let open = scope.cage(false);
        let current = scope.cage(0usize);

        let beacons = scope.cage(false);
        let rows = [
            ("first", "First target"),
            ("second", "Second target"),
            ("third", "Third target"),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (id, label))| {
            let target: Box<dyn Widget> =
                if index == 1 {
                    Box::new(
                        sapid::text_field()
                            .theme(theme.clone())
                            .name(label)
                            .placeholder(label),
                    )
                } else {
                    Box::new(sapid::button().theme(theme.clone()).label(label).variant(
                        if index == 2 {
                            sapid::ButtonVariant::Outline
                        } else {
                            sapid::ButtonVariant::Primary
                        },
                    ))
                };
            view().class("site-ref-beacon-target").fill((
                sapid::tour_target(&context, id).fill(target),
                when(beacons, move || {
                    glory::ui::button()
                        .name(format!("Beacon {}", index + 1))
                        .label("")
                        .class("site-ref-beacon")
                        .on(glory::render::EventKind::Activate, move |_| {
                            current.revise(|v| *v = index);
                            open.revise(|v| *v = true)
                        })
                }),
            ))
        })
        .collect::<Vec<_>>();
        sapid::stack()
            .theme(theme.clone())
            .class("site-ref-tour-form")
            .gap(16.)
            .fill((
                sapid::group()
                    .theme(theme.clone())
                    .class("site-reference-actions")
                    .fill(rows),
                sapid::button()
                    .theme(theme.clone())
                    .label(beacons.map(|v| {
                        if *v {
                            "End beacon tour".to_owned()
                        } else {
                            "Start beacon tour".to_owned()
                        }
                    }))
                    .on_press(move || {
                        beacons.revise(|v| *v = !*v);
                        open.revise(|v| *v = false)
                    }),
                sapid::tour(&context, "reference-beacons")
                    .theme(theme.clone())
                    .open(open)
                    .current(current)
                    .steps([
                        sapid::tour_step(
                            "first",
                            "Beacon 1",
                            "Select a beacon to open its tooltip.",
                        ),
                        sapid::tour_step(
                            "second",
                            "Beacon 2",
                            "This is the second beacon tooltip.",
                        ),
                        sapid::tour_step("third", "Beacon 3", "This is the third beacon tooltip."),
                    ])
                    .on_current_change(move |v| current.revise(|old| *old = v))
                    .on_open_change(move |v| open.revise(|old| *old = v)),
            ))
            .show_in(scope)
    })
}
