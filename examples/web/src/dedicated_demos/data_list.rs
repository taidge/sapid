use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(Some("md".to_owned()));
        let orientation = scope.cage(Some("vertical".to_owned()));
        let divider = scope.cage(false);
        let palette = theme.clone();
        let preview = each(scope.bond(move || vec![(size.get(), orientation.get())])).key(Clone::clone).children(move |_| { let theme=palette.clone();

sapid::data_list()
        .theme(theme.clone())
        .size(controls::size_value(size.get().as_deref().unwrap_or("md"))).orientation(if orientation.get().as_deref()==Some("vertical"){sapid::DataListOrientation::Vertical}else{sapid::DataListOrientation::Horizontal}).with_divider(divider).label_width(96.)
        .name("Project facts")
        .item(sapid::data_list_item().term("Name").fill(sapid::text().theme(theme.clone()).content("John Doe")))
        .item(sapid::data_list_item().term("Email").fill(sapid::text().theme(theme.clone()).content("john@example.com")))
        .item(sapid::data_list_item().term("Role").fill(sapid::text().theme(theme.clone()).content("Software Engineer")))
        .item(sapid::data_list_item().term("Location").fill(sapid::text().theme(theme.clone()).content("San Francisco, CA")))
});
        view()
            .class("site-configurator site-display-configurator site-data-list-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::size_slider(size, &theme),
                    controls::option_segments(
                        "data_list-orientation",
                        "Orientation",
                        orientation,
                        &[("horizontal", "Horizontal"), ("vertical", "Vertical")],
                        &theme,
                    ),
                    controls::switch("With divider", divider, &theme),
                ],
            ))
            .show_in(scope)
    })
}
