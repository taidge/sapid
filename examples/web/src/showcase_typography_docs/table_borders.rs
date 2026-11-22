use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let stripes = scope.cage(true);
        let hover = scope.cage(true);
        let border = scope.cage(true);
        let columns = scope.cage(true);
        let rows = scope.cage(true);
        let palette = theme.clone();
        let preview = each(scope.bond(move || {
            vec![(
                stripes.get(),
                hover.get(),
                border.get(),
                columns.get(),
                rows.get(),
            )]
        }))
        .key(Clone::clone)
        .children(move |_| {
            let theme = palette.clone();

            sapid::scroll_area("bordered-project-scroll")
                .axes(glory::render::ScrollAxes::Both)
                .theme(theme.clone())
                .name("Bordered project tracker")
                .style(
                    Style::new()
                        .height(Length::Px(280.))
                        .max_width(glory::style::Limit::Percent(100.)),
                )
                .content(move |_| {
                    projects(&theme, "bordered-projects")
                        .style(Style::new().min_width(Length::Px(560.)))
                        .striped(stripes.get())
                        .highlight_on_hover(hover.get())
                        .with_table_border(border.get())
                        .with_column_borders(columns.get())
                        .with_row_borders(rows.get())
                        .tabular_nums(true)
                })
        });
        view()
            .class("site-configurator site-display-configurator site-table-borders-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Striped", stripes, &theme),
                    controls::switch("Highlight on hover", hover, &theme),
                    controls::switch("With table border", border, &theme),
                    controls::switch("With column borders", columns, &theme),
                    controls::switch("With row borders", rows, &theme),
                ],
            ))
            .show_in(scope)
    })
}

fn projects(theme: &sapid::Theme, key: &str) -> sapid::Table {
    sapid::table(key)
        .theme(theme.clone())
        .caption("Studio projects")
        .head(sapid::table_head().row(sapid::table_row("head").cells([
            sapid::table_header_cell("project").text("Project"),
            sapid::table_header_cell("owner").text("Owner"),
            sapid::table_header_cell("status").text("Status"),
            sapid::table_header_cell("due").text("Due date"),
        ])))
        .body(
            sapid::table_body().rows(
                [
                    (
                        "website",
                        "Website redesign",
                        "Ava Patel",
                        "In review",
                        "Nov 18",
                    ),
                    (
                        "library",
                        "Component library",
                        "Jamie Chen",
                        "In progress",
                        "Nov 24",
                    ),
                    (
                        "research",
                        "Research notes",
                        "Sam Rivera",
                        "Ready",
                        "Nov 28",
                    ),
                    (
                        "mobile",
                        "Mobile prototype",
                        "Robin Park",
                        "Planning",
                        "Dec 02",
                    ),
                ]
                .into_iter()
                .map(|(key, project, owner, status, due)| {
                    sapid::table_row(key).cells([
                        sapid::table_row_header("project").text(project),
                        sapid::table_cell("owner").text(owner),
                        sapid::table_cell("status").text(status),
                        sapid::table_cell("due").text(due),
                    ])
                }),
            ),
        )
}

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn playground(preview: Box<dyn Widget>, controls: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
        Box::new(
            view()
                .class("site-button-playground")
                .style(
                    Style::new()
                        .display(Display::Flex)
                        .flex_wrap(FlexWrap::Wrap)
                        .gap(24.0),
                )
                .fill((
                    col()
                        .class("site-playground-preview")
                        .style(Style {
                            grow: Some(1.0),
                            align: Some(Align::Center),
                            justify: Some(Justify::Center),
                            ..Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .min_width(Length::Px(180.0))
                        })
                        .fill(preview),
                    col()
                        .class("site-playground-controls")
                        .style(
                            Style::new()
                                .display(Display::Flex)
                                .flex_direction(FlexDirection::Column)
                                .gap(16.0)
                                .width(Length::Px(240.0))
                                .max_width(Limit::Percent(100.0)),
                        )
                        .fill(controls),
                )),
        )
    }

    pub fn switch(label: &str, value: Cage<bool>, theme: &Theme) -> Box<dyn Widget> {
        Box::new(
            sapid::switch()
                .theme(theme.clone())
                .size(Size::Small)
                .with_thumb_indicator(true)
                .label(label.to_owned())
                .checked(value)
                .on_checked_change(move |next| value.revise(|current| *current = next)),
        )
    }
}
