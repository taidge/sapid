use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack()
        .class("site-field-state-example")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill(
            sapid::field("native-dividers-field")
                .theme(theme)
                .label("With dividers")
                .control(|field| {
                    sapid::native_select("native-dividers")
                        .field(field)
                        .option(sapid::select_option("select-library", "Select library"))
                        .native_entry(glory::render::SelectEntry::Divider)
                        .group(
                            sapid::select_group("Frontend libraries").options(
                                ["React", "Angular", "Vue"]
                                    .map(|name| sapid::select_option(name.to_lowercase(), name)),
                            ),
                        )
                        .native_entry(glory::render::SelectEntry::Divider)
                        .group(
                            sapid::select_group("Backend libraries").options(
                                ["Express", "Koa", "Django"]
                                    .map(|name| sapid::select_option(name.to_lowercase(), name)),
                            ),
                        )
                        .default_value(Some("select-library".into()))
                }),
        )
}

#[cfg(test)]
mod tests {
    #[test]
    fn dividers_render_inside_the_real_select() {
        let html = glory::ssr::snapshot(super::demo(sapid::Theme::default()))
            .unwrap()
            .html;
        assert_eq!(html.matches("<hr>").count(), 2);
        assert!(html.contains("</option><hr><optgroup"));
        assert!(html.contains("</optgroup><hr><optgroup"));
    }
}
