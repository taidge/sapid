use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let size = scope.cage(1.);
        let length = scope.cage(4.);
        let kind = scope.cage(Some("alphanumeric".to_owned()));
        let mask = scope.cage(false);
        let disabled = scope.cage(false);
        let error = scope.cage(false);
        let placeholder = scope.cage("○".to_owned());
        let code = scope.cage(String::new());
        let status = scope.cage(String::new());
        let palette = theme.clone();
        let preview = each(
            scope.bond(move || vec![(size.get() as usize, length.get() as usize, kind.get())]),
        )
        .key(Clone::clone)
        .children(move |row| {
            let (index, count, kind) = row.value.get();
            let digits = kind.as_deref() == Some("number");
            let accepted = code.map(move |text| {
                text.chars()
                    .filter(|c| !digits || c.is_ascii_digit())
                    .take(count)
                    .collect::<String>()
            });
            sapid::stack().theme(palette.clone()).fill((
                sapid::input_otp()
                    .theme(palette.clone())
                    .size(controls::step_size(index as f64))
                    .name("Verification code")
                    .length(count)
                    .alphabet(if digits {
                        sapid::PinAlphabet::Digits
                    } else {
                        sapid::PinAlphabet::AsciiAlphanumeric
                    })
                    .masked(mask)
                    .invalid(error)
                    .placeholder(placeholder)
                    .enabled(disabled.map(|v| !*v))
                    .value(accepted)
                    .on_value_change(move |next| {
                        code.revise(|v| *v = next);
                        status.revise(String::clear);
                    })
                    .on_complete(move |_| status.revise(|v| *v = "Code ready to verify".to_owned()))
                    .on_reject(move |_| {
                        status.revise(|v| {
                            *v = format!(
                                "Enter {count} {}",
                                if digits {
                                    "digits"
                                } else {
                                    "letters or digits"
                                }
                            )
                        })
                    }),
                sapid::visually_hidden().content(status),
            ))
        });
        view()
            .class("site-configurator site-field-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![
                    controls::field_scale("pin-size".into(), "Size", size, &theme),
                    controls::range("pin-length", "Length", length, 1., 5., 1., &theme),
                    controls::switch("Mask", mask, &theme),
                    controls::field_setting(
                        "pin-placeholder".into(),
                        "Placeholder",
                        placeholder,
                        &theme,
                    ),
                    controls::switch("Disabled", disabled, &theme),
                    controls::switch("Error", error, &theme),
                    controls::select(
                        "pin-kind",
                        "Type",
                        kind,
                        &[("alphanumeric", "Alphanumeric"), ("number", "Number")],
                        &theme,
                    ),
                ],
            ))
            .show_in(scope)
    })
}
