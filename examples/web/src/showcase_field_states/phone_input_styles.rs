use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview_theme = theme.clone();
    let preview = widget(move |scope| {
        let numbers = scope.cage(sapid::PhoneDraft::new("US", "+1"));
        let countries = [
            ("US", "United States"),
            ("GB", "United Kingdom"),
            ("DE", "Germany"),
            ("FR", "France"),
            ("CA", "Canada"),
        ];
        let country = numbers.map(move |draft| {
            countries
                .iter()
                .find(|(code, _)| *code == draft.region)
                .map_or("Select country", |(_, name)| *name)
                .to_owned()
        });
        sapid::field("phone-input-styles-field")
            .theme(preview_theme.clone())
            .label("Phone number")
            .control(move |field| {
                sapid::phone_input("styles-phone")
                    .searchable_region(true)
                    .default_region_open(true)
                    .regions(countries)
                    .country_label(country)
                    .value(numbers)
                    .on_value_change(move |next| numbers.revise(|v| *v = next))
                    .placeholder("Your phone number")
                    .field(field)
            })
            .show_in(scope)
    });
    crate::showcase_style_explorer::explorer(
        "phone-input-styles",
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
            ("wrapper", "Root element of the Input", ".sapid-phone-input"),
            ("input", "Input element", "input[type=\"tel\"]"),
            ("section", "Left and right sections", ".sapid-phone-region"),
            (
                "bottomSection",
                "Bottom section element, rendered inside the input border at the bottom",
                "",
            ),
            ("root", "Root element", ".sapid-field"),
            ("label", "Label element", ".sapid-field-label"),
            (
                "required",
                "Required asterisk element, rendered inside label",
                ".sapid-field-label.is-required::after",
            ),
            (
                "description",
                "Description element",
                ".sapid-field-description",
            ),
            ("error", "Error element", ".sapid-field-error"),
            (
                "success",
                "Success element",
                ".site-valid-field .sapid-field-description",
            ),
            (
                "countryButton",
                "Country selector button in leftSection",
                ".sapid-phone-region .sapid-popover-trigger",
            ),
            (
                "countryButtonFlag",
                "Flag wrapper inside country button",
                ".sapid-phone-country-flag",
            ),
            (
                "countryButtonCode",
                "Country ISO2 code text inside country button",
                "",
            ),
            (
                "dropdown",
                "Combobox dropdown",
                ".sapid-phone-region [role=listbox]",
            ),
            (
                "search",
                "Search input in dropdown",
                ".sapid-phone-region input.sapid-combobox",
            ),
            (
                "countryOption",
                "Country option content wrapper in dropdown",
                ".sapid-phone-region [role=option]",
            ),
            (
                "optionName",
                "Country name text in option",
                ".sapid-phone-region .sapid-combobox-option-label",
            ),
            (
                "optionDialCode",
                "Dial code text in option",
                ".sapid-phone-region .sapid-combobox-option-trailing",
            ),
            (
                "divider",
                "Divider between preferred and other countries",
                ".sapid-phone-region .sapid-combobox-option:nth-child(3)::after",
            ),
        ],
    )
}
