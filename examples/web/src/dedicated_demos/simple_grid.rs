use glory::prelude::*;
use glory::style::{GridPlacement, GridTrack, GridTracks};

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let columns = scope.cage(3.);
        let spacing = scope.cage(Some("md".to_owned()));
        let vertical = scope.cage(Some("md".to_owned()));
        let palette = theme.clone();
        let layout = scope.bond(move || {
            Style::new()
                .width(Length::Percent(100.))
                .grid_columns(GridTracks::Tracks(vec![
                    GridTrack::Fr(1.);
                    columns.get() as usize
                ]))
                .gap(palette.spacing(controls::size_value(
                    spacing.get().as_deref().unwrap_or("md"),
                )))
        });
        let preview = sapid::simple_grid("project-cards")
            .theme(theme.clone())
            .columns(3)
            .class(vertical.map(|s| format!("site-grid-row-gap-{}", s.as_deref().unwrap_or("md"))))
            .style(layout)
            .items((1..=5).map(|i| {
                sapid::grid_item(i.to_string())
                    .style(
                        Style::new()
                            .grid_column(GridPlacement::Auto)
                            .grid_row(GridPlacement::Auto),
                    )
                    .fill(controls::grid_demo_cell(i.to_string(), &theme))
            }));
        view()
            .class("site-configurator site-layout-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::range("grid-columns", "Cols", columns, 1., 6., 1., &theme),
                    controls::scale_slider("grid-column-spacing", "Spacing", spacing, &theme),
                    controls::scale_slider(
                        "grid-row-spacing",
                        "Vertical spacing",
                        vertical,
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
