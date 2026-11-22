use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let names = [
        ("James Smith", "Acme Corp", "New York"),
        ("Maria Garcia", "Globex Inc", "Los Angeles"),
        ("Robert Johnson", "Initech", "Chicago"),
        ("Linda Williams", "Umbrella Corp", "Houston"),
        ("Michael Brown", "Stark Industries", "Phoenix"),
        ("Barbara Jones", "Wayne Enterprises", "Philadelphia"),
        ("David Miller", "Cyberdyne Systems", "San Antonio"),
        ("Patricia Davis", "Oscorp", "San Diego"),
        ("Richard Wilson", "LexCorp", "Dallas"),
        ("Jennifer Moore", "Massive Dynamic", "San Jose"),
    ];
    let count = 2000;
    let rows = (1..=count)
        .map(|n| {
            let (name, company, city) = names[(n - 1) % names.len()];
            sapid::DataRecord::new(
                format!("employee-{n}"),
                [
                    n.to_string(),
                    name.to_owned(),
                    format!("user{n}@example.com"),
                    company.to_owned(),
                    city.to_owned(),
                ],
            )
        })
        .collect::<Vec<_>>();
    sapid::data_table("employee-table")
        .theme(theme)
        .class("sapid-demo-employee-table")
        .name("Employee directory")
        .columns(
            [
                ("id", "#", 48),
                ("name", "Name", 115),
                ("email", "Email", 174),
                ("company", "Company", 150),
                ("city", "City", 125),
            ]
            .map(|(key, label, width)| {
                let mut column = sapid::DataColumn::new(key, label);
                column.width = width;
                column.sortable = false;
                column
            }),
        )
        .local_rows(rows)
        .default_query(sapid::DataTableQuery {
            request: sapid::DataRequest {
                page_size: count,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_controls(false)
        .selectable(false)
        .height(360.)
        .row_height(36.)
}
