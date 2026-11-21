use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let ignore_case = scope.cage(false);
        let whole_words = scope.cage(false);
        let options = scope.bond(move || vec![(ignore_case.get(), whole_words.get())]);
        let preview_theme = theme.clone();
        let preview = each(options).key(|options| *options).seed(move |(ignore_case, whole_words)| {
            sapid::highlight().theme(preview_theme.clone())
                .content("Consistent design makes important details easy to find. A shared design language helps every team. Compare DESIGN and redesigns.")
                .queries(["design".to_owned(), "important".to_owned()])
                .ignore_ascii_case(ignore_case).whole_words(whole_words)
        });
        view()
            .class("site-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::switch("Ignore ASCII case", ignore_case, &theme),
                    controls::switch("Whole words", whole_words, &theme),
                ],
            ))
            .show_in(scope)
    })
}
