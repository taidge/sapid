use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let media = option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets");
        let images = [
            ("lake", "alpine-lake-photo.png", "Alpine lake"),
            ("ridge", "mountain-evening.svg", "Mountain evening"),
        ]
        .map(|(key, file, caption)| {
            sapid::lightbox_image(key, format!("{media}/{file}"), caption).caption(caption)
        })
        .to_vec();
        let chosen = scope.cage(Some("pop".to_owned()));
        let options = [
            ("pop", "Pop"),
            ("fade", "Fade"),
            ("scale", "Scale"),
            ("slide", "Slide up"),
        ];
        let effect = chosen.map(|v| match v.as_deref() {
            Some("fade") => sapid::TransitionEffect::Fade,
            Some("scale") => sapid::TransitionEffect::Scale,
            Some("slide") => sapid::TransitionEffect::SlideUp,
            _ => sapid::TransitionEffect::Pop,
        });
        sapid::group()
            .theme(theme.clone())
            .fill((
                sapid::native_select("gallery-transition")
                    .theme(theme.clone())
                    .label("Transition")
                    .options(options.map(|(k, n)| sapid::select_option(k, n)))
                    .value(chosen)
                    .on_value_change(move |v| chosen.revise(|old| *old = v)),
                sapid::lightbox("transition-gallery")
                    .theme(theme.clone())
                    .label("Open lightbox")
                    .images(images)
                    .transition(effect, 240),
            ))
            .show_in(scope)
    })
}
