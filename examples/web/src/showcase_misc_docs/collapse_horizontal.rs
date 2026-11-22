use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let expanded = scope.cage(false);
        sapid::collapsible("horizontal")
            .theme(theme.clone()).horizontal(true).open(expanded)
            .label(expanded.map(|v| if *v { "Collapse".to_string() } else { "Expand".to_string() }))
            .on_open_change(move |v| expanded.revise(|value| *value = v))
            .class("showcase-horizontal-collapse")
            .content(move || text().class("showcase-horizontal-collapse-copy").content("Follow the walking trail along the lake. It curves through the trees and ends at a viewpoint above the old ferry landing."))
            .show_in(scope)
    })
}
