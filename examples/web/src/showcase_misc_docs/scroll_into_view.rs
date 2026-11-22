use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(0usize);
        let handle = scope.cage(None::<sapid::ScrollHandle>);
        let owner = scope.downgrade();
        let jump = move |step: i32, label: &'static str| {
            let owner = owner.clone();
            sapid::button().label(label).on_press(move || {
                let index = (active.get() as i32 + step).clamp(0, 29) as usize;
                active.revise(|v| *v = index);
                if let Some(viewport) = handle.get()
                    && let Err(error) = viewport.scroll_to(0., index as f64 * 32.)
                    && let Some(scope) = owner.upgrade()
                {
                    scope.report_error(error);
                }
            })
        };
        let palette = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::scroll_area("jump-to-row")
                    .theme(theme.clone())
                    .name("Search results")
                    .style(
                        Style::new()
                            .max_width(Limit::Px(330.))
                            .height(Length::Px(144.)),
                    )
                    .on_ready(move |h| handle.revise(|v| *v = Some(h)))
                    .content(move |_| {
                        sapid::stack().theme(palette.clone()).gap(0.).fill(
                            (0..30)
                                .map(|i| {
                                    let (surface, active_fill) =
                                        (palette.surface, palette.accent_soft);
                                    sapid::text()
                                        .theme(palette.clone())
                                        .content(format!("Result {}", i + 1))
                                        .style(active.map(move |n| {
                                            Style {
                                                background: Some(if *n == i {
                                                    active_fill
                                                } else {
                                                    surface
                                                }),
                                                ..Style::new()
                                                    .display(Display::Block)
                                                    .height(Length::Px(32.))
                                                    .padding(Edges::all(6.))
                                            }
                                        }))
                                })
                                .collect::<Vec<_>>(),
                        )
                    }),
                sapid::group()
                    .theme(theme.clone())
                    .gap(8.)
                    .fill((jump(-1, "Previous result"), jump(1, "Next result"))),
            ))
            .show_in(scope)
    })
}
