use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::combobox("styles-combo")
        .theme(theme)
        .name("Pick value")
        .open(true)
        .header("Combobox header")
        .footer("Combobox footer")
        .empty_group_message("Nothing found in this group...")
        .group(sapid::select_group("Third group"))
        .group(
            sapid::select_group("Frontend")
                .options(["React", "Angular"].map(|v| sapid::select_option(v, v))),
        )
        .group(
            sapid::select_group("Backend")
                .options(["Node", "Django"].map(|v| sapid::select_option(v, v))),
        );
    crate::showcase_style_explorer::explorer(
        "combobox-styles",
        Box::new(
            sapid::stack()
                .class("site-field-state-example")
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(340.)),
                )
                .fill(preview),
        ),
        &[
            (
                "options",
                "`Combobox.Options` component",
                "[role=\"listbox\"]",
            ),
            (
                "dropdown",
                "`Combobox.Dropdown` component",
                "[role=\"listbox\"]",
            ),
            ("option", "`Combobox.Option` component", "[role=\"option\"]"),
            ("search", "`Combobox.Search` input", "input.sapid-combobox"),
            (
                "empty",
                "`Combobox.Empty` component",
                ".sapid-combobox-empty",
            ),
            (
                "header",
                "`Combobox.Header` component",
                ".sapid-combobox-header",
            ),
            (
                "footer",
                "`Combobox.Footer` component",
                ".sapid-combobox-footer",
            ),
            ("group", "`Combobox.Group` component", "[role=\"group\"]"),
            (
                "groupLabel",
                "Label of `Combobox.Group` component",
                "[role=\"group\"] > [data-gly-kind=\"text\"]",
            ),
        ],
    )
}
