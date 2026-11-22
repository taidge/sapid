use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    view().fill(
        ["Error: ", "Warning: ", "Success: "]
            .into_iter()
            .zip([
                ("Invalid input. ", Color(250, 180, 185, 255)),
                ("Check this field. ", Color(255, 227, 135, 255)),
                ("All tests passed.", Color(181, 229, 190, 255)),
            ])
            .map(|(term, (sentence, background))| {
                let mut palette = theme.clone();
                palette.accent_soft = background;
                palette.accent_text = theme.contrasting_foreground(background, theme.foreground);
                sapid::highlight()
                    .theme(palette)
                    .content(format!("{term}{sentence}"))
                    .query(term.trim())
            })
            .collect::<Vec<_>>(),
    )
}
