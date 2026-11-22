pub(super) fn element_table(
    theme: &sapid::Theme,
    id: &str,
    long: bool,
    caption: bool,
    sticky: bool,
    by_mass: bool,
) -> sapid::Table {
    let short = [
        ("6", "Carbon", "C", "12.011"),
        ("7", "Nitrogen", "N", "14.007"),
        ("39", "Yttrium", "Y", "88.906"),
        ("56", "Barium", "Ba", "137.33"),
        ("58", "Cerium", "Ce", "140.12"),
    ];
    let first = [
        ("1", "Hydrogen", "H", "1.008"),
        ("2", "Helium", "He", "4.0026"),
        ("3", "Lithium", "Li", "6.94"),
        ("4", "Beryllium", "Be", "9.0122"),
        ("5", "Boron", "B", "10.81"),
        ("6", "Carbon", "C", "12.011"),
        ("7", "Nitrogen", "N", "14.007"),
        ("8", "Oxygen", "O", "15.999"),
        ("9", "Fluorine", "F", "18.998"),
        ("10", "Neon", "Ne", "20.18"),
        ("11", "Sodium", "Na", "22.99"),
        ("12", "Magnesium", "Mg", "24.305"),
        ("13", "Aluminium", "Al", "26.982"),
        ("14", "Silicon", "Si", "28.085"),
        ("15", "Phosphorus", "P", "30.974"),
        ("16", "Sulfur", "S", "32.06"),
        ("17", "Chlorine", "Cl", "35.45"),
        ("18", "Argon", "Ar", "39.948"),
        ("19", "Potassium", "K", "39.098"),
        ("20", "Calcium", "Ca", "40.078"),
    ];
    let data = if long { &first[..] } else { &short[..] };
    let headers = if by_mass {
        ["Element position", "Atomic mass", "Symbol", "Element name"]
    } else {
        ["Element position", "Element name", "Symbol", "Atomic mass"]
    };
    let headings = |id: &str| {
        sapid::table_row(id).cells(
            headers.map(|s| sapid::table_header_cell(s.to_lowercase().replace(' ', "-")).text(s)),
        )
    };
    let mut table = sapid::table(id)
        .theme(theme.clone())
        .class("sapid-demo-periodic-table")
        .with_row_borders(true)
        .horizontal_spacing(10.)
        .vertical_spacing(7.)
        .sticky_header(sticky)
        .head(sapid::table_head().row(headings("head")))
        .body(
            sapid::table_body().rows(data.iter().map(|&(number, name, symbol, mass)| {
                let cols = if by_mass {
                    [number, mass, symbol, name]
                } else {
                    [number, name, symbol, mass]
                };
                sapid::table_row(format!("element-{number}")).cells(
                    cols.into_iter()
                        .enumerate()
                        .map(|(i, s)| sapid::table_cell(format!("cell-{i}")).text(s)),
                )
            })),
        );
    if caption {
        table = table
            .caption("Some elements from periodic table")
            .caption_side(sapid::TableCaptionSide::Bottom)
            .foot(sapid::table_foot().row(headings("foot")));
    }
    table
}
