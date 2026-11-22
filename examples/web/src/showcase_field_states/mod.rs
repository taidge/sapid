use super::showcase_examples::DetailSection;
mod autocomplete_disabled;
mod autocomplete_error;
mod autocomplete_readonly;
mod autocomplete_styles;
mod autocomplete_success;
mod cascader_clear_only;
mod cascader_disabled;
mod cascader_error;
mod cascader_readonly;
mod cascader_styles;
mod cascader_success;
mod checkbox_group_submit;
mod checkbox_package_cards;
mod checkbox_styles;
mod checkbox_submit;
mod color_input_styles;
mod color_picker_styles;
mod combobox_button_target;
mod combobox_hidden_list;
mod combobox_popover_styles;
mod combobox_static_list;
mod combobox_styles;
mod field_form_feedback;
mod file_input_disabled;
mod file_input_error;
mod file_input_styles;
mod file_input_success;
mod input_component;
mod input_compound;
mod input_container;
mod input_focus_styles;
mod input_placeholder;
mod input_styles;
mod input_styles_wrapper;
mod input_success_prop;
mod json_input_disabled;
mod json_input_loading;
mod json_input_styles;
mod json_input_success;
mod mask_input_disabled;
mod mask_input_error;
mod mask_input_form;
mod mask_input_success;
mod multi_select_disabled;
mod multi_select_error;
mod multi_select_readonly;
mod multi_select_styles;
mod multi_select_success;
mod native_select_data;
mod native_select_disabled;
mod native_select_dividers;
mod native_select_error;
mod native_select_loading;
mod native_select_options;
mod native_select_sections;
mod native_select_styles;
mod native_select_success;
mod number_input_disabled;
mod number_input_error;
mod number_input_external_controls;
mod number_input_styles;
mod number_input_success;
mod password_input_disabled;
mod password_input_error;
mod password_input_styles;
mod password_input_success;
mod phone_input_disabled;
mod phone_input_error;
mod phone_input_loading;
mod phone_input_readonly;
mod phone_input_styles;
mod phone_input_success;
mod pill_styles;
mod pin_input_styles;
mod pin_input_success;
mod radio_package_cards;
mod radio_styles;
mod segmented_control_styles;
mod segmented_mixed_values;
mod select_disabled;
mod select_error;
mod select_readonly;
mod select_styles;
mod select_success;
mod slider_styles;
mod switch_styles;
mod tag_suggestions;
mod tags_input_disabled;
mod tags_input_error;
mod tags_input_readonly;
mod tags_input_styles;
mod tags_input_success;
mod text_input_disabled;
mod text_input_error;
mod text_input_loading;
mod text_input_styles;
mod text_input_success;
mod textarea_bottom_section;
mod textarea_disabled;
mod textarea_error;
mod textarea_loading;
mod textarea_resize;
mod textarea_styles;
mod textarea_success;
mod tree_scroll_list;
mod tree_select_disabled;
mod tree_select_error;
mod tree_select_readonly;
mod tree_select_success;
struct Example {
    slug: &'static str,
    id: &'static str,
    link: &'static str,
    title: &'static str,
    note: &'static str,
    source: fn() -> &'static str,
    build: fn(sapid::Theme) -> Box<dyn glory::ui::Widget>,
}
macro_rules! example {
    ($slug:literal,$module:ident,$id:literal,$title:literal,$note:literal) => {
        Example {
            slug: $slug,
            id: $id,
            link: concat!("#", $id),
            title: $title,
            note: $note,
            source: || {
                crate::showcase_source_archive::get(concat!(
                    "showcase_field_states/",
                    stringify!($module)
                ))
            },
            build: |theme| Box::new($module::demo(theme)),
        }
    };
}
mod angle_slider_formatlabel_demo;
mod angle_slider_onchangeend_demo;
mod autocomplete_control_dropdown_opened_state_demo;
mod autocomplete_dropdown_animation_demo;
mod autocomplete_dropdown_padding_demo;
mod autocomplete_dropdown_shadow_demo;
mod autocomplete_fit_dropdown_to_viewport_height_demo;
mod autocomplete_inside_popover_demo;
mod autocomplete_left_and_right_sections_demo;
mod autocomplete_renderoption_demo;
mod autocomplete_scrollable_dropdown_demo;
mod autocomplete_sort_options_demo;
mod cascader_allow_deselect_demo;
mod cascader_clearable_demo;
mod cascader_close_on_select_demo;
mod cascader_column_width_demo;
mod cascader_custom_option_rendering_demo;
mod cascader_display_value_demo;
mod cascader_dropdown_animation_demo;
mod cascader_dropdown_offset_demo;
mod cascader_dropdown_padding_demo;
mod cascader_dropdown_position_demo;
mod cascader_dropdown_shadow_demo;
mod cascader_dropdown_width_demo;
mod cascader_flat_list_layout_demo;
mod cascader_inside_popover_demo;
mod cascader_left_and_right_sections_demo;
mod cascader_loading_state_demo;
mod cascader_nothing_found_demo;
mod cascader_scrollable_columns_demo;
mod checkbox_add_custom_sizes_demo;
mod checkbox_autocontrast_demo;
mod checkbox_change_icon_color_demo;
mod checkbox_change_icons_demo;
mod checkbox_checkbox_card_component_demo;
mod checkbox_checkbox_group_disabled_demo;
mod checkbox_checkbox_indicator_demo;
mod checkbox_checkbox_with_tooltip_demo;
mod checkbox_example_customize_with_styles_api_demo;
mod checkbox_label_with_link_demo;
mod checkbox_light_variant_demo;
mod checkbox_maxselectedvalues_demo;
mod checkbox_pointer_cursor_demo;
mod chip_change_checked_icon_demo;
mod chip_chip_with_tooltip_demo;
mod combobox_active_option_demo;
mod combobox_control_dropdown_opened_state_demo;
mod combobox_fit_dropdown_to_viewport_height_demo;
mod combobox_popover_allow_deselect_demo;
mod combobox_popover_controlled_search_value_demo;
mod combobox_popover_dropdown_animation_demo;
mod combobox_popover_dropdown_position_demo;
mod combobox_popover_dropdown_width_demo;
mod combobox_popover_form_submission_demo;
mod combobox_popover_large_data_sets_demo;
mod combobox_popover_limit_options_demo;
mod combobox_popover_multiple_selection_demo;
mod combobox_popover_render_option_demo;
mod combobox_popover_searchable_demo;
mod combobox_popover_sort_options_demo;
mod combobox_scrollable_list_demo;
mod combobox_scrollable_list_with_scrollarea_demo;
mod combobox_select_first_option_demo;
mod combobox_virtualization_demo;
mod file_input_left_and_right_sections_demo;
mod file_input_loading_state_demo;
mod framework_popover;
mod mask_input_reset_value_demo;
mod mask_input_transform_demo;
mod multi_select_clearable_demo;
mod multi_select_control_dropdown_opened_state_demo;
mod multi_select_disabled_options_demo;
mod multi_select_dropdown_animation_demo;
mod multi_select_dropdown_padding_demo;
mod multi_select_dropdown_shadow_demo;
mod multi_select_dropdown_width_demo;
mod multi_select_fit_dropdown_to_viewport_height_demo;
mod multi_select_hide_selected_options_demo;
mod multi_select_inside_popover_demo;
mod multi_select_left_and_right_sections_demo;
mod multi_select_options_filtering_demo;
mod multi_select_renderoption_demo;
mod multi_select_scrollable_dropdown_demo;
mod multi_select_searchable_demo;
mod multi_select_sort_options_demo;
mod number_input_bigint_values_demo;
mod number_input_left_and_right_sections_demo;
mod number_input_loading_state_demo;
mod number_input_negative_numbers_demo;
mod number_input_prefix_and_suffix_demo;
mod password_input_change_visibility_toggle_icon_demo;
mod password_input_strength_meter_example_demo;
mod password_input_visibility_toggle_focus_demo;
mod phone_input_controlled_demo;
mod phone_input_country_auto_detection_demo;
mod phone_input_custom_flags_demo;
mod phone_input_filter_countries_demo;
mod phone_input_phone_extensions_demo;
mod phone_input_searchable_demo;
mod phone_input_use_within_popover_demo;
mod pin_input_regex_type_demo;
mod radio_change_icon_color_demo;
mod radio_change_icon_demo;
mod radio_light_variant_demo;
mod radio_radio_card_component_demo;
mod radio_radio_group_disabled_state_demo;
mod radio_radio_indicator_demo;
mod radio_radio_with_tooltip_demo;
mod segmented_control_accessibility_and_usability_demo;
mod segmented_control_auto_contrast_demo;
mod segmented_control_react_node_as_label_demo;
mod select_control_dropdown_opened_state_demo;
mod select_dropdown_animation_demo;
mod select_dropdown_offset_demo;
mod select_dropdown_padding_demo;
mod select_dropdown_shadow_demo;
mod select_dropdown_width_demo;
mod select_fit_dropdown_to_viewport_height_demo;
mod select_inside_popover_demo;
mod select_left_and_right_sections_demo;
mod select_nothing_found_demo;
mod select_open_on_focus_demo;
mod select_options_filtering_demo;
mod select_sort_options_demo;
mod slider_hidden_marks_demo;
mod slider_marks_demo;
mod slider_vertical_slider_demo;
mod switch_change_styles_based_on_checked_state_demo;
mod switch_icon_labels_demo;
mod switch_inner_labels_demo;
mod switch_maxselectedvalues_demo;
mod switch_thumb_icon_demo;
mod switch_with_tooltip_demo;
mod tags_input_control_dropdown_opened_state_demo;
mod tags_input_disabled_options_demo;
mod tags_input_dropdown_animation_demo;
mod tags_input_dropdown_padding_demo;
mod tags_input_dropdown_position_demo;
mod tags_input_dropdown_shadow_demo;
mod tags_input_dropdown_width_demo;
mod tags_input_fit_dropdown_to_viewport_height_demo;
mod tags_input_group_options_demo;
mod tags_input_inside_popover_demo;
mod tags_input_large_data_sets_demo;
mod tags_input_left_and_right_sections_demo;
mod tags_input_options_filtering_demo;
mod tags_input_renderoption_demo;
mod tags_input_scrollable_dropdown_demo;
mod tags_input_sort_options_demo;
mod tags_input_with_suggestions_demo;
mod tree_select_clearable_demo;
mod tree_select_dropdown_animation_demo;
mod tree_select_dropdown_offset_demo;
mod tree_select_dropdown_padding_demo;
mod tree_select_dropdown_position_demo;
mod tree_select_dropdown_shadow_demo;
mod tree_select_dropdown_width_demo;
mod tree_select_left_and_right_sections_demo;
const EXAMPLES: &[Example] = &[
    example!(
        "segmented-control",
        segmented_mixed_values,
        "segmented-mixed-values-example",
        "Generic value type",
        "A typed age value can represent a number or an adult category."
    ),
    example!(
        "combobox",
        combobox_static_list,
        "combobox-static-list-example",
        "Without dropdown",
        "Options remain visible while the text field accepts edits."
    ),
    example!(
        "combobox",
        combobox_hidden_list,
        "combobox-hidden-list-example",
        "Hide dropdown",
        "The list closes when the query has no match."
    ),
    example!(
        "combobox",
        combobox_button_target,
        "combobox-button-target-example",
        "Combobox.Target",
        "A button opens the grocery list and the selected name appears above it."
    ),
    example!(
        "tree-select",
        tree_scroll_list,
        "tree-scroll-list-example",
        "Scrollable dropdown",
        "The expanded tree scrolls inside a 200px popup."
    ),
    example!(
        "checkbox",
        checkbox_package_cards,
        "checkbox-package-cards-example",
        "Checkbox.Card component",
        "Select any combination of package cards."
    ),
    example!(
        "radio",
        radio_package_cards,
        "radio-package-cards-example",
        "Radio.Card component",
        "Choose one package by clicking its card."
    ),
    example!(
        "cascader",
        cascader_clear_only,
        "cascader-clear-only-example",
        "Clearable",
        "The right section clears the selected location."
    ),
    Example {
        slug: "checkbox",
        id: "table-checkbox-rows-example",
        link: "#table-checkbox-rows-example",
        title: "Example: Table with row selection",
        note: "Each checkbox toggles the highlight on its own row.",
        source: || include_str!("../showcase_typography_docs/table_selection.rs"),
        build: |theme| {
            Box::new(crate::showcase_typography_docs::table_selection::demo(
                theme,
            ))
        },
    },
    example!(
        "input",
        input_component,
        "input-component-example",
        "Change input element",
        "A native button and a native select share the input appearance."
    ),
    example!(
        "input",
        input_placeholder,
        "input-placeholder-example",
        "Input.Placeholder component",
        "The button displays placeholder text."
    ),
    example!(
        "input",
        input_compound,
        "input-compound-example",
        "Input.Label, Input.Description and Input.Error components",
        "Use the field label, description and error as independent parts."
    ),
    example!(
        "input",
        input_success_prop,
        "input-success-prop-example",
        "success prop",
        "The lower field includes a success message."
    ),
    example!(
        "input",
        input_focus_styles,
        "input-focus-styles-example",
        "Change focus styles",
        "Focus either field to see the custom outline."
    ),
    example!(
        "input",
        input_container,
        "input-container-example",
        "inputContainer",
        "Focus the editor to show its tooltip."
    ),
    example!(
        "checkbox",
        checkbox_submit,
        "checkbox-submit-example",
        "Checkbox with @mantine/form",
        "Submit after accepting the terms."
    ),
    example!(
        "checkbox",
        checkbox_group_submit,
        "checkbox-group-submit-example",
        "Checkbox.Group with @mantine/form",
        "Choose one or more frameworks and submit."
    ),
    example!(
        "mask-input",
        mask_input_form,
        "mask-input-form-example",
        "With use-form",
        "The form submits the raw phone digits."
    ),
    example!(
        "number-input",
        number_input_external_controls,
        "number-input-external-controls-example",
        "Custom increment and decrement controls",
        "The buttons change the value by two within the field bounds."
    ),
    example!(
        "tags-input",
        tags_input_with_suggestions_demo,
        "tags-input-with-suggestions-demo",
        "With suggestions",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_options_filtering_demo,
        "tags-input-options-filtering-demo",
        "Options filtering",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_sort_options_demo,
        "tags-input-sort-options-demo",
        "Sort options",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_group_options_demo,
        "tags-input-group-options-demo",
        "Group options",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_disabled_options_demo,
        "tags-input-disabled-options-demo",
        "Disabled options",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_renderoption_demo,
        "tags-input-renderoption-demo",
        "renderOption",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_large_data_sets_demo,
        "tags-input-large-data-sets-demo",
        "Large data sets",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_fit_dropdown_to_viewport_height_demo,
        "tags-input-fit-dropdown-to-viewport-height-demo",
        "Fit dropdown to viewport height",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_dropdown_position_demo,
        "tags-input-dropdown-position-demo",
        "Dropdown position",
        "The suggestions open above the editor."
    ),
    example!(
        "tags-input",
        tags_input_dropdown_animation_demo,
        "tags-input-dropdown-animation-demo",
        "Dropdown animation",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_dropdown_width_demo,
        "tags-input-dropdown-width-demo",
        "Dropdown width",
        "The suggestion list is 200 pixels wide."
    ),
    example!(
        "tags-input",
        tags_input_dropdown_shadow_demo,
        "tags-input-dropdown-shadow-demo",
        "Dropdown shadow",
        "Type to filter suggestions, choose a value to add its pill, or submit a free-form tag."
    ),
    example!(
        "tags-input",
        tags_input_control_dropdown_opened_state_demo,
        "tags-input-control-dropdown-opened-state-demo",
        "Control dropdown opened state",
        "The button toggles suggestions independently of field focus."
    ),
    example!(
        "tags-input",
        tags_input_dropdown_padding_demo,
        "tags-input-dropdown-padding-demo",
        "Dropdown padding",
        "Open both fields to compare their dropdown layouts."
    ),
    example!(
        "tags-input",
        tags_input_scrollable_dropdown_demo,
        "tags-input-scrollable-dropdown-demo",
        "Scrollable dropdown",
        "Open both fields to compare their dropdown layouts."
    ),
    example!(
        "cascader",
        cascader_close_on_select_demo,
        "cascader-close-on-select-demo",
        "Close on select",
        "Picking a city leaves the columns open."
    ),
    example!(
        "cascader",
        cascader_allow_deselect_demo,
        "cascader-allow-deselect-demo",
        "Allow deselect",
        "Choose the selected city again to clear its path."
    ),
    example!(
        "cascader",
        cascader_nothing_found_demo,
        "cascader-nothing-found-demo",
        "Nothing found",
        "Search the hierarchy for a location or an unmatched word."
    ),
    example!(
        "cascader",
        cascader_display_value_demo,
        "cascader-display-value-demo",
        "Display value",
        "The selected value shows the full path separated by chevrons."
    ),
    example!(
        "cascader",
        cascader_custom_option_rendering_demo,
        "cascader-custom-option-rendering-demo",
        "Custom option rendering",
        "Each branch displays a directional marker alongside its label."
    ),
    example!(
        "cascader",
        cascader_scrollable_columns_demo,
        "cascader-scrollable-columns-demo",
        "Scrollable columns",
        "Browse thirty cities inside a bounded column."
    ),
    example!(
        "cascader",
        cascader_dropdown_position_demo,
        "cascader-dropdown-position-demo",
        "Dropdown position",
        "The location options open above the field."
    ),
    example!(
        "cascader",
        cascader_dropdown_offset_demo,
        "cascader-dropdown-offset-demo",
        "Dropdown offset",
        "The first column opens flush with its trigger."
    ),
    example!(
        "cascader",
        cascader_loading_state_demo,
        "cascader-loading-state-demo",
        "Loading state",
        "The pending state displays a spinner at the trailing edge."
    ),
    example!(
        "combobox",
        combobox_scrollable_list_demo,
        "combobox-scrollable-list-demo",
        "Scrollable list",
        "Open the options and choose with the mouse or arrow keys."
    ),
    example!(
        "combobox",
        combobox_scrollable_list_with_scrollarea_demo,
        "combobox-scrollable-list-with-scrollarea-demo",
        "Scrollable list with ScrollArea",
        "Open the options and choose with the mouse or arrow keys."
    ),
    example!(
        "combobox",
        combobox_fit_dropdown_to_viewport_height_demo,
        "combobox-fit-dropdown-to-viewport-height-demo",
        "Fit dropdown to viewport height",
        "Open the options and choose with the mouse or arrow keys."
    ),
    example!(
        "combobox",
        combobox_select_first_option_demo,
        "combobox-select-first-option-demo",
        "Select first option",
        "Open the options and choose with the mouse or arrow keys."
    ),
    example!(
        "combobox",
        combobox_active_option_demo,
        "combobox-active-option-demo",
        "Active option",
        "Open the options and choose with the mouse or arrow keys."
    ),
    example!(
        "combobox",
        combobox_virtualization_demo,
        "combobox-virtualization-demo",
        "Virtualization",
        "Open the options and choose with the mouse or arrow keys."
    ),
    example!(
        "combobox-popover",
        combobox_popover_allow_deselect_demo,
        "combobox-popover-allow-deselect-demo",
        "Allow deselect",
        "Select React again in each popover to compare how the current choice is retained or cleared."
    ),
    example!(
        "combobox-popover",
        combobox_popover_large_data_sets_demo,
        "combobox-popover-large-data-sets-demo",
        "Large data sets",
        "Scroll through fifty options in a dropdown with a maximum height of 200px."
    ),
    example!(
        "combobox-popover",
        combobox_popover_form_submission_demo,
        "combobox-popover-form-submission-demo",
        "Form submission",
        "Select a framework and submit the form to read its named value."
    ),
    example!(
        "combobox-popover",
        combobox_popover_searchable_demo,
        "combobox-popover-searchable-demo",
        "Searchable",
        "The popover target reflects the current selection. Type in the popup to narrow the choices."
    ),
    example!(
        "combobox-popover",
        combobox_popover_controlled_search_value_demo,
        "combobox-popover-controlled-search-value-demo",
        "Controlled search value",
        "The popover target reflects the current selection. Type in the popup to narrow the choices."
    ),
    example!(
        "combobox-popover",
        combobox_popover_multiple_selection_demo,
        "combobox-popover-multiple-selection-demo",
        "Multiple selection",
        "The popover target reflects the current selection. Type in the popup to narrow the choices."
    ),
    example!(
        "checkbox",
        checkbox_light_variant_demo,
        "checkbox-light-variant-demo",
        "Light variant",
        "A tinted background preserves the selected mark in each color."
    ),
    example!(
        "checkbox",
        checkbox_change_icon_color_demo,
        "checkbox-change-icon-color-demo",
        "Change icon color",
        "The selected mark uses a dark foreground on lime."
    ),
    example!(
        "checkbox",
        checkbox_checkbox_with_tooltip_demo,
        "checkbox-checkbox-with-tooltip-demo",
        "Checkbox with tooltip",
        "Hover the control or the whole labeled target to compare tooltip hit areas."
    ),
    example!(
        "checkbox",
        checkbox_checkbox_card_component_demo,
        "checkbox-checkbox-card-component-demo",
        "Checkbox.Card component",
        "The entire package card is a selection target."
    ),
    example!(
        "checkbox",
        checkbox_checkbox_indicator_demo,
        "checkbox-checkbox-indicator-demo",
        "Checkbox.Indicator",
        "Passive indicators show selected, empty and disabled states."
    ),
    example!(
        "radio",
        radio_light_variant_demo,
        "radio-light-variant-demo",
        "Light variant",
        "A tinted background preserves the selected mark in each color."
    ),
    example!(
        "radio",
        radio_change_icon_color_demo,
        "radio-change-icon-color-demo",
        "Change icon color",
        "The selected mark uses a dark foreground on lime."
    ),
    example!(
        "radio",
        radio_radio_with_tooltip_demo,
        "radio-radio-with-tooltip-demo",
        "Radio with tooltip",
        "Hover the control or the whole labeled target to compare tooltip hit areas."
    ),
    example!(
        "radio",
        radio_radio_card_component_demo,
        "radio-radio-card-component-demo",
        "Radio.Card component",
        "The entire package card is a selection target."
    ),
    example!(
        "radio",
        radio_radio_indicator_demo,
        "radio-radio-indicator-demo",
        "Radio.Indicator",
        "Passive indicators show selected, empty and disabled states."
    ),
    example!(
        "checkbox",
        checkbox_change_icons_demo,
        "checkbox-change-icons-demo",
        "Change icons",
        "Checked and indeterminate states display distinct symbols."
    ),
    example!(
        "radio",
        radio_change_icon_demo,
        "radio-change-icon-demo",
        "Change icon",
        "A checkmark replaces the standard radio dot."
    ),
    example!(
        "checkbox",
        checkbox_label_with_link_demo,
        "checkbox-label-with-link-demo",
        "Label with link",
        "The agreement text contains a keyboard-accessible link."
    ),
    example!(
        "checkbox",
        checkbox_pointer_cursor_demo,
        "checkbox-pointer-cursor-demo",
        "Pointer cursor",
        "Hover the control or its label to see the pointer cursor."
    ),
    example!(
        "checkbox",
        checkbox_autocontrast_demo,
        "checkbox-autocontrast-demo",
        "autoContrast",
        "Compare light and dark foreground marks on the same lime background."
    ),
    example!(
        "checkbox",
        checkbox_add_custom_sizes_demo,
        "checkbox-add-custom-sizes-demo",
        "Add custom sizes",
        "The two controls use 14 and 42 pixel hit indicators with proportional labels."
    ),
    example!(
        "checkbox",
        checkbox_example_customize_with_styles_api_demo,
        "checkbox-example-customize-with-styles-api-demo",
        "Example: Customize with Styles API",
        "Toggle the outlined label to apply its filled selection style."
    ),
    example!(
        "chip",
        chip_change_checked_icon_demo,
        "chip-change-checked-icon-demo",
        "Change checked icon",
        "The checked chip uses a cross and a red filled background."
    ),
    example!(
        "chip",
        chip_chip_with_tooltip_demo,
        "chip-chip-with-tooltip-demo",
        "Chip with tooltip",
        "Hover or focus the chip to reveal its description."
    ),
    example!(
        "switch",
        switch_inner_labels_demo,
        "switch-inner-labels-demo",
        "Inner Labels",
        "Five control sizes display the active ON or OFF label inside the rail."
    ),
    example!(
        "switch",
        switch_icon_labels_demo,
        "switch-icon-labels-demo",
        "Icon labels",
        "The rail changes between moon and sun icons."
    ),
    example!(
        "switch",
        switch_thumb_icon_demo,
        "switch-thumb-icon-demo",
        "Thumb icon",
        "The moving thumb changes from a red cross to a teal checkmark."
    ),
    example!(
        "switch",
        switch_with_tooltip_demo,
        "switch-with-tooltip-demo",
        "With tooltip",
        "Hover or focus the switch to show its tooltip."
    ),
    example!(
        "switch",
        switch_change_styles_based_on_checked_state_demo,
        "switch-change-styles-based-on-checked-state-demo",
        "Change styles based on checked state",
        "Switch on to apply the lime rail and black thumb."
    ),
    example!(
        "password-input",
        password_input_visibility_toggle_focus_demo,
        "password-input-visibility-toggle-focus-demo",
        "Visibility toggle focus",
        "Tab from the password to its visibility button and then the nickname."
    ),
    example!(
        "password-input",
        password_input_change_visibility_toggle_icon_demo,
        "password-input-change-visibility-toggle-icon-demo",
        "Change visibility toggle icon",
        "A custom vector reflects whether the entered password is visible."
    ),
    example!(
        "password-input",
        password_input_strength_meter_example_demo,
        "password-input-strength-meter-example-demo",
        "Strength meter example",
        "Focus the password to see five requirements update as you type."
    ),
    example!(
        "pin-input",
        pin_input_regex_type_demo,
        "pin-input-regex-type-demo",
        "Regex type",
        "The four cells accept only the digits 0 through 3."
    ),
    example!(
        "segmented-control",
        segmented_control_react_node_as_label_demo,
        "segmented-control-react-node-as-label-demo",
        "React node as label",
        "Each text label includes its corresponding icon."
    ),
    example!(
        "segmented-control",
        segmented_control_accessibility_and_usability_demo,
        "segmented-control-accessibility-and-usability-demo",
        "Accessibility and usability",
        "Every icon-only choice retains its accessible name."
    ),
    example!(
        "segmented-control",
        segmented_control_auto_contrast_demo,
        "segmented-control-auto-contrast-demo",
        "Auto contrast",
        "The second control uses dark selected text on the same lime background."
    ),
    example!(
        "autocomplete",
        autocomplete_left_and_right_sections_demo,
        "autocomplete-left-and-right-sections-demo",
        "Left and right sections",
        "The same passive adornment can sit inside either edge of the actual input."
    ),
    example!(
        "select",
        select_left_and_right_sections_demo,
        "select-left-and-right-sections-demo",
        "Left and right sections",
        "The same passive adornment can sit inside either edge of the actual input."
    ),
    example!(
        "multi-select",
        multi_select_left_and_right_sections_demo,
        "multi-select-left-and-right-sections-demo",
        "Left and right sections",
        "The same passive adornment can sit inside either edge of the actual input."
    ),
    example!(
        "tags-input",
        tags_input_left_and_right_sections_demo,
        "tags-input-left-and-right-sections-demo",
        "Left and right sections",
        "The same passive adornment can sit inside either edge of the actual input."
    ),
    example!(
        "file-input",
        file_input_left_and_right_sections_demo,
        "file-input-left-and-right-sections-demo",
        "Left and right sections",
        "The same passive adornment can sit inside either edge of the actual input."
    ),
    example!(
        "number-input",
        number_input_left_and_right_sections_demo,
        "number-input-left-and-right-sections-demo",
        "Left and right sections",
        "The same passive adornment can sit inside either edge of the actual input."
    ),
    example!(
        "cascader",
        cascader_left_and_right_sections_demo,
        "cascader-left-and-right-sections-demo",
        "Left and right sections",
        "The same passive adornment can sit inside either edge of the actual input."
    ),
    example!(
        "tree-select",
        tree_select_left_and_right_sections_demo,
        "tree-select-left-and-right-sections-demo",
        "Left and right sections",
        "The same passive adornment can sit inside either edge of the actual input."
    ),
    example!(
        "autocomplete",
        autocomplete_inside_popover_demo,
        "autocomplete-inside-popover-demo",
        "Inside Popover",
        "Open the popover and use the field inside it."
    ),
    example!(
        "select",
        select_inside_popover_demo,
        "select-inside-popover-demo",
        "Inside Popover",
        "Open the popover and use the field inside it."
    ),
    example!(
        "multi-select",
        multi_select_inside_popover_demo,
        "multi-select-inside-popover-demo",
        "Inside Popover",
        "Open the popover and use the field inside it."
    ),
    example!(
        "tags-input",
        tags_input_inside_popover_demo,
        "tags-input-inside-popover-demo",
        "Inside Popover",
        "Open the popover and use the field inside it."
    ),
    example!(
        "cascader",
        cascader_inside_popover_demo,
        "cascader-inside-popover-demo",
        "Inside Popover",
        "Open the popover and use the field inside it."
    ),
    example!(
        "phone-input",
        phone_input_use_within_popover_demo,
        "phone-input-use-within-popover-demo",
        "Use within Popover",
        "A searchable country menu and telephone editor remain usable inside the parent popover."
    ),
    example!(
        "number-input",
        number_input_negative_numbers_demo,
        "number-input-negative-numbers-demo",
        "Negative numbers",
        "The editor rejects values below zero."
    ),
    example!(
        "number-input",
        number_input_bigint_values_demo,
        "number-input-bigint-values-demo",
        "BigInt values",
        "The decimal model retains the exact integer above the JavaScript safe-integer threshold."
    ),
    example!(
        "number-input",
        number_input_loading_state_demo,
        "number-input-loading-state-demo",
        "Loading state",
        "A loading indicator occupies the trailing section."
    ),
    example!(
        "number-input",
        number_input_prefix_and_suffix_demo,
        "number-input-prefix-and-suffix-demo",
        "Prefix and suffix",
        "The adornment stays visible while the underlying value remains numeric."
    ),
    example!(
        "mask-input",
        mask_input_reset_value_demo,
        "mask-input-reset-value-demo",
        "Reset value",
        "Reset the accepted raw digits and the formatted editor together."
    ),
    example!(
        "mask-input",
        mask_input_transform_demo,
        "mask-input-transform-demo",
        "Transform",
        "Accepted letters are converted to uppercase while editing the code."
    ),
    example!(
        "slider",
        slider_marks_demo,
        "slider-marks-demo",
        "Marks",
        "Compare an unlabeled set of marks with marked percentages."
    ),
    example!(
        "slider",
        slider_hidden_marks_demo,
        "slider-hidden-marks-demo",
        "Hidden marks",
        "The slider can stop at 25% and 75% although those marks are hidden."
    ),
    example!(
        "slider",
        slider_vertical_slider_demo,
        "slider-vertical-slider-demo",
        "Vertical slider",
        "The single and range sliders both respond to vertical pointer movement and keyboard steps."
    ),
    example!(
        "angle-slider",
        angle_slider_onchangeend_demo,
        "angle-slider-onchangeend-demo",
        "onChangeEnd",
        "Dragging updates the current angle. Release commits the final value."
    ),
    example!(
        "angle-slider",
        angle_slider_formatlabel_demo,
        "angle-slider-formatlabel-demo",
        "formatLabel",
        "The displayed and accessible labels both include the degree sign."
    ),
    example!(
        "combobox-popover",
        combobox_popover_limit_options_demo,
        "combobox-popover-limit-options-demo",
        "Limit options",
        "Filtering updates the list while the visible option count stays bounded."
    ),
    example!(
        "multi-select",
        multi_select_options_filtering_demo,
        "multi-select-options-filtering-demo",
        "Options filtering",
        "A prefix filter matches the beginning of the option label."
    ),
    example!(
        "select",
        select_options_filtering_demo,
        "select-options-filtering-demo",
        "Options filtering",
        "A prefix filter matches the beginning of the option label."
    ),
    example!(
        "select",
        select_nothing_found_demo,
        "select-nothing-found-demo",
        "Nothing found",
        "A query with no matching libraries displays the empty-state message."
    ),
    example!(
        "select",
        select_open_on_focus_demo,
        "select-open-on-focus-demo",
        "Open on focus",
        "Focusing the trigger with the keyboard opens the options."
    ),
    example!(
        "switch",
        switch_maxselectedvalues_demo,
        "switch-maxselectedvalues-demo",
        "maxSelectedValues",
        "Select two switches to disable the remaining choices. Deselect one to enable them again."
    ),
    example!(
        "autocomplete",
        autocomplete_dropdown_padding_demo,
        "autocomplete-dropdown-padding-demo",
        "Dropdown padding",
        "Compare the same list with zero or ten pixels of inner padding."
    ),
    example!(
        "autocomplete",
        autocomplete_dropdown_shadow_demo,
        "autocomplete-dropdown-shadow-demo",
        "Dropdown shadow",
        "Open the list to see the shadow around its surface."
    ),
    example!(
        "autocomplete",
        autocomplete_dropdown_animation_demo,
        "autocomplete-dropdown-animation-demo",
        "Dropdown animation",
        "Open the list to see its 200 ms entrance transition."
    ),
    example!(
        "autocomplete",
        autocomplete_scrollable_dropdown_demo,
        "autocomplete-scrollable-dropdown-demo",
        "Scrollable dropdown",
        "Scroll the options inside a bounded list."
    ),
    example!(
        "autocomplete",
        autocomplete_fit_dropdown_to_viewport_height_demo,
        "autocomplete-fit-dropdown-to-viewport-height-demo",
        "Fit dropdown to viewport height",
        "The open list shrinks to the space available in a short viewport."
    ),
    example!(
        "select",
        select_dropdown_padding_demo,
        "select-dropdown-padding-demo",
        "Dropdown padding",
        "Compare the same list with zero or ten pixels of inner padding."
    ),
    example!(
        "select",
        select_dropdown_shadow_demo,
        "select-dropdown-shadow-demo",
        "Dropdown shadow",
        "Open the list to see the shadow around its surface."
    ),
    example!(
        "select",
        select_dropdown_animation_demo,
        "select-dropdown-animation-demo",
        "Dropdown animation",
        "Open the list to see its 200 ms entrance transition."
    ),
    example!(
        "select",
        select_dropdown_width_demo,
        "select-dropdown-width-demo",
        "Dropdown width",
        "Open the list to compare its width with its trigger."
    ),
    example!(
        "select",
        select_dropdown_offset_demo,
        "select-dropdown-offset-demo",
        "Dropdown offset",
        "Open the list flush with its trigger."
    ),
    example!(
        "select",
        select_fit_dropdown_to_viewport_height_demo,
        "select-fit-dropdown-to-viewport-height-demo",
        "Fit dropdown to viewport height",
        "The open list shrinks to the space available in a short viewport."
    ),
    example!(
        "multi-select",
        multi_select_dropdown_padding_demo,
        "multi-select-dropdown-padding-demo",
        "Dropdown padding",
        "Compare the same list with zero or ten pixels of inner padding."
    ),
    example!(
        "multi-select",
        multi_select_dropdown_shadow_demo,
        "multi-select-dropdown-shadow-demo",
        "Dropdown shadow",
        "Open the list to see the shadow around its surface."
    ),
    example!(
        "multi-select",
        multi_select_dropdown_animation_demo,
        "multi-select-dropdown-animation-demo",
        "Dropdown animation",
        "Open the list to see its 200 ms entrance transition."
    ),
    example!(
        "multi-select",
        multi_select_dropdown_width_demo,
        "multi-select-dropdown-width-demo",
        "Dropdown width",
        "Open the list to compare its width with its trigger."
    ),
    example!(
        "multi-select",
        multi_select_scrollable_dropdown_demo,
        "multi-select-scrollable-dropdown-demo",
        "Scrollable dropdown",
        "Scroll the options inside a bounded list."
    ),
    example!(
        "multi-select",
        multi_select_fit_dropdown_to_viewport_height_demo,
        "multi-select-fit-dropdown-to-viewport-height-demo",
        "Fit dropdown to viewport height",
        "The open list shrinks to the space available in a short viewport."
    ),
    example!(
        "tree-select",
        tree_select_dropdown_padding_demo,
        "tree-select-dropdown-padding-demo",
        "Dropdown padding",
        "Compare the same list with zero or ten pixels of inner padding."
    ),
    example!(
        "tree-select",
        tree_select_dropdown_shadow_demo,
        "tree-select-dropdown-shadow-demo",
        "Dropdown shadow",
        "Open the list to see the shadow around its surface."
    ),
    example!(
        "tree-select",
        tree_select_dropdown_animation_demo,
        "tree-select-dropdown-animation-demo",
        "Dropdown animation",
        "Open the list to see its 200 ms entrance transition."
    ),
    example!(
        "tree-select",
        tree_select_dropdown_width_demo,
        "tree-select-dropdown-width-demo",
        "Dropdown width",
        "Open the list to compare its width with its trigger."
    ),
    example!(
        "tree-select",
        tree_select_dropdown_offset_demo,
        "tree-select-dropdown-offset-demo",
        "Dropdown offset",
        "Open the list flush with its trigger."
    ),
    example!(
        "tree-select",
        tree_select_dropdown_position_demo,
        "tree-select-dropdown-position-demo",
        "Dropdown position",
        "Open the available choices above the field."
    ),
    example!(
        "cascader",
        cascader_dropdown_padding_demo,
        "cascader-dropdown-padding-demo",
        "Dropdown padding",
        "Compare the same list with zero or ten pixels of inner padding."
    ),
    example!(
        "cascader",
        cascader_dropdown_shadow_demo,
        "cascader-dropdown-shadow-demo",
        "Dropdown shadow",
        "Open the list to see the shadow around its surface."
    ),
    example!(
        "cascader",
        cascader_dropdown_animation_demo,
        "cascader-dropdown-animation-demo",
        "Dropdown animation",
        "Open the list to see its 200 ms entrance transition."
    ),
    example!(
        "cascader",
        cascader_dropdown_width_demo,
        "cascader-dropdown-width-demo",
        "Dropdown width",
        "Open the list to compare its width with its trigger."
    ),
    example!(
        "combobox-popover",
        combobox_popover_dropdown_animation_demo,
        "combobox-popover-dropdown-animation-demo",
        "Dropdown animation",
        "Open the list to see its 200 ms entrance transition."
    ),
    example!(
        "combobox-popover",
        combobox_popover_dropdown_width_demo,
        "combobox-popover-dropdown-width-demo",
        "Dropdown width",
        "Open the list to compare its width with its trigger."
    ),
    example!(
        "combobox-popover",
        combobox_popover_dropdown_position_demo,
        "combobox-popover-dropdown-position-demo",
        "Dropdown position",
        "Open the available choices above the field."
    ),
    example!(
        "autocomplete",
        autocomplete_control_dropdown_opened_state_demo,
        "autocomplete-control-dropdown-opened-state-demo",
        "Control dropdown opened state",
        "The button and the field share the same open state."
    ),
    example!(
        "select",
        select_control_dropdown_opened_state_demo,
        "select-control-dropdown-opened-state-demo",
        "Control dropdown opened state",
        "The button and the field share the same open state."
    ),
    example!(
        "multi-select",
        multi_select_control_dropdown_opened_state_demo,
        "multi-select-control-dropdown-opened-state-demo",
        "Control dropdown opened state",
        "The button and the field share the same open state."
    ),
    example!(
        "combobox",
        combobox_control_dropdown_opened_state_demo,
        "combobox-control-dropdown-opened-state-demo",
        "Control dropdown opened state",
        "The button and the field share the same open state."
    ),
    example!(
        "autocomplete",
        autocomplete_sort_options_demo,
        "autocomplete-sort-options-demo",
        "Sort options",
        "Options keep their display labels while the list uses a sorted order."
    ),
    example!(
        "autocomplete",
        autocomplete_renderoption_demo,
        "autocomplete-renderoption-demo",
        "renderOption",
        "Each real option includes a second line with the department."
    ),
    example!(
        "multi-select",
        multi_select_sort_options_demo,
        "multi-select-sort-options-demo",
        "Sort options",
        "Options keep their display labels while the list uses a sorted order."
    ),
    example!(
        "multi-select",
        multi_select_renderoption_demo,
        "multi-select-renderoption-demo",
        "renderOption",
        "Each real option includes a second line with the department."
    ),
    example!(
        "select",
        select_sort_options_demo,
        "select-sort-options-demo",
        "Sort options",
        "Options keep their display labels while the list uses a sorted order."
    ),
    example!(
        "combobox-popover",
        combobox_popover_sort_options_demo,
        "combobox-popover-sort-options-demo",
        "Sort options",
        "Options keep their display labels while the list uses a sorted order."
    ),
    example!(
        "combobox-popover",
        combobox_popover_render_option_demo,
        "combobox-popover-render-option-demo",
        "Render option",
        "Each real option includes a second line with the department."
    ),
    example!(
        "multi-select",
        multi_select_clearable_demo,
        "multi-select-clearable-demo",
        "Clearable",
        "The clear action removes all selected values."
    ),
    example!(
        "multi-select",
        multi_select_searchable_demo,
        "multi-select-searchable-demo",
        "Searchable",
        "Type to filter the available options. Selected values remain as removable pills."
    ),
    example!(
        "multi-select",
        multi_select_hide_selected_options_demo,
        "multi-select-hide-selected-options-demo",
        "Hide selected options",
        "Selected values disappear from the available choices until removed."
    ),
    example!(
        "multi-select",
        multi_select_disabled_options_demo,
        "multi-select-disabled-options-demo",
        "Disabled options",
        "Unavailable options remain in the list and cannot be selected."
    ),
    example!(
        "tree-select",
        tree_select_clearable_demo,
        "tree-select-clearable-demo",
        "Clearable",
        "Clear the selected full path and choose another location."
    ),
    example!(
        "cascader",
        cascader_clearable_demo,
        "cascader-clearable-demo",
        "Clearable",
        "Clear the selected full path and choose another location."
    ),
    example!(
        "cascader",
        cascader_flat_list_layout_demo,
        "cascader-flat-list-layout-demo",
        "Flat list layout",
        "Browse the same location hierarchy in a single column."
    ),
    example!(
        "cascader",
        cascader_column_width_demo,
        "cascader-column-width-demo",
        "Column width",
        "Each column is 140 pixels wide. The open list allows scrolling."
    ),
    example!(
        "checkbox",
        checkbox_checkbox_group_disabled_demo,
        "checkbox-checkbox-group-disabled-demo",
        "Checkbox.Group disabled",
        "The entire group, including its selected item, is disabled."
    ),
    example!(
        "checkbox",
        checkbox_maxselectedvalues_demo,
        "checkbox-maxselectedvalues-demo",
        "maxSelectedValues",
        "The selection is limited to two values."
    ),
    example!(
        "radio",
        radio_radio_group_disabled_state_demo,
        "radio-radio-group-disabled-state-demo",
        "Radio.Group disabled state",
        "The selected radio remains visible while the group is disabled."
    ),
    example!(
        "file-input",
        file_input_loading_state_demo,
        "file-input-loading-state-demo",
        "Loading state",
        "File selection displays its pending indicator."
    ),
    example!(
        "phone-input",
        phone_input_filter_countries_demo,
        "phone-input-filter-countries-demo",
        "Filter countries",
        "Only the six listed countries can be selected."
    ),
    example!(
        "phone-input",
        phone_input_custom_flags_demo,
        "phone-input-custom-flags-demo",
        "Custom flags",
        "The selected calling region is shown as an emoji flag."
    ),
    example!(
        "phone-input",
        phone_input_country_auto_detection_demo,
        "phone-input-country-auto-detection-demo",
        "Country auto-detection",
        "Entering a valid international number also updates its dialing region."
    ),
    example!(
        "phone-input",
        phone_input_searchable_demo,
        "phone-input-searchable-demo",
        "Searchable",
        "Use the native country menu when a text search is unnecessary."
    ),
    example!(
        "phone-input",
        phone_input_controlled_demo,
        "phone-input-controlled-demo",
        "Controlled",
        "The dialing region and latest phone text are retained together."
    ),
    example!(
        "phone-input",
        phone_input_phone_extensions_demo,
        "phone-input-phone-extensions-demo",
        "Phone extensions",
        "Commit an international number and its extension to view the RFC 3966 value."
    ),
    example!(
        "textarea",
        textarea_bottom_section,
        "textarea-bottom-section-example",
        "Bottom section",
        "Count the characters as the message changes."
    ),
    example!(
        "textarea",
        textarea_resize,
        "textarea-resize-example",
        "Enable resize",
        "Drag the lower corner to change the field height."
    ),
    example!(
        "tags-input",
        tags_input_styles,
        "tags-input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "select",
        select_styles,
        "select-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "pill",
        pill_styles,
        "pill-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "multi-select",
        multi_select_styles,
        "multi-select-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "combobox-popover",
        combobox_popover_styles,
        "combobox-popover-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "combobox",
        combobox_styles,
        "combobox-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "cascader",
        cascader_styles,
        "cascader-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "autocomplete",
        autocomplete_styles,
        "autocomplete-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "text-input",
        text_input_styles,
        "text-input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "textarea",
        textarea_styles,
        "textarea-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "switch",
        switch_styles,
        "switch-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "slider",
        slider_styles,
        "slider-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "segmented-control",
        segmented_control_styles,
        "segmented-control-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "radio",
        radio_styles,
        "radio-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "pin-input",
        pin_input_styles,
        "pin-input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "phone-input",
        phone_input_styles,
        "phone-input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "password-input",
        password_input_styles,
        "password-input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "number-input",
        number_input_styles,
        "number-input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "native-select",
        native_select_styles,
        "native-select-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "json-input",
        json_input_styles,
        "json-input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "input",
        input_styles_wrapper,
        "input-styles-wrapper-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "input",
        input_styles,
        "input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "file-input",
        file_input_styles,
        "file-input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "color-picker",
        color_picker_styles,
        "color-picker-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "color-input",
        color_input_styles,
        "color-input-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "checkbox",
        checkbox_styles,
        "checkbox-styles-example",
        "Styles API",
        "Inspect the rendered component parts and their CSS selectors."
    ),
    example!(
        "phone-input",
        phone_input_loading,
        "phone-input-loading-example",
        "Loading state",
        "Show loading progress next to the telephone editor."
    ),
    example!(
        "json-input",
        json_input_loading,
        "json-input-loading-example",
        "Loading state",
        "Display the loading status in the JSON editor."
    ),
    example!(
        "textarea",
        textarea_loading,
        "textarea-loading-example",
        "Loading state",
        "Show a loading indicator while the field is editable."
    ),
    example!(
        "text-input",
        text_input_loading,
        "text-input-loading-example",
        "Loading state",
        "Show a loading indicator while the field is editable."
    ),
    example!(
        "native-select",
        native_select_sections,
        "native-select-sections-example",
        "Left and right sections",
        "Decorate either side of the native select."
    ),
    example!(
        "native-select",
        native_select_loading,
        "native-select-loading-example",
        "Loading state",
        "Display a loading indicator in the right section."
    ),
    example!(
        "native-select",
        native_select_dividers,
        "native-select-dividers-example",
        "With dividers",
        "Native separators divide the initial choice and the frontend and backend library groups."
    ),
    example!(
        "native-select",
        native_select_options,
        "native-select-options-example",
        "children options",
        "Build option groups directly from option widgets."
    ),
    example!(
        "native-select",
        native_select_data,
        "native-select-data-example",
        "data prop",
        "Pass grouped option data, including unavailable options."
    ),
    example!(
        "file-input",
        file_input_error,
        "file-input-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "file-input",
        file_input_success,
        "file-input-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "file-input",
        file_input_disabled,
        "file-input-disabled-example",
        "Disabled state",
        "Disabled controls cannot be edited."
    ),
    example!(
        "json-input",
        json_input_success,
        "json-input-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "json-input",
        json_input_disabled,
        "json-input-disabled-example",
        "Disabled state",
        "Disabled controls cannot be edited."
    ),
    example!(
        "mask-input",
        mask_input_disabled,
        "mask-input-disabled-example",
        "Disabled state",
        "Disabled controls cannot be edited."
    ),
    example!(
        "mask-input",
        mask_input_error,
        "mask-input-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "mask-input",
        mask_input_success,
        "mask-input-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "native-select",
        native_select_disabled,
        "native-select-disabled-example",
        "Disabled state",
        "Disabled controls cannot be edited."
    ),
    example!(
        "native-select",
        native_select_error,
        "native-select-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "native-select",
        native_select_success,
        "native-select-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "number-input",
        number_input_error,
        "number-input-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "number-input",
        number_input_success,
        "number-input-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "number-input",
        number_input_disabled,
        "number-input-disabled-example",
        "Disabled state",
        "Disabled controls cannot be edited."
    ),
    example!(
        "password-input",
        password_input_error,
        "password-input-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "password-input",
        password_input_success,
        "password-input-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "password-input",
        password_input_disabled,
        "password-input-disabled-example",
        "Disabled",
        "Disabled controls cannot be edited."
    ),
    example!(
        "phone-input",
        phone_input_disabled,
        "phone-input-disabled-example",
        "Disabled state",
        "Disabled controls cannot be edited."
    ),
    example!(
        "phone-input",
        phone_input_readonly,
        "phone-input-readonly-example",
        "Read only",
        "The field keeps its value during editing attempts."
    ),
    example!(
        "phone-input",
        phone_input_error,
        "phone-input-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "phone-input",
        phone_input_success,
        "phone-input-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "pin-input",
        pin_input_success,
        "pin-input-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "textarea",
        textarea_error,
        "textarea-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "textarea",
        textarea_success,
        "textarea-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "textarea",
        textarea_disabled,
        "textarea-disabled-example",
        "Disabled state",
        "Disabled controls cannot be edited."
    ),
    example!(
        "text-input",
        text_input_error,
        "text-input-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "text-input",
        text_input_success,
        "text-input-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "text-input",
        text_input_disabled,
        "text-input-disabled-example",
        "Disabled state",
        "Disabled controls cannot be edited."
    ),
    example!(
        "autocomplete",
        autocomplete_readonly,
        "autocomplete-readonly-example",
        "Read only",
        "The field keeps its value during editing attempts."
    ),
    example!(
        "autocomplete",
        autocomplete_disabled,
        "autocomplete-disabled-example",
        "Disabled",
        "Disabled controls cannot be edited."
    ),
    example!(
        "autocomplete",
        autocomplete_error,
        "autocomplete-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "autocomplete",
        autocomplete_success,
        "autocomplete-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "cascader",
        cascader_readonly,
        "cascader-readonly-example",
        "Read only",
        "The field keeps its value during editing attempts."
    ),
    example!(
        "cascader",
        cascader_disabled,
        "cascader-disabled-example",
        "Disabled",
        "Disabled controls cannot be edited."
    ),
    example!(
        "cascader",
        cascader_error,
        "cascader-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "cascader",
        cascader_success,
        "cascader-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "multi-select",
        multi_select_readonly,
        "multi-select-readonly-example",
        "Read only",
        "The field keeps its value during editing attempts."
    ),
    example!(
        "multi-select",
        multi_select_disabled,
        "multi-select-disabled-example",
        "Disabled",
        "Disabled controls cannot be edited."
    ),
    example!(
        "multi-select",
        multi_select_error,
        "multi-select-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "multi-select",
        multi_select_success,
        "multi-select-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "select",
        select_readonly,
        "select-readonly-example",
        "Read only",
        "The field keeps its value during editing attempts."
    ),
    example!(
        "select",
        select_disabled,
        "select-disabled-example",
        "Disabled",
        "Disabled controls cannot be edited."
    ),
    example!(
        "select",
        select_error,
        "select-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "select",
        select_success,
        "select-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "tags-input",
        tags_input_readonly,
        "tags-input-readonly-example",
        "Read only",
        "The field keeps its value during editing attempts."
    ),
    example!(
        "tags-input",
        tags_input_disabled,
        "tags-input-disabled-example",
        "Disabled",
        "Disabled controls cannot be edited."
    ),
    example!(
        "tags-input",
        tags_input_error,
        "tags-input-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "tags-input",
        tags_input_success,
        "tags-input-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
    example!(
        "tree-select",
        tree_select_readonly,
        "tree-select-readonly-example",
        "Read only",
        "The field keeps its value during editing attempts."
    ),
    example!(
        "tree-select",
        tree_select_disabled,
        "tree-select-disabled-example",
        "Disabled",
        "Disabled controls cannot be edited."
    ),
    example!(
        "tree-select",
        tree_select_error,
        "tree-select-error-example",
        "Error state",
        "Show invalid controls with and without a feedback message."
    ),
    example!(
        "tree-select",
        tree_select_success,
        "tree-select-success-example",
        "Success state",
        "A successful value has a confirmation message."
    ),
];
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    EXAMPLES
        .iter()
        .filter(|record| record.slug == slug)
        .map(|record| DetailSection {
            id: record.id,
            title: record.title,
            note: record.note,
            source: (record.source)(),
            example: (record.build)(theme.clone()),
        })
        .collect()
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    EXAMPLES
        .iter()
        .filter(|record| record.slug == slug)
        .map(|record| (record.title, record.link))
        .collect()
}
