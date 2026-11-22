use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::dropdown_menu("reference-submenus")
        .theme(theme)
        .label("Toggle Menu")
        .width(200.)
        .item(sapid::menu_item("dashboard").label("Dashboard"))
        .submenu(
            sapid::menu_submenu("products")
                .label("Products")
                .item(sapid::menu_item("all").label("All products"))
                .item(sapid::menu_item("categories").label("Categories"))
                .item(sapid::menu_item("tags").label("Tags"))
                .item(sapid::menu_item("attributes").label("Attributes"))
                .item(sapid::menu_item("shipping").label("Shipping classes")),
        )
        .item(sapid::menu_item("customers").label("Customers"))
        .item(sapid::menu_item("reports").label("Reports"))
        .submenu(
            sapid::menu_submenu("orders")
                .label("Orders")
                .item(sapid::menu_item("open").label("Open"))
                .item(sapid::menu_item("completed").label("Completed"))
                .item(sapid::menu_item("canceled").label("Canceled")),
        )
}
