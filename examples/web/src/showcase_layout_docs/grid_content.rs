use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::grid("content-column")
        .theme(theme.clone())
        .column_tracks([
            glory::style::GridTrack::Auto,
            glory::style::GridTrack::Percent(50.),
            glory::style::GridTrack::Fr(1.),
        ])
        .gap(16.)
        .items(
            [("content", "fit content"), ("half", "2")]
                .into_iter()
                .map(|(key, label)| {
                    sapid::grid_item(key).fill(controls::grid_demo_cell(label.to_owned(), &theme))
                }),
        )
}

#[allow(unused_imports)]
mod controls {
    use glory::prelude::*;
    use glory::style::{Color, FlexWrap};
    use sapid::{Size, Theme};

    pub fn grid_demo_cell(label: String, theme: &Theme) -> impl Widget {
        let (background, color) = if theme.scheme == sapid::ColorScheme::Dark {
            (theme.surface, theme.muted)
        } else {
            (
                glory::style::Color(231, 245, 255, 255),
                glory::style::Color(34, 139, 230, 255),
            )
        };
        view()
            .style(Style {
                justify: Some(Justify::Center),
                align: Some(Align::Center),
                background: Some(background),
                ..Style::new()
                    .display(Display::Flex)
                    .padding(glory::style::Edges::all(16.))
                    .height(Length::Percent(100.))
            })
            .fill(
                text()
                    .style(Style {
                        font_size: Some(20.),
                        color: Some(color),
                        ..Style::new().font_weight(700.)
                    })
                    .content(label),
            )
    }
}
