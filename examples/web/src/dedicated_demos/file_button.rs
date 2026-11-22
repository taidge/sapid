use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let files = scope.cage(Vec::<glory::render::FileInfo>::new());
        let feedback_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .class("site-standalone-action")
            .gap(12.)
            .fill((
                sapid::file_button()
                    .theme(theme.clone())
                    .style(Style {
                        background: Some(theme.accent),
                        color: Some(theme.on_accent),
                        border_color: Some(theme.accent),
                        ..Style::new().radius(4.)
                    })
                    .name("Upload profile image")
                    .label("Upload image")
                    .configuration(glory::render::FileConfig {
                        filters: vec![
                            glory::render::FileFilter::Mime("image/png".into()),
                            glory::render::FileFilter::Mime("image/jpeg".into()),
                        ],
                        ..Default::default()
                    })
                    .files_ref(files),
                when(files.map(|v| !v.is_empty()), move || {
                    sapid::text()
                        .theme(feedback_theme.clone())
                        .content(files.map(|v| {
                            v.first()
                                .map(|f| format!("Selected: {}", f.name))
                                .unwrap_or_default()
                        }))
                }),
            ))
            .show_in(scope)
    })
}
