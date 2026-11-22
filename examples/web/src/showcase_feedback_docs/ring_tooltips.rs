use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(None::<usize>);
        let names = ["Documents", "Photos", "Other"];
        let sizes = [35., 25., 15.];
        let colors = [
            Color(21, 170, 191, 255),
            Color(230, 73, 128, 255),
            Color(253, 126, 20, 255),
        ];
        let progress = sapid::ring_progress(75.)
            .theme(theme.clone())
            .name("Storage by category")
            .size(150.)
            .thickness(14.)
            .sections((0..3).map(|i| sapid::ProgressSection::new(sizes[i], colors[i])))
            .on_section_hover(move |index| selected.revise(|value| *value = index));
        sapid::stack()
            .theme(theme.clone())
            .align(Align::Center)
            .gap(10.)
            .fill((
                progress,
                sapid::text()
                    .theme(theme.clone())
                    .content(selected.map(move |i| {
                        i.map(|i| format!("{}: {:.0}%", names[i], sizes[i]))
                            .unwrap_or_else(|| "Hover a section to see its share.".to_owned())
                    })),
            ))
            .show_in(scope)
    })
}
