use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage("pdf".to_owned());
        let receipt = scope.cage(String::new());
        sapid::form("report-format")
            .theme(theme.clone())
            .on_submit(move |_| {
                receipt.revise(|text| *text = format!("Submitted format: {}", current.get()))
            })
            .fill((
                sapid::segmented_control("report-format-options")
                    .theme(theme.clone())
                    .label("Report format")
                    .form_name("format")
                    .default_value("pdf")
                    .segments([
                        sapid::segment("pdf").label("PDF"),
                        sapid::segment("csv").label("Spreadsheet"),
                        sapid::segment("html").label("Web page"),
                    ])
                    .on_value_change(move |next| current.revise(|value| *value = next)),
                sapid::submit_button()
                    .theme(theme.clone())
                    .label("Submit format"),
                sapid::text().theme(theme.clone()).content(receipt),
            ))
            .show_in(scope)
    })
}
