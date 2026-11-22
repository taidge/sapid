use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let preview = sapid::field("cascader-styles-field")
        .theme(theme.clone())
        .label("Location")
        .description("Description")
        .required(true)
        .control(move |field| {
            sapid::cascader("styles-cascader")
                .nodes(vec![
                    sapid::TreeNode::new("asia", "Asia").kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("jp", "Japan").parent("asia"),
                ])
                .default_value(
                    sapid::TreeChoices::new([
                        sapid::TreePath::new(["asia".into(), "jp".into()]).unwrap()
                    ])
                    .unwrap(),
                )
                .default_expanded(["asia".to_owned()].into_iter().collect())
                .open(true)
                .field(field)
        });
    crate::showcase_style_explorer::explorer(
        "cascader-styles",
        Box::new(
            sapid::stack()
                .class("site-field-state-example")
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .max_width(Limit::Px(340.)),
                )
                .fill(preview),
        ),
        &[
            (
                "wrapper",
                "Root element of the Input",
                ".sapid-tree-select-control",
            ),
            ("input", "Input element", "[role=\"combobox\"]"),
            (
                "section",
                "Left and right sections",
                ".sapid-tree-select-chevron",
            ),
            (
                "bottomSection",
                "Bottom section element, rendered inside the input border at the bottom",
                "",
            ),
            ("root", "Root element", ".sapid-field"),
            ("label", "Label element", ".sapid-field-label"),
            (
                "required",
                "Required asterisk element, rendered inside label",
                ".sapid-field-label::after",
            ),
            (
                "description",
                "Description element",
                ".sapid-field-description",
            ),
            ("error", "Error element", ".sapid-field-error"),
            (
                "success",
                "Success element",
                ".site-valid-field .sapid-field-description",
            ),
            (
                "dropdown",
                "Dropdown root element",
                ".sapid-tree-select-popup",
            ),
            ("options", "Options wrapper", "[role=\"tree\"]"),
            ("option", "Option", "[role=\"treeitem\"]"),
            ("empty", "Nothing found message", ""),
            ("group", "Options group wrapper", ""),
            ("groupLabel", "Options group label", ""),
            (
                "columnsList",
                "Wrapper of all cascading columns in the dropdown",
                "[role=\"tree\"]",
            ),
            (
                "columnsOverflow",
                "Control displayed to reveal levels hidden by `maxDisplayedLevels`",
                "",
            ),
            ("column", "Single cascading column", "[role=\"tree\"]"),
            (
                "columnScroll",
                "Scrollable options area of a column",
                ".sapid-tree-select-popup",
            ),
            (
                "columnOption",
                "Option control inside a column",
                "[role=\"treeitem\"]",
            ),
            (
                "columnOptionLabel",
                "Label of the column option",
                "[role=\"treeitem\"] [data-gly-kind=\"text\"]",
            ),
            (
                "columnOptionIcon",
                "Expand chevron of the column option",
                ".sapid-tree-disclosure",
            ),
            (
                "columnOptionCheck",
                "Check icon displayed on the selected leaf option",
                ".sapid-tree-item[aria-selected=true]:not([aria-expanded])::after",
            ),
            ("columnEmpty", "Empty state of a column", ""),
            (
                "flatOption",
                "Option content in the flat list (search results / `withColumns={false}`)",
                "",
            ),
        ],
    )
}
