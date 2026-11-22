use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    let ink = theme.muted;
    sapid::stack()
        .theme(theme.clone())
        .class("site-field-state-example")
        .gap(16.)
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .fill((
            sapid::field("tree-select-inset-start")
                .theme(theme.clone())
                .label("Your favorite item")
                .control(move |field| {
                    sapid::input_group()
                        .inset_start(
                            sapid::theme_icon(sapid::Icon::SquaresFour)
                                .icon_size(16.)
                                .color(ink)
                                .variant(sapid::ThemeIconVariant::Transparent),
                        )
                        .input(
                            sapid::tree_select("tree-select-inset-start-control")
                                .placeholder("Your favorite item")
                                .nodes(items())
                                .field(field),
                        )
                }),
            sapid::field("tree-select-inset-end")
                .theme(theme.clone())
                .label("Your favorite item")
                .control(move |field| {
                    sapid::input_group()
                        .inset_end(
                            sapid::theme_icon(sapid::Icon::SquaresFour)
                                .icon_size(16.)
                                .color(ink)
                                .variant(sapid::ThemeIconVariant::Transparent),
                        )
                        .input(
                            sapid::tree_select("tree-select-inset-end-control")
                                .placeholder("Your favorite item")
                                .nodes(items())
                                .field(field),
                        )
                }),
        ))
}

fn items() -> Vec<sapid::TreeNode> {
    let items = [
        ("electronics", "Electronics", ""),
        ("phones", "Phones", "electronics"),
        ("iphone", "iPhone", "phones"),
        ("samsung", "Samsung Galaxy", "phones"),
        ("pixel", "Google Pixel", "phones"),
        ("laptops", "Laptops", "electronics"),
        ("macbook", "MacBook", "laptops"),
        ("thinkpad", "ThinkPad", "laptops"),
        ("headphones", "Headphones", "electronics"),
        ("groceries", "Groceries", ""),
        ("fruits", "Fruits", "groceries"),
        ("apples", "Apples", "fruits"),
        ("bananas", "Bananas", "fruits"),
        ("vegetables", "Vegetables", "groceries"),
        ("carrots", "Carrots", "vegetables"),
        ("broccoli", "Broccoli", "vegetables"),
        ("books", "Books", ""),
        ("toys", "Toys", ""),
    ];
    let branches: std::collections::BTreeSet<String> = items
        .iter()
        .filter(|(_, _, parent)| !parent.is_empty())
        .map(|(_, _, p)| (*p).to_owned())
        .collect();
    items
        .map(|(key, label, parent)| {
            let mut n = sapid::TreeNode::new(key, label);
            if !parent.is_empty() {
                n = n.parent(parent);
            }
            if branches.contains(key) {
                n = n.kind(sapid::TreeKind::Branch);
            }
            n
        })
        .into_iter()
        .collect()
}
