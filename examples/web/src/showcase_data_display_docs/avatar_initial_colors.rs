use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let colors = [
        Color(250, 193, 202, 255),
        Color(199, 222, 252, 255),
        Color(222, 240, 190, 255),
    ];
    sapid::group()
        .theme(theme.clone())
        .justify(Justify::Center)
        .gap(12.)
        .fill(
            [
                "John Davis",
                "Jane Moon",
                "Alex Lee",
                "Sam Chen",
                "Mira King",
                "Kai Kim",
                "Tim Smith",
            ]
            .map(|name| {
                let seed = name.bytes().fold(0usize, |n, b| n + usize::from(b));
                let fill = colors[seed % colors.len()];
                sapid::avatar(name)
                    .fallback(
                        name.split_whitespace()
                            .filter_map(|part| part.chars().next())
                            .collect::<String>(),
                    )
                    .theme(theme.clone())
                    .style(Style {
                        background: Some(fill),
                        color: Some(theme.contrasting_foreground(fill, theme.foreground)),
                        ..Style::new()
                    })
            })
            .into_iter()
            .collect::<Vec<_>>(),
        )
}
