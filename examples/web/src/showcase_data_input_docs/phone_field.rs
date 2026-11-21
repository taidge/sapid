use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let loading = scope.cage(false);
        let readonly = scope.cage(false);
        let disabled = scope.cage(false);
        let show_error = scope.cage(false);
        let toggle = |label, state: glory::Cage<bool>| {
            sapid::checkbox()
                .theme(theme.clone())
                .label(label)
                .checked(state)
                .on_checked_change(move |value| state.revise(|old| *old = value))
        };
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    toggle("Loading", loading),
                    toggle("Read only", readonly),
                    toggle("Disabled", disabled),
                    toggle("Show error", show_error),
                )),
                sapid::form("support-contact").theme(theme.clone()).fill(
                    sapid::field("support-phone-field")
                        .theme(theme.clone())
                        .label("Support phone")
                        .description("Include the extension for the support desk.")
                        .form_name("support_phone")
                        .required(true)
                        .enabled(scope.bond(move || !disabled.get()))
                        .readonly(readonly)
                        .error(scope.bond(move || {
                            if show_error.get() {
                                "Check the support number."
                            } else {
                                ""
                            }
                            .to_owned()
                        }))
                        .control(move |field| {
                            sapid::phone_input("support-region")
                                .field(field)
                                .country_label("Support region")
                                .regions([
                                    ("GB", "United Kingdom"),
                                    ("US", "United States"),
                                    ("CA", "Canada"),
                                ])
                                .default_value(sapid::PhoneDraft::new(
                                    "GB",
                                    "020 7946 0018 ext. 42",
                                ))
                                .loading(loading)
                        }),
                ),
            ))
            .show_in(scope)
    })
}
