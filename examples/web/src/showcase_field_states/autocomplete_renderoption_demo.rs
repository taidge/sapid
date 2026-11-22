use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill({
            let mut picker =
                sapid::autocomplete("autocomplete-employees").placeholder("Search for employee");
            for (id, name, position) in [
                ("emily", "Emily Johnson", "Engineering"),
                ("ava", "Ava Rodriguez", "Design"),
                ("olivia", "Olivia Chen", "Research"),
                ("ethan", "Ethan Barnes", "Operations"),
                ("mason", "Mason Taylor", "Support"),
            ] {
                picker = picker
                    .option(sapid::select_option(id, name))
                    .option_content(
                        id,
                        sapid::SelectOptionContent::default().description(position),
                    );
            }
            let mut picker = Some(picker);
            sapid::field("autocomplete-employees-label")
                .theme(theme.clone())
                .label("Employee of the month")
                .control(move |field| picker.take().expect("mount once").field(field))
        })
}
