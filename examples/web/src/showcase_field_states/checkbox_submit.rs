use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let accepted = scope.cage(false);
        let invalid = scope.cage(false);
        let error = invalid.map(|v| {
            if *v {
                "You must accept terms and conditions".to_owned()
            } else {
                String::new()
            }
        });
        sapid::form("checkbox-submit-form")
            .theme(theme.clone())
            .on_submit(move |_| {
                let ok = accepted.get();
                invalid.revise(|v| *v = !ok);
                if ok {
                    super::field_form_feedback::submit(serde_json::json!({"terms":true}));
                }
            })
            .fill(
                sapid::stack()
                    .class("site-field-state-example site-example-validation-form")
                    .gap(16.)
                    .style(
                        Style::new()
                            .width(Length::Percent(100.))
                            .max_width(Limit::Px(340.)),
                    )
                    .fill((
                        sapid::field("terms-checkbox")
                            .form_name("terms")
                            .invalid(invalid)
                            .error(error)
                            .control(move |field| {
                                sapid::checkbox()
                                    .field(field)
                                    .label("I accept the terms and conditions")
                                    .checked(accepted)
                                    .on_checked_change(move |next| {
                                        accepted.revise(|v| *v = next);
                                        invalid.revise(|v| *v = false);
                                    })
                            }),
                        sapid::submit_button().theme(theme.clone()).label("Submit"),
                    )),
            )
            .show_in(scope)
    })
}
