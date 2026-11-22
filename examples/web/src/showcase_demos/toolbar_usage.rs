use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    format_toolbar(theme, "document-tools", ToolbarExample::Basic)
}

pub(crate) enum ToolbarExample {
    Basic,
    Groups,
    Contrast(bool),
    WithActions,
    Disabled,
}
pub(crate) fn format_toolbar(
    theme: sapid::Theme,
    id: &'static str,
    example: ToolbarExample,
) -> impl Widget {
    widget(move |scope| {
        let mut toolbar = sapid::toolbar(id)
            .theme(theme.clone())
            .name("Document formatting")
            .size(sapid::Size::Small)
            .class(if matches!(example, ToolbarExample::Contrast(true)) {
                "sapid-demo-format-toolbar contrast-auto"
            } else if matches!(example, ToolbarExample::Contrast(false)) {
                "sapid-demo-format-toolbar contrast-plain"
            } else {
                "sapid-demo-format-toolbar"
            });
        let alignment = scope.cage(Some(0usize));
        let disabled = matches!(example, ToolbarExample::Disabled);
        if matches!(example, ToolbarExample::WithActions) {
            let actions = ["Save", "Cancel"].map(|label| {
                let active = scope.cage(false);
                sapid::button()
                    .theme(theme.clone())
                    .label(label)
                    .class("site-toolbar-text-action")
                    .variant(sapid::ButtonVariant::Secondary)
                    .pressed(active)
                    .on_press(move || active.revise(|v| *v = !*v))
            });
            let formatting = [("Bold", "B"), ("Italic", "I")].map(|(label, glyph)| {
                let active = scope.cage(false);
                sapid::button()
                    .theme(theme.clone())
                    .name(label)
                    .label(glyph)
                    .variant(sapid::ButtonVariant::Subtle)
                    .pressed(active)
                    .on_press(move || active.revise(|v| *v = !*v))
            });
            return toolbar
                .group(sapid::toolbar_group("format").buttons(formatting))
                .group(sapid::toolbar_group("edits").buttons(actions))
                .show_in(scope);
        }
        let mut format = sapid::toolbar_group("format").name("Text formatting");
        for (i, (label, glyph)) in [("Bold", "B"), ("Italic", "I"), ("Underline", "U")]
            .into_iter()
            .enumerate()
        {
            let state = scope.cage(false);
            format = format.button(
                sapid::button()
                    .theme(theme.clone())
                    .name(label)
                    .class(format!("sapid-demo-format-{i}"))
                    .variant(sapid::ButtonVariant::Subtle)
                    .pressed(state)
                    .enabled(!disabled || i != 1)
                    .fill(glory::ui::text().content(glyph))
                    .on_press(move || state.revise(|v| *v = !*v)),
            );
        }
        let mut align = sapid::toolbar_group("alignment").name("Text alignment");
        for (i, label) in ["Align left", "Align center", "Align right"]
            .into_iter()
            .enumerate()
        {
            let ink = theme.foreground;
            align = align.button(
                sapid::button()
                    .theme(theme.clone())
                    .name(label)
                    .variant(sapid::ButtonVariant::Subtle)
                    .enabled(!disabled || i == 0)
                    .style(
                        Style::new()
                            .width(Length::Px(30.))
                            .height(Length::Px(30.))
                            .min_height(Length::Px(30.))
                            .padding(glory::style::Edges::all(0.)),
                    )
                    .pressed(alignment.map(move |v| *v == Some(i)))
                    .fill(
                        glory::ui::graphic()
                            .drawing(align_icon(i, ink))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                    )
                    .on_press(move || {
                        alignment.revise(|v| *v = if *v == Some(i) { None } else { Some(i) })
                    }),
            );
        }
        toolbar = toolbar.group(format).group(align);
        if matches!(example, ToolbarExample::Basic) {
            toolbar = toolbar.group(
                sapid::toolbar_group("links").button(
                    sapid::button()
                        .theme(theme.clone())
                        .variant(sapid::ButtonVariant::Subtle)
                        .name("Toolbar documentation")
                        .fill(
                            glory::ui::graphic()
                                .drawing(sapid::Icon::ArrowRight.drawing(theme.foreground))
                                .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                        )
                        .on_press(|| {
                            #[cfg(target_arch = "wasm32")]
                            if let Some(w) = web_sys::window() {
                                let _ = w.location().set_hash("toolbar-groups");
                            }
                        }),
                ),
            );
        }
        toolbar.show_in(scope)
    })
}
fn align_icon(i: usize, color: Color) -> glory::render::GraphicProps {
    use glory::render::{GraphicPoint as P, PathCommand as C};
    let mut graphic = sapid::Icon::More.drawing(color);
    graphic.commands = [
        (4., 4., 20.),
        (
            9.,
            if i == 2 {
                9.
            } else if i == 1 {
                6.5
            } else {
                4.
            },
            if i == 0 {
                15.
            } else if i == 1 {
                17.5
            } else {
                20.
            },
        ),
        (14., 4., 20.),
        (
            19.,
            if i == 2 {
                9.
            } else if i == 1 {
                6.5
            } else {
                4.
            },
            if i == 0 {
                15.
            } else if i == 1 {
                17.5
            } else {
                20.
            },
        ),
    ]
    .into_iter()
    .flat_map(|(y, x, end)| [C::Move(P::new(x, y)), C::Line(P::new(end, y))])
    .collect();
    graphic
}
