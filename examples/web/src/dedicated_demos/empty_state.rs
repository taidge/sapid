use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let did_reset = scope.cage(false);

        let size = scope.cage(Some("md".to_owned()));
        let align = scope.cage(Some("center".to_owned()));
        let background = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(size.get(),)]))
            .key(Clone::clone)
            .children(move |_| {
                let theme = palette.clone();
                let empty_theme = theme.clone();
                let ready_theme = theme.clone();
                sapid::stack().theme(theme.clone()).gap(12.).fill((
                    when(did_reset.map(|v| !*v), move || {
                        sapid::empty_state()
                            .theme(empty_theme.clone())
                            .size(controls::size_value(size.get().as_deref().unwrap_or("lg")))
                            .style(align.map(|s| Style {
                                align: Some(match s.as_deref() {
                                    Some("start") => Align::Start,
                                    Some("end") => Align::End,
                                    _ => Align::Center,
                                }),
                                ..Style::new()
                            }))
                            .illustration(
                                glory::ui::graphic()
                                    .drawing(sapid::Icon::Search.drawing(empty_theme.muted))
                                    .style(background.map({
                                        let soft = Color(241,243,245,255);
                                        move |on| Style {
                                            background: Some(if *on {
                                                soft
                                            } else {
                                                Color(0, 0, 0, 0)
                                            }),
                                            ..Style::new()
                                                .width(Length::Px(if *on {80.} else {48.}))
                                                .height(Length::Px(if *on {80.} else {48.}))
                                                .padding(Edges::all(if *on {18.} else {2.}))
                                                .radius(40.)
                                        }
                                    })),
                            )
                            .title("No results found")
                            .description(
                                "We couldn't find anything matching your search. Try adjusting your filters or searching with different keywords to see more results.",
                            )
                            .actions(sapid::button().theme(empty_theme.clone()).variant(sapid::ButtonVariant::Secondary).label("Reset filters").on_press(move || did_reset.revise(|v| *v = true)))
                    }),
                    when(did_reset, move || {
                        sapid::stack().theme(ready_theme.clone()).align(Align::Center).gap(12.).fill((sapid::text().theme(ready_theme.clone()).content("Filters reset."),sapid::button().theme(ready_theme.clone()).variant(sapid::ButtonVariant::Secondary).label("Restore filters").on_press(move || did_reset.revise(|v| *v=false))))
                    }),
                ))
            });
        view()
            .class("site-configurator site-display-configurator site-empty-state-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::size_slider(size, &theme),
                    controls::option_segments(
                        "empty_state-align",
                        "Align",
                        align,
                        &[("center", "Center"), ("start", "Left"), ("end", "Right")],
                        &theme,
                    ),
                    controls::switch("With indicator background", background, &theme),
                ],
            ))
            .show_in(scope)
    })
}
