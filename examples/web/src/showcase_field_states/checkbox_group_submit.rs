use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(Vec::<String>::new());
        let invalid = scope.cage(false);
        let error = invalid.map(|v| {
            if *v {
                "Select at least one framework".to_owned()
            } else {
                String::new()
            }
        });
        sapid::form("framework-checkbox-submit")
            .theme(theme.clone())
            .on_submit(move |_| {
                let values = selected.get();
                invalid.revise(|v| *v = values.is_empty());
                if !values.is_empty() {
                    super::field_form_feedback::submit(serde_json::json!({"frameworks":values}));
                }
            })
            .fill(
                sapid::stack()
                    .class("site-example-validation-form site-framework-checkbox-form")
                    .gap(16.)
                    .fill((
                        sapid::field("framework-checkboxes")
                            .label("Select your favorite frameworks/libraries")
                            .with_asterisk(true)
                            .invalid(invalid)
                            .error(error)
                            .control(move |field| {
                                sapid::checkbox_group("checkbox-form-options")
                                    .field(field)
                                    .selected(selected)
                                    .items(
                                        [
                                            ("react", "React"),
                                            ("svelte", "Svelte"),
                                            ("ng", "Angular"),
                                            ("vue", "Vue"),
                                        ]
                                        .map(|(key, label)| sapid::checkbox_item(key, label)),
                                    )
                                    .on_selected_change(move |next| {
                                        selected.revise(|v| *v = next);
                                        invalid.revise(|v| *v = false);
                                    })
                            }),
                        sapid::submit_button().theme(theme.clone()).label("Submit"),
                    )),
            )
            .show_in(scope)
    })
}
