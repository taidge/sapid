use super::showcase_examples::DetailSection;
mod autocomplete_properties;
mod cascader_properties;
mod date_input_properties;
mod date_picker_input_properties;
mod date_time_picker_properties;
mod json_input_properties;
mod month_picker_input_properties;
mod multi_select_properties;
mod phone_input_properties;
mod pills_input_properties;
mod select_properties;
mod tags_input_properties;
mod time_picker_properties;
mod tree_select_properties;
mod year_picker_input_properties;

struct Example {
    title: &'static str,
    source: &'static str,
    build: fn(sapid::Theme) -> Box<dyn glory::ui::Widget>,
}
fn find(id: &str) -> Option<Example> {
    macro_rules! section {
        ($module:ident, $title:literal) => {
            Example {
                title: $title,
                source: include_str!(concat!(stringify!($module), ".rs")),
                build: |theme| Box::new($module::demo(theme)),
            }
        };
    }
    Some(match id {
        "date-input" => section!(date_input_properties, "Input props"),
        "date-picker-input" => section!(date_picker_input_properties, "Input props"),
        "month-picker-input" => section!(month_picker_input_properties, "Input props"),
        "year-picker-input" => section!(year_picker_input_properties, "Input props"),
        "time-picker" => section!(time_picker_properties, "Input props"),
        "date-time-picker" => section!(date_time_picker_properties, "Input props"),
        "autocomplete" => section!(autocomplete_properties, "Input props"),
        "cascader" => section!(cascader_properties, "Input props"),
        "json-input" => section!(json_input_properties, "Input props"),
        "multi-select" => section!(multi_select_properties, "Input props"),
        "phone-input" => section!(phone_input_properties, "Input features"),
        "pills-input" => section!(pills_input_properties, "Input props"),
        "select" => section!(select_properties, "Input props"),
        "tags-input" => section!(tags_input_properties, "Input props"),
        "tree-select" => section!(tree_select_properties, "Input props"),
        _ => return None,
    })
}
pub fn section(id: &str, theme: sapid::Theme) -> Option<DetailSection> {
    let example = find(id)?;
    Some(DetailSection {
        id: "input-properties",
        title: example.title,
        note: "Change the field appearance and its label, description and validation message.",
        example: (example.build)(theme),
        source: example.source,
    })
}
pub fn navigation(id: &str) -> Option<(&'static str, &'static str)> {
    find(id).map(|example| (example.title, "#input-properties"))
}
