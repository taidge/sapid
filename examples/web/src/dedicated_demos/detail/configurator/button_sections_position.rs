use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let justify = scope.cage("center".to_owned());
        let row_style = justify.map(|v| Style {
            justify: Some(controls::layout_justify(v)),
            ..Style::new().width(Length::Percent(100.))
        });
        let button = || {
            sapid::button()
                .theme(theme.clone())
                .variant(sapid::ButtonVariant::Secondary)
                .label("Button label")
                .style(row_style)
        };
        let icon = || {
            use glory::render::{GraphicPoint as P, PathCommand as Path};
            let mut image = sapid::Icon::Check.drawing(theme.foreground);
            image.commands = vec![
                Path::Move(P::new(2., 4.)),
                Path::Line(P::new(22., 4.)),
                Path::Line(P::new(22., 20.)),
                Path::Line(P::new(2., 20.)),
                Path::Close,
                Path::Move(P::new(2., 16.)),
                Path::Line(P::new(8., 10.)),
                Path::Line(P::new(18., 20.)),
                Path::Move(P::new(14., 16.)),
                Path::Line(P::new(17., 13.)),
                Path::Line(P::new(22., 18.)),
                Path::Move(P::new(15., 8.)),
                Path::Line(P::new(16., 8.)),
            ];
            glory::ui::graphic()
                .drawing(image)
                .style(Style::new().width(Length::Px(14.)).height(Length::Px(14.)))
        };
        let preview = sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .style(Style::new().width(Length::Percent(100.)))
            .fill((
                button().left_section(icon()).right_section(icon()),
                button().left_section(icon()),
                button().right_section(icon()),
                button()
                    .left_section(text().content(""))
                    .right_section(icon()),
            ));
        view()
            .class("site-configurator site-action-configurator")
            .fill(controls::playground(
                Box::new(preview),
                vec![controls::segmented_choice(
                    "sections-position",
                    "Justify",
                    justify,
                    &[("center", "Center"), ("between", "Space-between")],
                    &theme,
                )],
            ))
            .show_in(scope)
    })
}
