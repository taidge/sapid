use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
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
    let nodes = items
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
        .collect::<Vec<_>>();
    sapid::field("scroll-tree-field")
        .theme(theme)
        .class("site-field-state-example")
        .style(
            Style::new()
                .width(Length::Percent(100.))
                .max_width(Limit::Px(340.)),
        )
        .label("Your favorite item")
        .control(move |field| {
            sapid::tree_select("scroll-tree")
                .field(field)
                .nodes(nodes.clone())
                .default_expanded(branches.clone())
                .max_dropdown_height(200.)
                .placeholder("Pick value")
        })
}
