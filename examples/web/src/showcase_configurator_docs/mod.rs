mod bubble_chart_change_color;
mod carousel_embla_options;
mod carousel_options;
mod carousel_styles;
mod checkbox_group_config;
mod color_picker_format;
mod color_picker_size;
mod combobox_popover_check_icon;
mod donut_chart_padding_angle;
mod donut_chart_segment_color;
mod donut_chart_segments_labels;
mod donut_chart_segments_stroke;
mod donut_chart_size_and_thickness;
mod funnel_chart_segment_color;
mod funnel_chart_segments_labels;
mod funnel_chart_segments_stroke;
mod funnel_chart_size_and_thickness;
mod heatmap_rect_size_gap_and_radius;
mod input_clear_button;
mod input_wrapper;
mod multi_select_check_icon;
mod pie_chart_segment_color;
mod pie_chart_segments_labels;
mod pie_chart_segments_stroke;
mod pie_chart_size;
mod radar_chart_change_color;
mod radar_chart_parts;
mod radial_bar_chart_change_color;
mod radio_group_config;
mod segmented_control_color;
mod select_check_icon;
mod slider_thumb_size;
mod sunburst_chart_gap;
mod switch_group_config;
mod waffle_chart_fill_direction;
mod waffle_chart_gap_and_grid_size;
mod waffle_chart_legend;
use super::showcase_examples::DetailSection;
mod area_chart_area_color;
mod area_chart_connect_nulls;
mod area_chart_fill_opacity;
mod area_chart_stroke_width;
mod bar_chart_area_color;
mod composite_chart_chart_color;
mod composite_chart_stroke_width;
mod date_picker_size;
mod line_chart_connect_nulls;
mod line_chart_line_color;
mod line_chart_stroke_width;
mod month_picker_size;
mod year_picker_size;

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
    ($slug:literal, $module:ident, $id:literal, $title:literal, $note:literal) => {
        Example {
            slug: $slug,
            id: $id,
            link: concat!("#", $id),
            title: $title,
            note: $note,
            source: || {
                crate::showcase_source_archive::get(concat!(
                    "showcase_configurator_docs/",
                    stringify!($module)
                ))
            },
            build: |theme| Box::new($module::demo(theme)),
        }
    };
}
mod action_bar_close_on_escape;
mod action_icon_add_custom_sizes;
mod action_icon_add_custom_variants;
mod action_icon_autocontrast;
mod action_icon_customize_disabled_styles;
mod action_icon_disabled_button_with_tooltip;
mod action_icon_disabled_controls;
mod action_icon_disabled_state_when_actionicon_is_link;
mod action_icon_gradient;
mod action_icon_group_counter;
mod action_icon_group_orientation;
mod action_icon_input_size;
mod action_icon_loader_props;
mod anchor_underline;
mod app_shell_alternative_layout;
mod app_shell_resizable_alternative;
mod app_shell_without_transitions;
mod area_chart_brush;
mod area_chart_customize_dots;
mod area_chart_dashed_area_line;
mod area_chart_legend;
mod area_chart_legend_position;
mod area_chart_percent_area_chart;
mod area_chart_points_labels;
mod area_chart_reference_areas;
mod area_chart_reference_dots;
mod area_chart_reference_lines;
mod area_chart_remove_tooltip;
mod area_chart_series_labels;
mod area_chart_stacked_area_chart;
mod area_chart_sync_multiple_areacharts;
mod area_chart_x_and_y_axis_props;
mod aspect_ratio_flex_avatar;
mod aspect_ratio_map_embed;
mod aspect_ratio_video_embed;
mod bar_chart_bar_value_label;
mod bar_chart_brush;
mod bar_chart_legend;
mod bar_chart_legend_position;
mod bar_chart_percent_bar_chart;
mod bar_chart_reference_areas;
mod bar_chart_reference_dots;
mod bar_chart_reference_lines;
mod bar_chart_remove_tooltip;
mod bar_chart_series_labels;
mod bar_chart_stacked_bar_chart;
mod bar_chart_sync_multiple_barcharts;
mod bar_chart_x_and_y_axis_props;
mod burger_line_size;
mod button_autocontrast;
mod button_compact;
mod button_custom_variants;
mod button_customize_disabled_styles;
mod button_disabled_button_with_tooltip;
mod button_disabled_control;
mod button_disabled_state_when_button_is_link;
mod button_full_width;
mod button_gradient;
mod button_group_counter;
mod button_group_orientation;
mod button_loader_props;
mod button_loading_variants;
mod button_sections_position;
mod button_styles;
mod button_styles_api;
mod center_inline_link;
mod composite_chart_brush;
mod composite_chart_customize_dots;
mod composite_chart_dashed_lines;
mod composite_chart_legend;
mod composite_chart_legend_position;
mod composite_chart_points_labels;
mod composite_chart_reference_areas;
mod composite_chart_reference_dots;
mod composite_chart_reference_lines;
mod composite_chart_remove_tooltip;
mod composite_chart_series_labels;
mod composite_chart_sync_multiple_charts;
mod composite_chart_x_and_y_axis_props;
mod container_custom_container;
mod container_grid_strategy;
mod container_responsive_container;
mod drawer_change_transition;
mod drawer_compound_components;
mod drawer_initial_focus;
mod drawer_transition_callbacks;
mod floating_indicator_multiple_rows;
mod floating_window_constrain_offset;
mod floating_window_lock_axis;
mod floating_window_window_handle;
mod grid_grow;
mod grid_justify_align;
mod hover_card_delay_group;
mod hover_card_dropdown_role;
mod line_chart_brush;
mod line_chart_customize_dots;
mod line_chart_dashed_line;
mod line_chart_legend;
mod line_chart_legend_position;
mod line_chart_points_labels;
mod line_chart_reference_areas;
mod line_chart_reference_dots;
mod line_chart_reference_lines;
mod line_chart_remove_tooltip;
mod line_chart_series_labels;
mod line_chart_sync_multiple_linecharts;
mod line_chart_x_and_y_axis_props;
mod loading_overlay_custom_inline_loaders;
mod menu_aligned_labels;
mod menu_click_hover;
mod menu_context_menu;
mod menu_custom_check_icon;
mod menu_custom_target;
mod menu_dropdown_position;
mod menu_hover;
mod menu_link_items;
mod menu_styles;
mod menu_submenus;
mod menu_transitions;
mod menubar_styles;
mod modal_change_offsets;
mod modal_change_transitions;
mod modal_compound_components;
mod modal_initial_focus;
mod modal_size_auto;
mod modal_transition_callbacks;
mod nav_link_active;
mod nav_link_autocontrast;
mod overlay_blur;
mod overlay_gradient;
mod pagination_auto_contrast;
mod pagination_boundaries;
mod pagination_change_icons;
mod pagination_compound_components;
mod pagination_controls_size;
mod pagination_example_with_chunked_content;
mod pagination_hide_pages;
mod pagination_links;
mod pagination_responsive_layout;
mod pagination_siblings;
mod pagination_start_value;
mod pagination_styles;
mod popover_arrow;
mod popover_axes;
mod popover_context_target;
mod popover_disabled;
mod popover_focus_trap;
mod popover_hide_detached;
mod popover_inline_elements;
mod popover_offset;
mod popover_with_overlay;
mod reference_agenda_view_all_day_and_multi_day_events;
mod reference_agenda_view_custom_event_rendering;
mod reference_agenda_view_date_formats;
mod reference_agenda_view_event_click;
mod reference_agenda_view_localization;
mod reference_agenda_view_static_mode;
mod reference_area_chart_axis_labels;
mod reference_area_chart_change_area_color_depending_on_color_scheme;
mod reference_area_chart_grid_and_text_colors;
mod reference_area_chart_rotate_x_axis_labels;
mod reference_area_chart_value_formatter;
mod reference_area_chart_x_axis_offset;
mod reference_area_chart_y_axis_scale;
mod reference_bar_chart_axis_labels;
mod reference_bar_chart_change_bar_color_depending_on_color_scheme;
mod reference_bar_chart_grid_and_text_colors;
mod reference_bar_chart_value_formatter;
mod reference_bar_chart_x_axis_offset;
mod reference_bar_chart_y_axis_scale;
mod reference_bars_list_bar_color;
mod reference_bubble_chart_remove_tooltip;
mod reference_bullet_chart_bar_color;
mod reference_bullet_chart_chart_size;
mod reference_bullet_chart_label;
mod reference_bullet_chart_value_formatter;
mod reference_bullet_chart_vertical_orientation;
mod reference_bullet_chart_without_target;
mod reference_composite_chart_axis_labels;
mod reference_composite_chart_change_chart_color_depending_on_color_scheme;
mod reference_composite_chart_grid_and_text_colors;
mod reference_composite_chart_rotate_x_axis_labels;
mod reference_composite_chart_value_formatter;
mod reference_composite_chart_x_axis_offset;
mod reference_composite_chart_y_axis_scale;
mod reference_date_input_date_parser;
mod reference_date_input_disabled_state;
mod reference_date_input_min_and_max_date;
mod reference_date_picker_allow_deselect;
mod reference_date_picker_controlled_date;
mod reference_date_picker_dates_range;
mod reference_date_picker_default_date;
mod reference_date_picker_default_level;
mod reference_date_picker_display_week_numbers;
mod reference_date_picker_exclude_dates;
mod reference_date_picker_first_day_of_week;
mod reference_date_picker_hide_outside_dates;
mod reference_date_picker_hide_weekdays;
mod reference_date_picker_input_clearable;
mod reference_date_picker_input_dates_range;
mod reference_date_picker_input_default_level;
mod reference_date_picker_input_disabled_state;
mod reference_date_picker_input_exclude_dates;
mod reference_date_picker_input_first_day_of_week;
mod reference_date_picker_input_hide_outside_dates;
mod reference_date_picker_input_hide_weekdays;
mod reference_date_picker_input_max_level;
mod reference_date_picker_input_min_and_max_dates;
mod reference_date_picker_input_multiple_dates;
mod reference_date_picker_input_presets;
mod reference_date_picker_input_presets_2;
mod reference_date_picker_input_with_week_numbers;
mod reference_date_picker_localization;
mod reference_date_picker_max_level;
mod reference_date_picker_min_and_max_date;
mod reference_date_picker_multiple_dates;
mod reference_date_picker_presets;
mod reference_date_picker_presets_2;
mod reference_date_picker_single_date_in_range;
mod reference_day_view_all_day_events;
mod reference_day_view_header_format;
mod reference_day_view_interval_minutes;
mod reference_day_view_localization;
mod reference_day_view_overlapping_events;
mod reference_day_view_radius;
mod reference_day_view_recurring_events;
mod reference_day_view_slot_height;
mod reference_day_view_slot_label_format;
mod reference_day_view_slot_label_interval;
mod reference_day_view_static_mode;
mod reference_day_view_time_range;
mod reference_day_view_without_header;
mod reference_donut_chart_chart_label;
mod reference_donut_chart_legend;
mod reference_donut_chart_segments_stroke;
mod reference_donut_chart_start_and_end_angle;
mod reference_donut_chart_tooltip_data_source;
mod reference_donut_chart_without_tooltip;
mod reference_funnel_chart_legend;
mod reference_funnel_chart_segments_stroke;
mod reference_funnel_chart_tooltip_data_source;
mod reference_funnel_chart_without_tooltip;
mod reference_gauge_chart_custom_label;
mod reference_gauge_chart_round_caps;
mod reference_gauge_chart_sections;
mod reference_gauge_chart_start_and_end_angle;
mod reference_gauge_chart_target_marker;
mod reference_gauge_chart_thickness;
mod reference_heatmap_change_colors;
mod reference_heatmap_change_labels_text;
mod reference_heatmap_first_day_of_week;
mod reference_heatmap_legend;
mod reference_heatmap_weekdays_and_months_labels;
mod reference_heatmap_with_tooltip;
mod reference_line_chart_axis_labels;
mod reference_line_chart_change_line_color_depending_on_color_scheme;
mod reference_line_chart_grid_and_text_colors;
mod reference_line_chart_value_formatter;
mod reference_line_chart_x_axis_offset;
mod reference_line_chart_y_axis_scale;
mod reference_matrix_chart_cell_size_and_gap;
mod reference_matrix_chart_colors;
mod reference_matrix_chart_labels;
mod reference_matrix_chart_legend;
mod reference_matrix_chart_tooltip;
mod reference_matrix_chart_values_domain;
mod reference_mini_calendar_getdayprops;
mod reference_mini_calendar_min_and_max_dates;
mod reference_mini_calendar_number_of_days;
mod reference_mobile_month_view_first_day_of_week;
mod reference_mobile_month_view_highlight_today;
mod reference_mobile_month_view_localization;
mod reference_mobile_month_view_static_mode;
mod reference_mobile_month_view_with_outside_days;
mod reference_mobile_month_view_without_week_days;
mod reference_month_picker_allow_deselect;
mod reference_month_picker_controlled_date;
mod reference_month_picker_dates_range;
mod reference_month_picker_default_date;
mod reference_month_picker_input_clearable;
mod reference_month_picker_input_dates_range;
mod reference_month_picker_input_disabled_state;
mod reference_month_picker_input_min_and_max_dates;
mod reference_month_picker_input_multiple_dates;
mod reference_month_picker_input_value_format;
mod reference_month_picker_localization;
mod reference_month_picker_min_and_max_date;
mod reference_month_picker_multiple_dates;
mod reference_month_picker_single_date_in_range;
mod reference_month_view_first_day_of_week;
mod reference_month_view_highlight_today;
mod reference_month_view_localization;
mod reference_month_view_max_events_per_day;
mod reference_month_view_radius;
mod reference_month_view_recurring_events;
mod reference_month_view_static_mode;
mod reference_month_view_without_header;
mod reference_month_view_without_outside_days;
mod reference_month_view_without_week_days;
mod reference_pie_chart_enable_tooltip;
mod reference_pie_chart_legend;
mod reference_pie_chart_segments_stroke;
mod reference_pie_chart_start_and_end_angle;
mod reference_pie_chart_tooltip_data_source;
mod reference_radial_bar_chart_hide_tooltip;
mod reference_radial_bar_chart_legend;
mod reference_recurring_events_dayview_demo;
mod reference_recurring_events_monthview_demo;
mod reference_recurring_events_weekview_demo;
mod reference_recurring_events_yearview_demo;
mod reference_resources_day_view_all_day_events;
mod reference_resources_day_view_localization;
mod reference_resources_day_view_multi_hour_intervals;
mod reference_resources_day_view_radius;
mod reference_resources_day_view_recurring_events;
mod reference_resources_day_view_static_mode;
mod reference_resources_day_view_time_range_and_intervals;
mod reference_resources_month_view_localization;
mod reference_resources_month_view_radius;
mod reference_resources_month_view_recurring_events;
mod reference_resources_month_view_static_mode;
mod reference_resources_schedule_static_mode;
mod reference_resources_schedule_week_view;
mod reference_resources_week_view_all_day_events;
mod reference_resources_week_view_localization;
mod reference_resources_week_view_multi_hour_intervals;
mod reference_resources_week_view_radius;
mod reference_resources_week_view_recurring_events;
mod reference_resources_week_view_static_mode;
mod reference_resources_week_view_time_range_and_intervals;
mod reference_sankey_chart_custom_colors;
mod reference_sankey_chart_disable_tooltip;
mod reference_sankey_chart_link_opacity;
mod reference_sankey_chart_node_color;
mod reference_sankey_chart_node_width_and_padding;
mod reference_scatter_chart_remove_tooltip;
mod reference_sunburst_chart_chart_size;
mod reference_sunburst_chart_disable_tooltip;
mod reference_sunburst_chart_labels;
mod reference_sunburst_chart_node_color;
mod reference_sunburst_chart_stroke_color;
mod reference_time_grid_disable_specific_controls;
mod reference_time_grid_disabled;
mod reference_time_grid_min_and_max_time;
mod reference_time_input_disabled_state;
mod reference_time_input_with_seconds;
mod reference_time_picker_12_hour_format;
mod reference_time_picker_change_am_pm_labels;
mod reference_time_picker_clearable;
mod reference_time_picker_close_dropdown_on_preset_select;
mod reference_time_picker_control_dropdown_opened_state;
mod reference_time_picker_disabled;
mod reference_time_picker_dropdown_position;
mod reference_time_picker_dropdown_width;
mod reference_time_picker_hours_minutes_seconds_step;
mod reference_time_picker_min_and_max_values;
mod reference_time_picker_paste_events;
mod reference_time_picker_read_only;
mod reference_time_picker_time_presets;
mod reference_time_picker_time_presets_groups;
mod reference_time_picker_time_presets_range;
mod reference_time_picker_with_dropdown;
mod reference_time_picker_with_seconds;
mod reference_time_value_with_seconds;
mod reference_treemap_disable_tooltip;
mod reference_treemap_nested_data;
mod reference_treemap_node_color;
mod reference_treemap_node_stroke;
mod reference_waffle_chart_tooltip;
mod reference_week_view_all_day_events;
mod reference_week_view_highlight_today;
mod reference_week_view_interval_minutes;
mod reference_week_view_localization;
mod reference_week_view_overlapping_events;
mod reference_week_view_radius;
mod reference_week_view_recurring_events;
mod reference_week_view_slot_height;
mod reference_week_view_slot_label_format;
mod reference_week_view_slot_label_interval;
mod reference_week_view_static_mode;
mod reference_week_view_time_range;
mod reference_week_view_week_label_format;
mod reference_week_view_weekday_format;
mod reference_week_view_without_all_day_slots;
mod reference_week_view_without_header;
mod reference_year_picker_allow_deselect;
mod reference_year_picker_controlled_date;
mod reference_year_picker_dates_range;
mod reference_year_picker_default_date;
mod reference_year_picker_input_clearable;
mod reference_year_picker_input_dates_range;
mod reference_year_picker_input_disabled_state;
mod reference_year_picker_input_min_and_max_dates;
mod reference_year_picker_input_multiple_dates;
mod reference_year_picker_input_value_format;
mod reference_year_picker_min_and_max_date;
mod reference_year_picker_multiple_dates;
mod reference_year_picker_single_date_in_range;
mod reference_year_view_first_day_of_week;
mod reference_year_view_hide_outside_days;
mod reference_year_view_highlight_today;
mod reference_year_view_localization;
mod reference_year_view_recurring_events;
mod reference_year_view_static_mode;
mod reference_year_view_without_header;
mod reference_year_view_without_week_days;
mod space_width;
mod splitter_css_units;
mod splitter_line_size;
mod splitter_per_handle_props;
mod splitter_without_handle;
mod stepper_appearance;
mod stepper_custom_icons;
mod stepper_icon_position;
mod stepper_icon_size;
mod stepper_icons_only;
mod stepper_label_position;
mod stepper_styles;
mod stepper_styles_connected;
mod stepper_styles_gradient;
mod table_of_contents_auto_contrast;
mod table_of_contents_styled_index;
mod tabs_custom_style;
mod tabs_deactivation;
mod tabs_inverted_tabs;
mod tabs_position;
mod tabs_styles;
mod tabs_vertical_placement;
mod tabs_with_scroller;
mod toggle_auto_width;
mod tooltip_arrow;
mod tooltip_axes;
mod tooltip_color;
mod tooltip_controlled;
mod tooltip_delay_group;
mod tooltip_floating_tooltip;
mod tooltip_hide_detached;
mod tooltip_inline;
mod tooltip_interactive_tooltip;
mod tooltip_offset;
mod tooltip_transition_effects;
mod tour_beacon_mode;
mod tour_centered_steps;
mod tour_custom_content;
mod tour_custom_labels;
mod tour_overlay_config;
mod tour_styled_tour;
mod tour_tooltip_position;
mod tree_connecting_lines;
mod tree_docs_editor;
mod tree_drag_handle;
mod tree_fuzzy_search;
mod tree_restricted_drop;
mod tree_virtual_rows;
const EXAMPLES: &[Example] = &[
    example!(
        "action-icon",
        action_icon_input_size,
        "action-icon-input-size",
        "Size",
        ""
    ),
    example!(
        "action-icon",
        action_icon_disabled_controls,
        "action-icon-disabled-controls",
        "Disabled state",
        ""
    ),
    example!(
        "button",
        button_disabled_control,
        "button-disabled-control",
        "Disabled state",
        ""
    ),
    example!(
        "button",
        button_loading_variants,
        "button-loading-variants",
        "Loading state",
        ""
    ),
    example!(
        "button",
        button_group_counter,
        "button-group-counter",
        "Button.GroupSection",
        ""
    ),
    example!(
        "action-icon",
        action_icon_group_counter,
        "action-icon-group-counter",
        "ActionIcon.GroupSection",
        ""
    ),
    example!(
        "center",
        center_inline_link,
        "center-inline-link",
        "Inline",
        ""
    ),
    example!(
        "container",
        container_custom_container,
        "container-custom-container",
        "Customize sizes",
        ""
    ),
    example!(
        "container",
        container_responsive_container,
        "container-responsive-container",
        "Responsive max-width",
        ""
    ),
    example!(
        "aspect-ratio",
        aspect_ratio_flex_avatar,
        "aspect-ratio-flex-avatar",
        "Inside flex container",
        ""
    ),
    example!(
        "pagination",
        pagination_siblings,
        "pagination-siblings",
        "Siblings",
        ""
    ),
    example!(
        "pagination",
        pagination_boundaries,
        "pagination-boundaries",
        "Boundaries",
        ""
    ),
    example!(
        "pagination",
        pagination_responsive_layout,
        "pagination-responsive-layout",
        "Responsive layout",
        ""
    ),
    example!(
        "pagination",
        pagination_hide_pages,
        "pagination-hide-pages",
        "Hide pages controls",
        ""
    ),
    example!(
        "pagination",
        pagination_controls_size,
        "pagination-controls-size",
        "Controls size",
        ""
    ),
    example!(
        "pagination",
        pagination_links,
        "pagination-links",
        "Controls as links",
        ""
    ),
    example!(
        "menu",
        menu_link_items,
        "menu-link-items",
        "Custom component as Menu.Item",
        ""
    ),
    example!(
        "tree",
        tree_drag_handle,
        "tree-drag-handle",
        "Drag handle",
        ""
    ),
    example!(
        "tree",
        tree_virtual_rows,
        "tree-virtual-rows",
        "Virtualization",
        ""
    ),
    example!(
        "tree",
        tree_docs_editor,
        "tree-docs-editor",
        "Example: docs navigation editor",
        ""
    ),
    example!(
        "splitter",
        splitter_css_units,
        "splitter-css-units",
        "CSS units",
        ""
    ),
    example!(
        "popover",
        popover_hide_detached,
        "popover-hide-detached",
        "Hide detached",
        ""
    ),
    example!(
        "tooltip",
        tooltip_hide_detached,
        "tooltip-hide-detached",
        "Hide detached",
        ""
    ),
    example!(
        "popover",
        popover_context_target,
        "popover-context-target",
        "Context menu",
        ""
    ),
    example!(
        "tree",
        tree_connecting_lines,
        "tree-connecting-lines",
        "Connecting lines",
        ""
    ),
    example!(
        "tree",
        tree_fuzzy_search,
        "tree-fuzzy-search",
        "Fuzzy search",
        ""
    ),
    example!(
        "tree",
        tree_restricted_drop,
        "tree-restricted-drop",
        "Restricting drop targets",
        ""
    ),
    example!(
        "floating-indicator",
        floating_indicator_multiple_rows,
        "floating-indicator-multiple-rows",
        "Multiple rows",
        ""
    ),
    example!(
        "drawer",
        drawer_transition_callbacks,
        "drawer-transition-callbacks",
        "onExitTransitionEnd and onEnterTransitionEnd",
        ""
    ),
    example!(
        "modal",
        modal_transition_callbacks,
        "modal-transition-callbacks",
        "onExitTransitionEnd and onEnterTransitionEnd",
        ""
    ),
    example!(
        "app-shell",
        app_shell_alternative_layout,
        "app-shell-alternative-layout",
        "AltLayout",
        ""
    ),
    example!(
        "app-shell",
        app_shell_without_transitions,
        "app-shell-without-transitions",
        "NoTransitions",
        ""
    ),
    example!(
        "app-shell",
        app_shell_resizable_alternative,
        "app-shell-resizable-alternative",
        "ResizableAltLayout",
        ""
    ),
    example!(
        "area-chart",
        reference_area_chart_axis_labels,
        "reference-area-chart-axis-labels",
        "Axis labels",
        ""
    ),
    example!(
        "area-chart",
        reference_area_chart_x_axis_offset,
        "reference-area-chart-x-axis-offset",
        "X axis offset",
        ""
    ),
    example!(
        "area-chart",
        reference_area_chart_y_axis_scale,
        "reference-area-chart-y-axis-scale",
        "Y axis scale",
        ""
    ),
    example!(
        "area-chart",
        reference_area_chart_rotate_x_axis_labels,
        "reference-area-chart-rotate-x-axis-labels",
        "Rotate x-axis labels",
        ""
    ),
    example!(
        "area-chart",
        reference_area_chart_value_formatter,
        "reference-area-chart-value-formatter",
        "Value formatter",
        ""
    ),
    example!(
        "area-chart",
        reference_area_chart_change_area_color_depending_on_color_scheme,
        "reference-area-chart-change-area-color-depending-on-color-scheme",
        "Change area color depending on color scheme",
        ""
    ),
    example!(
        "area-chart",
        reference_area_chart_grid_and_text_colors,
        "reference-area-chart-grid-and-text-colors",
        "Grid and text colors",
        ""
    ),
    example!(
        "bar-chart",
        reference_bar_chart_axis_labels,
        "reference-bar-chart-axis-labels",
        "Axis labels",
        ""
    ),
    example!(
        "bar-chart",
        reference_bar_chart_x_axis_offset,
        "reference-bar-chart-x-axis-offset",
        "X axis offset",
        ""
    ),
    example!(
        "bar-chart",
        reference_bar_chart_y_axis_scale,
        "reference-bar-chart-y-axis-scale",
        "Y axis scale",
        ""
    ),
    example!(
        "bar-chart",
        reference_bar_chart_value_formatter,
        "reference-bar-chart-value-formatter",
        "Value formatter",
        ""
    ),
    example!(
        "bar-chart",
        reference_bar_chart_change_bar_color_depending_on_color_scheme,
        "reference-bar-chart-change-bar-color-depending-on-color-scheme",
        "Change bar color depending on color scheme",
        ""
    ),
    example!(
        "bar-chart",
        reference_bar_chart_grid_and_text_colors,
        "reference-bar-chart-grid-and-text-colors",
        "Grid and text colors",
        ""
    ),
    example!(
        "composite-chart",
        reference_composite_chart_axis_labels,
        "reference-composite-chart-axis-labels",
        "Axis labels",
        ""
    ),
    example!(
        "composite-chart",
        reference_composite_chart_x_axis_offset,
        "reference-composite-chart-x-axis-offset",
        "X axis offset",
        ""
    ),
    example!(
        "composite-chart",
        reference_composite_chart_y_axis_scale,
        "reference-composite-chart-y-axis-scale",
        "Y axis scale",
        ""
    ),
    example!(
        "composite-chart",
        reference_composite_chart_rotate_x_axis_labels,
        "reference-composite-chart-rotate-x-axis-labels",
        "Rotate x-axis labels",
        ""
    ),
    example!(
        "composite-chart",
        reference_composite_chart_value_formatter,
        "reference-composite-chart-value-formatter",
        "Value formatter",
        ""
    ),
    example!(
        "composite-chart",
        reference_composite_chart_change_chart_color_depending_on_color_scheme,
        "reference-composite-chart-change-chart-color-depending-on-color-scheme",
        "Change chart color depending on color scheme",
        ""
    ),
    example!(
        "composite-chart",
        reference_composite_chart_grid_and_text_colors,
        "reference-composite-chart-grid-and-text-colors",
        "Grid and text colors",
        ""
    ),
    example!(
        "line-chart",
        reference_line_chart_axis_labels,
        "reference-line-chart-axis-labels",
        "Axis labels",
        ""
    ),
    example!(
        "line-chart",
        reference_line_chart_x_axis_offset,
        "reference-line-chart-x-axis-offset",
        "X axis offset",
        ""
    ),
    example!(
        "line-chart",
        reference_line_chart_y_axis_scale,
        "reference-line-chart-y-axis-scale",
        "Y axis scale",
        ""
    ),
    example!(
        "line-chart",
        reference_line_chart_value_formatter,
        "reference-line-chart-value-formatter",
        "Value formatter",
        ""
    ),
    example!(
        "line-chart",
        reference_line_chart_change_line_color_depending_on_color_scheme,
        "reference-line-chart-change-line-color-depending-on-color-scheme",
        "Change line color depending on color scheme",
        ""
    ),
    example!(
        "line-chart",
        reference_line_chart_grid_and_text_colors,
        "reference-line-chart-grid-and-text-colors",
        "Grid and text colors",
        ""
    ),
    example!(
        "tabs",
        tabs_deactivation,
        "tabs-deactivation",
        "Tab deactivation",
        ""
    ),
    example!(
        "pagination",
        pagination_start_value,
        "pagination-start-value",
        "Start value",
        ""
    ),
    example!(
        "pagination",
        pagination_change_icons,
        "pagination-change-icons",
        "Change icons",
        ""
    ),
    example!(
        "table-of-contents",
        table_of_contents_auto_contrast,
        "table-of-contents-auto-contrast",
        "autoContrast",
        ""
    ),
    example!(
        "table-of-contents",
        table_of_contents_styled_index,
        "table-of-contents-styled-index",
        "Styles API",
        ""
    ),
    example!(
        "stepper",
        stepper_custom_icons,
        "stepper-custom-icons",
        "With custom icons",
        ""
    ),
    example!(
        "stepper",
        stepper_icons_only,
        "stepper-icons-only",
        "With custom icons",
        ""
    ),
    example!(
        "aspect-ratio",
        aspect_ratio_map_embed,
        "aspect-ratio-map-embed",
        "Map embed",
        ""
    ),
    example!(
        "aspect-ratio",
        aspect_ratio_video_embed,
        "aspect-ratio-video-embed",
        "Video embed",
        ""
    ),
    example!(
        "floating-window",
        floating_window_constrain_offset,
        "floating-window-constrain-offset",
        "Constrain offset",
        ""
    ),
    example!(
        "floating-window",
        floating_window_lock_axis,
        "floating-window-lock-axis",
        "Lock axis",
        ""
    ),
    example!(
        "floating-window",
        floating_window_window_handle,
        "floating-window-window-handle",
        "use-floating-window hook",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_time_presets,
        "reference-time-picker-time-presets",
        "Time presets",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_time_presets_groups,
        "reference-time-picker-time-presets-groups",
        "Time presets groups",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_time_presets_range,
        "reference-time-picker-time-presets-range",
        "Time presets range",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_close_dropdown_on_preset_select,
        "reference-time-picker-close-dropdown-on-preset-select",
        "Close dropdown on preset select",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_dropdown_position,
        "reference-time-picker-dropdown-position",
        "Dropdown position",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_dropdown_width,
        "reference-time-picker-dropdown-width",
        "Dropdown width",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_min_and_max_values,
        "reference-time-picker-min-and-max-values",
        "Min and max values",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_paste_events,
        "reference-time-picker-paste-events",
        "Paste events",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_control_dropdown_opened_state,
        "reference-time-picker-control-dropdown-opened-state",
        "Control dropdown opened state",
        ""
    ),
    example!(
        "gauge-chart",
        reference_gauge_chart_sections,
        "reference-gauge-chart-sections",
        "Sections",
        ""
    ),
    example!(
        "gauge-chart",
        reference_gauge_chart_target_marker,
        "reference-gauge-chart-target-marker",
        "Target marker",
        ""
    ),
    example!(
        "gauge-chart",
        reference_gauge_chart_start_and_end_angle,
        "reference-gauge-chart-start-and-end-angle",
        "Start and end angle",
        ""
    ),
    example!(
        "gauge-chart",
        reference_gauge_chart_thickness,
        "reference-gauge-chart-thickness",
        "Thickness",
        ""
    ),
    example!(
        "gauge-chart",
        reference_gauge_chart_custom_label,
        "reference-gauge-chart-custom-label",
        "Custom label",
        ""
    ),
    example!(
        "gauge-chart",
        reference_gauge_chart_round_caps,
        "reference-gauge-chart-round-caps",
        "Round caps",
        ""
    ),
    example!(
        "sankey-chart",
        reference_sankey_chart_node_color,
        "reference-sankey-chart-node-color",
        "Node color",
        ""
    ),
    example!(
        "sankey-chart",
        reference_sankey_chart_link_opacity,
        "reference-sankey-chart-link-opacity",
        "Link opacity",
        ""
    ),
    example!(
        "sankey-chart",
        reference_sankey_chart_node_width_and_padding,
        "reference-sankey-chart-node-width-and-padding",
        "Node width and padding",
        ""
    ),
    example!(
        "sunburst-chart",
        reference_sunburst_chart_stroke_color,
        "reference-sunburst-chart-stroke-color",
        "Stroke color",
        ""
    ),
    example!(
        "sunburst-chart",
        reference_sunburst_chart_labels,
        "reference-sunburst-chart-labels",
        "Labels",
        ""
    ),
    example!(
        "sunburst-chart",
        reference_sunburst_chart_node_color,
        "reference-sunburst-chart-node-color",
        "Node color",
        ""
    ),
    example!(
        "treemap",
        reference_treemap_node_stroke,
        "reference-treemap-node-stroke",
        "Node stroke",
        ""
    ),
    example!(
        "container",
        container_grid_strategy,
        "container-grid-strategy",
        "Grid strategy",
        ""
    ),
    example!(
        "popover",
        popover_focus_trap,
        "popover-focus-trap",
        "Focus trap",
        ""
    ),
    example!(
        "popover",
        popover_inline_elements,
        "popover-inline-elements",
        "Inline elements",
        ""
    ),
    example!(
        "popover",
        popover_with_overlay,
        "popover-with-overlay",
        "With overlay",
        ""
    ),
    example!(
        "stepper",
        stepper_styles_connected,
        "stepper-styles-connected",
        "Styles API",
        ""
    ),
    example!(
        "stepper",
        stepper_styles_gradient,
        "stepper-styles-gradient",
        "Styles API",
        ""
    ),
    example!(
        "tabs",
        tabs_custom_style,
        "tabs-custom-style",
        "Styles API",
        ""
    ),
    example!(
        "pagination",
        pagination_compound_components,
        "pagination-compound-components",
        "Compound components",
        ""
    ),
    example!(
        "menu",
        menu_custom_target,
        "menu-custom-target",
        "Custom component as target",
        ""
    ),
    example!(
        "tooltip",
        tooltip_floating_tooltip,
        "tooltip-floating-tooltip",
        "Floating tooltip",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_with_seconds,
        "reference-time-picker-with-seconds",
        "With seconds",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_12_hour_format,
        "reference-time-picker-12-hour-format",
        "12-hour format",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_change_am_pm_labels,
        "reference-time-picker-change-am-pm-labels",
        "Change am/pm labels",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_with_dropdown,
        "reference-time-picker-with-dropdown",
        "With dropdown",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_hours_minutes_seconds_step,
        "reference-time-picker-hours-minutes-seconds-step",
        "Hours/minutes/seconds step",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_clearable,
        "reference-time-picker-clearable",
        "Clearable",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_disabled,
        "reference-time-picker-disabled",
        "Disabled",
        ""
    ),
    example!(
        "time-picker",
        reference_time_picker_read_only,
        "reference-time-picker-read-only",
        "Read only",
        ""
    ),
    example!(
        "time-input",
        reference_time_input_with_seconds,
        "reference-time-input-with-seconds",
        "With seconds",
        ""
    ),
    example!(
        "time-input",
        reference_time_input_disabled_state,
        "reference-time-input-disabled-state",
        "Disabled state",
        ""
    ),
    example!(
        "mini-calendar",
        reference_mini_calendar_number_of_days,
        "reference-mini-calendar-number-of-days",
        "Number of days",
        ""
    ),
    example!(
        "mini-calendar",
        reference_mini_calendar_getdayprops,
        "reference-mini-calendar-getdayprops",
        "getDayProps",
        ""
    ),
    example!(
        "mini-calendar",
        reference_mini_calendar_min_and_max_dates,
        "reference-mini-calendar-min-and-max-dates",
        "Min and max dates",
        ""
    ),
    example!(
        "date-input",
        reference_date_input_disabled_state,
        "reference-date-input-disabled-state",
        "Disabled state",
        ""
    ),
    example!(
        "date-input",
        reference_date_input_date_parser,
        "reference-date-input-date-parser",
        "Date parser",
        ""
    ),
    example!(
        "date-input",
        reference_date_input_min_and_max_date,
        "reference-date-input-min-and-max-date",
        "Min and max date",
        ""
    ),
    example!(
        "time-grid",
        reference_time_grid_min_and_max_time,
        "reference-time-grid-min-and-max-time",
        "Min and max time",
        ""
    ),
    example!(
        "time-grid",
        reference_time_grid_disable_specific_controls,
        "reference-time-grid-disable-specific-controls",
        "Disable specific controls",
        ""
    ),
    example!(
        "time-grid",
        reference_time_grid_disabled,
        "reference-time-grid-disabled",
        "Disabled",
        ""
    ),
    example!(
        "time-value",
        reference_time_value_with_seconds,
        "reference-time-value-with-seconds",
        "With seconds",
        ""
    ),
    example!(
        "tour",
        tour_custom_content,
        "tour-custom-content",
        "Custom content",
        ""
    ),
    example!(
        "modal",
        modal_compound_components,
        "modal-compound-components",
        "Compound components",
        ""
    ),
    example!(
        "drawer",
        drawer_compound_components,
        "drawer-compound-components",
        "Compound components",
        ""
    ),
    example!(
        "modal",
        modal_change_transitions,
        "modal-change-transitions",
        "Change transitions",
        ""
    ),
    example!(
        "drawer",
        drawer_change_transition,
        "drawer-change-transition",
        "Change transition",
        ""
    ),
    example!(
        "splitter",
        splitter_per_handle_props,
        "splitter-per-handle-props",
        "Per-handle props",
        ""
    ),
    example!(
        "tabs",
        tabs_with_scroller,
        "tabs-with-scroller",
        "Example with Scroller component",
        ""
    ),
    example!(
        "hover-card",
        hover_card_delay_group,
        "hover-card-delay-group",
        "HoverCard delay group",
        ""
    ),
    example!(
        "hover-card",
        hover_card_dropdown_role,
        "hover-card-dropdown-role",
        "Dropdown role",
        ""
    ),
    example!(
        "tour",
        tour_centered_steps,
        "tour-centered-steps",
        "Steps without target",
        ""
    ),
    example!(
        "tour",
        tour_tooltip_position,
        "tour-tooltip-position",
        "Tooltip position",
        ""
    ),
    example!(
        "tour",
        tour_custom_labels,
        "tour-custom-labels",
        "Custom labels",
        ""
    ),
    example!(
        "tour",
        tour_styled_tour,
        "tour-styled-tour",
        "Styles API",
        ""
    ),
    example!(
        "tour",
        tour_beacon_mode,
        "tour-beacon-mode",
        "Beacon mode",
        ""
    ),
    example!(
        "tooltip",
        tooltip_transition_effects,
        "tooltip-transition-effects",
        "Change transition",
        ""
    ),
    example!(
        "menu",
        menu_custom_check_icon,
        "menu-custom-check-icon",
        "Custom check icon",
        ""
    ),
    example!(
        "menu",
        menu_context_menu,
        "menu-context-menu",
        "Context menu",
        ""
    ),
    example!("menu", menu_hover, "menu-hover", "Show menu on hover", ""),
    example!(
        "menu",
        menu_click_hover,
        "menu-click-hover",
        "Click or hover",
        ""
    ),
    example!(
        "menu",
        menu_transitions,
        "menu-transitions",
        "Transitions",
        ""
    ),
    example!(
        "tooltip",
        tooltip_interactive_tooltip,
        "tooltip-interactive-tooltip",
        "Interactive tooltip",
        ""
    ),
    example!("tooltip", tooltip_inline, "tooltip-inline", "Inline", ""),
    example!(
        "tooltip",
        tooltip_delay_group,
        "tooltip-delay-group",
        "Tooltip delay group",
        ""
    ),
    example!(
        "pagination",
        pagination_auto_contrast,
        "pagination-auto-contrast",
        "autoContrast",
        ""
    ),
    example!(
        "resources-day-view",
        reference_resources_day_view_static_mode,
        "reference-resources-day-view-static-mode",
        "Static mode",
        ""
    ),
    example!(
        "resources-day-view",
        reference_resources_day_view_recurring_events,
        "reference-resources-day-view-recurring-events",
        "Recurring events",
        ""
    ),
    example!(
        "resources-day-view",
        reference_resources_day_view_localization,
        "reference-resources-day-view-localization",
        "Localization",
        ""
    ),
    example!(
        "resources-day-view",
        reference_resources_day_view_radius,
        "reference-resources-day-view-radius",
        "Radius",
        ""
    ),
    example!(
        "resources-day-view",
        reference_resources_day_view_all_day_events,
        "reference-resources-day-view-all-day-events",
        "All-day events",
        ""
    ),
    example!(
        "resources-day-view",
        reference_resources_day_view_time_range_and_intervals,
        "reference-resources-day-view-time-range-and-intervals",
        "Time range and intervals",
        ""
    ),
    example!(
        "resources-day-view",
        reference_resources_day_view_multi_hour_intervals,
        "reference-resources-day-view-multi-hour-intervals",
        "Multi-hour intervals",
        ""
    ),
    example!(
        "resources-week-view",
        reference_resources_week_view_static_mode,
        "reference-resources-week-view-static-mode",
        "Static mode",
        ""
    ),
    example!(
        "resources-week-view",
        reference_resources_week_view_recurring_events,
        "reference-resources-week-view-recurring-events",
        "Recurring events",
        ""
    ),
    example!(
        "resources-week-view",
        reference_resources_week_view_localization,
        "reference-resources-week-view-localization",
        "Localization",
        ""
    ),
    example!(
        "resources-week-view",
        reference_resources_week_view_radius,
        "reference-resources-week-view-radius",
        "Radius",
        ""
    ),
    example!(
        "resources-week-view",
        reference_resources_week_view_all_day_events,
        "reference-resources-week-view-all-day-events",
        "All-day events",
        ""
    ),
    example!(
        "resources-week-view",
        reference_resources_week_view_time_range_and_intervals,
        "reference-resources-week-view-time-range-and-intervals",
        "Time range and intervals",
        ""
    ),
    example!(
        "resources-week-view",
        reference_resources_week_view_multi_hour_intervals,
        "reference-resources-week-view-multi-hour-intervals",
        "Multi-hour intervals",
        ""
    ),
    example!(
        "resources-month-view",
        reference_resources_month_view_static_mode,
        "reference-resources-month-view-static-mode",
        "Static mode",
        ""
    ),
    example!(
        "resources-month-view",
        reference_resources_month_view_recurring_events,
        "reference-resources-month-view-recurring-events",
        "Recurring events",
        ""
    ),
    example!(
        "resources-month-view",
        reference_resources_month_view_localization,
        "reference-resources-month-view-localization",
        "Localization",
        ""
    ),
    example!(
        "resources-month-view",
        reference_resources_month_view_radius,
        "reference-resources-month-view-radius",
        "Radius",
        ""
    ),
    example!(
        "resources-schedule",
        reference_resources_schedule_static_mode,
        "reference-resources-schedule-static-mode",
        "Static mode",
        ""
    ),
    example!(
        "resources-schedule",
        reference_resources_schedule_week_view,
        "reference-resources-schedule-week-view",
        "Week view",
        ""
    ),
    example!(
        "recurring-events",
        reference_recurring_events_dayview_demo,
        "reference-recurring-events-dayview-demo",
        "DayView demo",
        ""
    ),
    example!(
        "recurring-events",
        reference_recurring_events_weekview_demo,
        "reference-recurring-events-weekview-demo",
        "WeekView demo",
        ""
    ),
    example!(
        "recurring-events",
        reference_recurring_events_monthview_demo,
        "reference-recurring-events-monthview-demo",
        "MonthView demo",
        ""
    ),
    example!(
        "recurring-events",
        reference_recurring_events_yearview_demo,
        "reference-recurring-events-yearview-demo",
        "YearView demo",
        ""
    ),
    example!(
        "day-view",
        reference_day_view_overlapping_events,
        "reference-day-view-overlapping-events",
        "Overlapping events",
        ""
    ),
    example!(
        "week-view",
        reference_week_view_overlapping_events,
        "reference-week-view-overlapping-events",
        "Overlapping events",
        ""
    ),
    example!(
        "year-view",
        reference_year_view_highlight_today,
        "reference-year-view-highlight-today",
        "Highlight today",
        ""
    ),
    example!(
        "week-view",
        reference_week_view_highlight_today,
        "reference-week-view-highlight-today",
        "Highlight today",
        ""
    ),
    example!(
        "year-view",
        reference_year_view_hide_outside_days,
        "reference-year-view-hide-outside-days",
        "Hide outside days",
        ""
    ),
    example!(
        "year-view",
        reference_year_view_static_mode,
        "reference-year-view-static-mode",
        "Static mode",
        ""
    ),
    example!(
        "year-view",
        reference_year_view_localization,
        "reference-year-view-localization",
        "Localization",
        ""
    ),
    example!(
        "year-view",
        reference_year_view_first_day_of_week,
        "reference-year-view-first-day-of-week",
        "First day of week",
        ""
    ),
    example!(
        "year-view",
        reference_year_view_recurring_events,
        "reference-year-view-recurring-events",
        "Recurring events",
        ""
    ),
    example!(
        "day-view",
        reference_day_view_radius,
        "reference-day-view-radius",
        "Radius",
        ""
    ),
    example!(
        "week-view",
        reference_week_view_radius,
        "reference-week-view-radius",
        "Radius",
        ""
    ),
    example!(
        "month-view",
        reference_month_view_radius,
        "reference-month-view-radius",
        "Radius",
        ""
    ),
    example!(
        "menu",
        menu_submenus,
        "menu-submenus",
        "Submenus",
        "Open Products or Orders to enter its submenu."
    ),
    example!(
        "tabs",
        tabs_inverted_tabs,
        "tabs-inverted-tabs",
        "Inverted tabs",
        "The panel appears above its tab list."
    ),
    example!(
        "stepper",
        stepper_label_position,
        "stepper-label-position",
        "Label position",
        "Select each step to update the active marker."
    ),
    example!(
        "stepper",
        stepper_icon_position,
        "stepper-icon-position",
        "Icon position",
        "Select each step to update the active marker."
    ),
    example!(
        "button",
        button_styles_api,
        "button-styles-api",
        "Styles API",
        "Style the button sections to display a count and a trailing arrow."
    ),
    example!(
        "bullet-chart",
        reference_bullet_chart_vertical_orientation,
        "reference-bullet-chart-vertical-orientation",
        "Vertical orientation",
        ""
    ),
    example!(
        "bullet-chart",
        reference_bullet_chart_bar_color,
        "reference-bullet-chart-bar-color",
        "Bar color",
        ""
    ),
    example!(
        "bullet-chart",
        reference_bullet_chart_value_formatter,
        "reference-bullet-chart-value-formatter",
        "Value formatter",
        ""
    ),
    example!(
        "bullet-chart",
        reference_bullet_chart_label,
        "reference-bullet-chart-label",
        "Label",
        ""
    ),
    example!(
        "modal",
        modal_initial_focus,
        "modal-initial-focus",
        "Initial focus",
        "Opening the panel focuses its second input."
    ),
    example!(
        "drawer",
        drawer_initial_focus,
        "drawer-initial-focus",
        "Initial focus",
        "Opening the panel focuses its second input."
    ),
    example!(
        "modal",
        modal_size_auto,
        "modal-size-auto",
        "Size auto",
        "Add and remove badges to adjust the width of the dialog."
    ),
    example!(
        "modal",
        modal_change_offsets,
        "modal-change-offsets",
        "Change offsets",
        "Open the dialog with a one-viewport-height-percent top offset and no horizontal inset."
    ),
    example!(
        "action-icon",
        action_icon_add_custom_sizes,
        "action-icon-add-custom-sizes",
        "Add custom sizes",
        "Compare two additional icon target sizes outside the default scale."
    ),
    example!(
        "button",
        button_full_width,
        "button-full-width",
        "Full width",
        "The button stretches across the available space."
    ),
    example!(
        "button",
        button_disabled_state_when_button_is_link,
        "button-disabled-state-when-button-is-link",
        "Disabled state when Button is link",
        "The link retains its button appearance and ignores activation."
    ),
    example!(
        "button",
        button_customize_disabled_styles,
        "button-customize-disabled-styles",
        "Customize disabled styles",
        "The disabled button retains a visible border and a transparent background."
    ),
    example!(
        "button",
        button_disabled_button_with_tooltip,
        "button-disabled-button-with-tooltip",
        "Disabled button with Tooltip",
        "Hover or focus the disabled-looking target to read its explanation."
    ),
    example!(
        "button",
        button_custom_variants,
        "button-custom-variants",
        "Custom variants",
        "Two button variants define their own backgrounds and text colors."
    ),
    example!(
        "button",
        button_autocontrast,
        "button-autocontrast",
        "autoContrast",
        "Compare the default and automatically selected foreground colors on lime."
    ),
    example!(
        "anchor",
        anchor_underline,
        "anchor-underline",
        "Underline",
        "Compare the four link underline behaviors and hover each link."
    ),
    example!(
        "toggle",
        toggle_auto_width,
        "toggle-auto-width",
        "Auto width",
        "Each toggle fits its label and maintains its own pressed state."
    ),
    example!(
        "splitter",
        splitter_without_handle,
        "splitter-without-handle",
        "Without handle",
        "Resize the two panes by dragging the divider or using the arrow keys."
    ),
    example!(
        "tooltip",
        tooltip_controlled,
        "tooltip-controlled",
        "Controlled",
        "The button controls when the keyboard shortcut is visible."
    ),
    example!(
        "action-bar",
        action_bar_close_on_escape,
        "action-bar-close-on-escape",
        "Close on Escape",
        "Open the action bar and press Escape or use its Close button."
    ),
    example!(
        "overlay",
        overlay_gradient,
        "overlay-gradient",
        "Gradient",
        "Toggle the gradient over the image."
    ),
    example!(
        "button",
        button_loader_props,
        "button-loader-props",
        "Loader props",
        "A dots loader replaces the button label while the button is busy."
    ),
    example!(
        "action-icon",
        action_icon_loader_props,
        "action-icon-loader-props",
        "Loader props",
        "The large icon action displays a dots loader."
    ),
    example!(
        "action-icon",
        action_icon_disabled_state_when_actionicon_is_link,
        "action-icon-disabled-state-when-actionicon-is-link",
        "Disabled state when ActionIcon is link",
        "The disabled link keeps its accessible label and suppresses activation."
    ),
    example!(
        "action-icon",
        action_icon_customize_disabled_styles,
        "action-icon-customize-disabled-styles",
        "Customize disabled styles",
        "The unavailable action remains outlined."
    ),
    example!(
        "action-icon",
        action_icon_disabled_button_with_tooltip,
        "action-icon-disabled-button-with-tooltip",
        "Disabled button with Tooltip",
        "The disabled-looking action can be focused to read its explanation."
    ),
    example!(
        "action-icon",
        action_icon_autocontrast,
        "action-icon-autocontrast",
        "autoContrast",
        "Compare foreground contrast on the two lime icon buttons."
    ),
    example!(
        "action-icon",
        action_icon_add_custom_variants,
        "action-icon-add-custom-variants",
        "Add custom variants",
        "Two icon actions show custom danger and primary variants."
    ),
    example!(
        "pagination",
        pagination_example_with_chunked_content,
        "pagination-example-with-chunked-content",
        "Example with chunked content",
        "Change pages to display five records from the thirty-item list."
    ),
    example!(
        "nav-link",
        nav_link_autocontrast,
        "nav-link-autocontrast",
        "autoContrast",
        "Compare foreground colors for active navigation links."
    ),
    example!(
        "loading-overlay",
        loading_overlay_custom_inline_loaders,
        "loading-overlay-custom-inline-loaders",
        "Custom inline loaders",
        "Toggle the loading layer over the form to see a custom inline indicator."
    ),
    example!(
        "matrix-chart",
        reference_matrix_chart_legend,
        "reference-matrix-chart-legend",
        "Legend",
        ""
    ),
    example!(
        "matrix-chart",
        reference_matrix_chart_labels,
        "reference-matrix-chart-labels",
        "Labels",
        ""
    ),
    example!(
        "matrix-chart",
        reference_matrix_chart_colors,
        "reference-matrix-chart-colors",
        "Colors",
        ""
    ),
    example!(
        "matrix-chart",
        reference_matrix_chart_cell_size_and_gap,
        "reference-matrix-chart-cell-size-and-gap",
        "Cell size and gap",
        ""
    ),
    example!(
        "donut-chart",
        reference_donut_chart_legend,
        "reference-donut-chart-legend",
        "Legend",
        ""
    ),
    example!(
        "donut-chart",
        reference_donut_chart_segments_stroke,
        "reference-donut-chart-segments-stroke",
        "Segments stroke",
        ""
    ),
    example!(
        "donut-chart",
        reference_donut_chart_start_and_end_angle,
        "reference-donut-chart-start-and-end-angle",
        "Start and end angle",
        ""
    ),
    example!(
        "donut-chart",
        reference_donut_chart_without_tooltip,
        "reference-donut-chart-without-tooltip",
        "Without tooltip",
        ""
    ),
    example!(
        "pie-chart",
        reference_pie_chart_legend,
        "reference-pie-chart-legend",
        "Legend",
        ""
    ),
    example!(
        "pie-chart",
        reference_pie_chart_segments_stroke,
        "reference-pie-chart-segments-stroke",
        "Segments stroke",
        ""
    ),
    example!(
        "pie-chart",
        reference_pie_chart_start_and_end_angle,
        "reference-pie-chart-start-and-end-angle",
        "Start and end angle",
        ""
    ),
    example!(
        "funnel-chart",
        reference_funnel_chart_legend,
        "reference-funnel-chart-legend",
        "Legend",
        ""
    ),
    example!(
        "funnel-chart",
        reference_funnel_chart_segments_stroke,
        "reference-funnel-chart-segments-stroke",
        "Segments stroke",
        ""
    ),
    example!(
        "funnel-chart",
        reference_funnel_chart_without_tooltip,
        "reference-funnel-chart-without-tooltip",
        "Without tooltip",
        ""
    ),
    example!(
        "donut-chart",
        reference_donut_chart_chart_label,
        "reference-donut-chart-chart-label",
        "Chart label",
        ""
    ),
    example!(
        "pie-chart",
        reference_pie_chart_enable_tooltip,
        "reference-pie-chart-enable-tooltip",
        "Enable tooltip",
        ""
    ),
    example!(
        "radial-bar-chart",
        reference_radial_bar_chart_legend,
        "reference-radial-bar-chart-legend",
        "Legend",
        ""
    ),
    example!(
        "radial-bar-chart",
        reference_radial_bar_chart_hide_tooltip,
        "reference-radial-bar-chart-hide-tooltip",
        "Hide tooltip",
        ""
    ),
    example!(
        "sunburst-chart",
        reference_sunburst_chart_disable_tooltip,
        "reference-sunburst-chart-disable-tooltip",
        "Disable tooltip",
        ""
    ),
    example!(
        "sankey-chart",
        reference_sankey_chart_disable_tooltip,
        "reference-sankey-chart-disable-tooltip",
        "Disable tooltip",
        ""
    ),
    example!(
        "treemap",
        reference_treemap_disable_tooltip,
        "reference-treemap-disable-tooltip",
        "Disable tooltip",
        ""
    ),
    example!(
        "bubble-chart",
        reference_bubble_chart_remove_tooltip,
        "reference-bubble-chart-remove-tooltip",
        "Remove tooltip",
        ""
    ),
    example!(
        "scatter-chart",
        reference_scatter_chart_remove_tooltip,
        "reference-scatter-chart-remove-tooltip",
        "Remove tooltip",
        ""
    ),
    example!(
        "waffle-chart",
        reference_waffle_chart_tooltip,
        "reference-waffle-chart-tooltip",
        "Tooltip",
        ""
    ),
    example!(
        "matrix-chart",
        reference_matrix_chart_tooltip,
        "reference-matrix-chart-tooltip",
        "Tooltip",
        ""
    ),
    example!(
        "matrix-chart",
        reference_matrix_chart_values_domain,
        "reference-matrix-chart-values-domain",
        "Values domain",
        ""
    ),
    example!(
        "sunburst-chart",
        reference_sunburst_chart_chart_size,
        "reference-sunburst-chart-chart-size",
        "Chart size",
        ""
    ),
    example!(
        "sankey-chart",
        reference_sankey_chart_custom_colors,
        "reference-sankey-chart-custom-colors",
        "Custom colors",
        ""
    ),
    example!(
        "treemap",
        reference_treemap_node_color,
        "reference-treemap-node-color",
        "Node color",
        ""
    ),
    example!(
        "treemap",
        reference_treemap_nested_data,
        "reference-treemap-nested-data",
        "Nested data",
        ""
    ),
    example!(
        "bars-list",
        reference_bars_list_bar_color,
        "reference-bars-list-bar-color",
        "Bar color",
        ""
    ),
    example!(
        "bullet-chart",
        reference_bullet_chart_without_target,
        "reference-bullet-chart-without-target",
        "Without target",
        ""
    ),
    example!(
        "bullet-chart",
        reference_bullet_chart_chart_size,
        "reference-bullet-chart-chart-size",
        "Chart size",
        ""
    ),
    example!(
        "heatmap",
        reference_heatmap_with_tooltip,
        "reference-heatmap-with-tooltip",
        "With tooltip",
        ""
    ),
    example!(
        "heatmap",
        reference_heatmap_change_colors,
        "reference-heatmap-change-colors",
        "Change colors",
        ""
    ),
    example!(
        "heatmap",
        reference_heatmap_weekdays_and_months_labels,
        "reference-heatmap-weekdays-and-months-labels",
        "Weekdays and months labels",
        ""
    ),
    example!(
        "heatmap",
        reference_heatmap_legend,
        "reference-heatmap-legend",
        "Legend",
        ""
    ),
    example!(
        "heatmap",
        reference_heatmap_first_day_of_week,
        "reference-heatmap-first-day-of-week",
        "First day of week",
        ""
    ),
    example!(
        "heatmap",
        reference_heatmap_change_labels_text,
        "reference-heatmap-change-labels-text",
        "Change labels text",
        ""
    ),
    example!(
        "day-view",
        reference_day_view_time_range,
        "reference-day-view-time-range",
        "Time range",
        "Show the working day from 09:00 to 17:00."
    ),
    example!(
        "day-view",
        reference_day_view_interval_minutes,
        "reference-day-view-interval-minutes",
        "Interval minutes",
        "The time scale uses fifteen-minute intervals."
    ),
    example!(
        "day-view",
        reference_day_view_slot_height,
        "reference-day-view-slot-height",
        "Slot height",
        "Each hourly slot is 80 pixels high."
    ),
    example!(
        "day-view",
        reference_day_view_without_header,
        "reference-day-view-without-header",
        "Without header",
        "The schedule grid is shown without navigation controls."
    ),
    example!(
        "day-view",
        reference_day_view_static_mode,
        "reference-day-view-static-mode",
        "Static mode",
        "The schedule can be browsed with editing and selection disabled."
    ),
    example!(
        "day-view",
        reference_day_view_localization,
        "reference-day-view-localization",
        "Localization",
        "Weekdays and calendar date labels use Chinese."
    ),
    example!(
        "day-view",
        reference_day_view_all_day_events,
        "reference-day-view-all-day-events",
        "All-day events",
        "The release event occupies the all-day section above the timed bookings."
    ),
    example!(
        "day-view",
        reference_day_view_recurring_events,
        "reference-day-view-recurring-events",
        "Recurring events",
        "The daily standup repeats seven times and skips November 5."
    ),
    example!(
        "week-view",
        reference_week_view_time_range,
        "reference-week-view-time-range",
        "Time range",
        "Show the working day from 09:00 to 17:00."
    ),
    example!(
        "week-view",
        reference_week_view_interval_minutes,
        "reference-week-view-interval-minutes",
        "Interval minutes",
        "The time scale uses fifteen-minute intervals."
    ),
    example!(
        "week-view",
        reference_week_view_slot_height,
        "reference-week-view-slot-height",
        "Slot height",
        "Each hourly slot is 80 pixels high."
    ),
    example!(
        "week-view",
        reference_week_view_without_all_day_slots,
        "reference-week-view-without-all-day-slots",
        "Without all-day slots",
        "The all-day row is hidden."
    ),
    example!(
        "week-view",
        reference_week_view_without_header,
        "reference-week-view-without-header",
        "Without header",
        "The schedule grid is shown without navigation controls."
    ),
    example!(
        "week-view",
        reference_week_view_static_mode,
        "reference-week-view-static-mode",
        "Static mode",
        "The schedule can be browsed with editing and selection disabled."
    ),
    example!(
        "week-view",
        reference_week_view_localization,
        "reference-week-view-localization",
        "Localization",
        "Weekdays and calendar date labels use Chinese."
    ),
    example!(
        "week-view",
        reference_week_view_all_day_events,
        "reference-week-view-all-day-events",
        "All-day events",
        "The release event occupies the all-day section above the timed bookings."
    ),
    example!(
        "week-view",
        reference_week_view_recurring_events,
        "reference-week-view-recurring-events",
        "Recurring events",
        "The daily standup repeats seven times and skips November 5."
    ),
    example!(
        "month-view",
        reference_month_view_without_header,
        "reference-month-view-without-header",
        "Without header",
        "The schedule grid is shown without navigation controls."
    ),
    example!(
        "month-view",
        reference_month_view_without_week_days,
        "reference-month-view-without-week-days",
        "Without week days",
        "The calendar omits weekday headings."
    ),
    example!(
        "month-view",
        reference_month_view_without_outside_days,
        "reference-month-view-without-outside-days",
        "Without outside days",
        "The first and last rows omit dates outside the current month."
    ),
    example!(
        "month-view",
        reference_month_view_highlight_today,
        "reference-month-view-highlight-today",
        "Highlight today",
        "November 4 is marked as today."
    ),
    example!(
        "month-view",
        reference_month_view_first_day_of_week,
        "reference-month-view-first-day-of-week",
        "First day of week",
        "Sunday is the first day of the displayed week."
    ),
    example!(
        "month-view",
        reference_month_view_max_events_per_day,
        "reference-month-view-max-events-per-day",
        "Max events per day",
        "A day with multiple events starts with one visible event and an expansion control."
    ),
    example!(
        "month-view",
        reference_month_view_static_mode,
        "reference-month-view-static-mode",
        "Static mode",
        "The schedule can be browsed with editing and selection disabled."
    ),
    example!(
        "month-view",
        reference_month_view_localization,
        "reference-month-view-localization",
        "Localization",
        "Weekdays and calendar date labels use Chinese."
    ),
    example!(
        "month-view",
        reference_month_view_recurring_events,
        "reference-month-view-recurring-events",
        "Recurring events",
        "The daily standup repeats seven times and skips November 5."
    ),
    example!(
        "mobile-month-view",
        reference_mobile_month_view_without_week_days,
        "reference-mobile-month-view-without-week-days",
        "Without week days",
        "The calendar omits weekday headings."
    ),
    example!(
        "mobile-month-view",
        reference_mobile_month_view_with_outside_days,
        "reference-mobile-month-view-with-outside-days",
        "With outside days",
        "Adjacent-month dates fill the first and last weeks."
    ),
    example!(
        "mobile-month-view",
        reference_mobile_month_view_highlight_today,
        "reference-mobile-month-view-highlight-today",
        "Highlight today",
        "November 4 is marked as today."
    ),
    example!(
        "mobile-month-view",
        reference_mobile_month_view_first_day_of_week,
        "reference-mobile-month-view-first-day-of-week",
        "First day of week",
        "Sunday is the first day of the displayed week."
    ),
    example!(
        "mobile-month-view",
        reference_mobile_month_view_static_mode,
        "reference-mobile-month-view-static-mode",
        "Static mode",
        "The schedule can be browsed with editing and selection disabled."
    ),
    example!(
        "mobile-month-view",
        reference_mobile_month_view_localization,
        "reference-mobile-month-view-localization",
        "Localization",
        "Weekdays and calendar date labels use Chinese."
    ),
    example!(
        "year-view",
        reference_year_view_without_header,
        "reference-year-view-without-header",
        "Without header",
        "The schedule grid is shown without navigation controls."
    ),
    example!(
        "year-view",
        reference_year_view_without_week_days,
        "reference-year-view-without-week-days",
        "Without week days",
        "The calendar omits weekday headings."
    ),
    example!(
        "date-picker",
        reference_date_picker_controlled_date,
        "reference-date-picker-controlled-date",
        "Controlled date",
        "The parent advances the displayed calendar after the first range endpoint."
    ),
    example!(
        "date-picker",
        reference_date_picker_single_date_in_range,
        "reference-date-picker-single-date-in-range",
        "Single date in range",
        "Select the same date twice to finish a one-day range."
    ),
    example!(
        "donut-chart",
        reference_donut_chart_tooltip_data_source,
        "reference-donut-chart-tooltip-data-source",
        "Tooltip data source",
        "Compare the hovered segment with the values for every visible segment."
    ),
    example!(
        "funnel-chart",
        reference_funnel_chart_tooltip_data_source,
        "reference-funnel-chart-tooltip-data-source",
        "Tooltip data source",
        "Compare the hovered segment with the values for every visible segment."
    ),
    example!(
        "month-picker",
        reference_month_picker_controlled_date,
        "reference-month-picker-controlled-date",
        "Controlled date",
        "The parent advances the displayed calendar after the first range endpoint."
    ),
    example!(
        "month-picker",
        reference_month_picker_single_date_in_range,
        "reference-month-picker-single-date-in-range",
        "Single date in range",
        "Select the same date twice to finish a one-day range."
    ),
    example!(
        "pie-chart",
        reference_pie_chart_tooltip_data_source,
        "reference-pie-chart-tooltip-data-source",
        "Tooltip data source",
        "Compare the hovered segment with the values for every visible segment."
    ),
    example!(
        "year-picker",
        reference_year_picker_controlled_date,
        "reference-year-picker-controlled-date",
        "Controlled date",
        "The parent advances the displayed calendar after the first range endpoint."
    ),
    example!(
        "year-picker",
        reference_year_picker_single_date_in_range,
        "reference-year-picker-single-date-in-range",
        "Single date in range",
        "Select the same date twice to finish a one-day range."
    ),
    example!(
        "day-view",
        reference_day_view_slot_label_interval,
        "reference-day-view-slot-label-interval",
        "Slot label interval",
        "Display one time label per selectable interval in the time gutter."
    ),
    example!(
        "day-view",
        reference_day_view_slot_label_format,
        "reference-day-view-slot-label-format",
        "Slot label format",
        "Label hourly time slots with a 12-hour clock and keep the same accessible selection intervals."
    ),
    example!(
        "week-view",
        reference_week_view_slot_label_interval,
        "reference-week-view-slot-label-interval",
        "Slot label interval",
        "Display one time label per selectable interval in the time gutter."
    ),
    example!(
        "week-view",
        reference_week_view_slot_label_format,
        "reference-week-view-slot-label-format",
        "Slot label format",
        "Label hourly time slots with a 12-hour clock and keep the same accessible selection intervals."
    ),
    example!(
        "week-view",
        reference_week_view_weekday_format,
        "reference-week-view-weekday-format",
        "Weekday format",
        "Use two-letter weekday labels above the seven timeline columns."
    ),
    example!(
        "day-view",
        reference_day_view_header_format,
        "reference-day-view-header-format",
        "Header format",
        "Show the weekday, month name and day for the displayed date."
    ),
    example!(
        "week-view",
        reference_week_view_week_label_format,
        "reference-week-view-week-label-format",
        "Week label format",
        "Show month, day and year for both ends of the displayed week."
    ),
    example!(
        "agenda-view",
        reference_agenda_view_custom_event_rendering,
        "reference-agenda-view-custom-event-rendering",
        "Custom event rendering",
        "Each event pairs its category with a location and a time range."
    ),
    example!(
        "agenda-view",
        reference_agenda_view_event_click,
        "reference-agenda-view-event-click",
        "Event click",
        "Select an event to open its description, time and location."
    ),
    example!(
        "agenda-view",
        reference_agenda_view_date_formats,
        "reference-agenda-view-date-formats",
        "Date formats",
        "Customize the range and day headings independently."
    ),
    example!(
        "agenda-view",
        reference_agenda_view_all_day_and_multi_day_events,
        "reference-agenda-view-all-day-and-multi-day-events",
        "All-day and multi-day events",
        "Timed appointments, a holiday and a three-day conference share one agenda."
    ),
    example!(
        "agenda-view",
        reference_agenda_view_static_mode,
        "reference-agenda-view-static-mode",
        "Static mode",
        "The schedule can be browsed with editing and selection disabled."
    ),
    example!(
        "agenda-view",
        reference_agenda_view_localization,
        "reference-agenda-view-localization",
        "Localization",
        "Weekdays and calendar date labels use Chinese."
    ),
    example!(
        "carousel",
        carousel_styles,
        "carousel-styles",
        "Styles API",
        "Highlight the viewport, slide, navigation or indicator elements."
    ),
    example!(
        "date-picker",
        reference_date_picker_allow_deselect,
        "reference-date-picker-allow-deselect",
        "Allow deselect",
        "Select the current value again to clear it."
    ),
    example!(
        "date-picker",
        reference_date_picker_multiple_dates,
        "reference-date-picker-multiple-dates",
        "Multiple dates",
        "Select more than one value, then select a value again to remove it."
    ),
    example!(
        "date-picker",
        reference_date_picker_dates_range,
        "reference-date-picker-dates-range",
        "Dates range",
        "Choose a start and an end for the range."
    ),
    example!(
        "date-picker",
        reference_date_picker_default_date,
        "reference-date-picker-default-date",
        "Default date",
        "The initial visible period is in 2024."
    ),
    example!(
        "date-picker",
        reference_date_picker_min_and_max_date,
        "reference-date-picker-min-and-max-date",
        "Min and max date",
        "The picker disables selections outside the configured interval."
    ),
    example!(
        "date-picker",
        reference_date_picker_localization,
        "reference-date-picker-localization",
        "Localization",
        "Month and weekday names use the selected locale."
    ),
    example!(
        "date-picker",
        reference_date_picker_default_level,
        "reference-date-picker-default-level",
        "Default level",
        "The picker opens at the month selection level."
    ),
    example!(
        "date-picker",
        reference_date_picker_max_level,
        "reference-date-picker-max-level",
        "Max level",
        "Header navigation stops at the month selection level."
    ),
    example!(
        "date-picker",
        reference_date_picker_hide_outside_dates,
        "reference-date-picker-hide-outside-dates",
        "Hide outside dates",
        "The calendar leaves adjacent-month cells empty."
    ),
    example!(
        "date-picker",
        reference_date_picker_hide_weekdays,
        "reference-date-picker-hide-weekdays",
        "Hide weekdays",
        "The calendar omits the weekday heading row."
    ),
    example!(
        "date-picker",
        reference_date_picker_display_week_numbers,
        "reference-date-picker-display-week-numbers",
        "Display week numbers",
        "The week number appears beside each calendar row."
    ),
    example!(
        "date-picker",
        reference_date_picker_first_day_of_week,
        "reference-date-picker-first-day-of-week",
        "First day of week",
        "Sunday appears in the first column."
    ),
    example!(
        "date-picker",
        reference_date_picker_exclude_dates,
        "reference-date-picker-exclude-dates",
        "Exclude dates",
        "November 6 and 13 are unavailable."
    ),
    example!(
        "date-picker",
        reference_date_picker_presets,
        "reference-date-picker-presets",
        "Presets",
        "Choose a preset to update the selection."
    ),
    example!(
        "date-picker",
        reference_date_picker_presets_2,
        "reference-date-picker-presets-2",
        "Presets",
        "Choose a preset date one or two weeks ahead."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_multiple_dates,
        "reference-date-picker-input-multiple-dates",
        "Multiple dates",
        "Select more than one value, then select a value again to remove it."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_dates_range,
        "reference-date-picker-input-dates-range",
        "Dates range",
        "Choose a start and an end for the range."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_min_and_max_dates,
        "reference-date-picker-input-min-and-max-dates",
        "Min and max dates",
        "The picker disables selections outside the configured interval."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_default_level,
        "reference-date-picker-input-default-level",
        "Default level",
        "The picker opens at the month selection level."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_max_level,
        "reference-date-picker-input-max-level",
        "Max level",
        "Header navigation stops at the month selection level."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_hide_outside_dates,
        "reference-date-picker-input-hide-outside-dates",
        "Hide outside dates",
        "The calendar leaves adjacent-month cells empty."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_hide_weekdays,
        "reference-date-picker-input-hide-weekdays",
        "Hide weekdays",
        "The calendar omits the weekday heading row."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_with_week_numbers,
        "reference-date-picker-input-with-week-numbers",
        "With week numbers",
        "The week number appears beside each calendar row."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_first_day_of_week,
        "reference-date-picker-input-first-day-of-week",
        "First day of week",
        "Sunday appears in the first column."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_exclude_dates,
        "reference-date-picker-input-exclude-dates",
        "Exclude dates",
        "November 6 and 13 are unavailable."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_presets,
        "reference-date-picker-input-presets",
        "Presets",
        "Choose a preset to update the selection."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_presets_2,
        "reference-date-picker-input-presets-2",
        "Presets",
        "Choose a preset date one or two weeks ahead."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_clearable,
        "reference-date-picker-input-clearable",
        "Clearable",
        "Clear the initial selection using the inset button."
    ),
    example!(
        "date-picker-input",
        reference_date_picker_input_disabled_state,
        "reference-date-picker-input-disabled-state",
        "Disabled state",
        "The field displays its disabled appearance and cannot open the picker."
    ),
    example!(
        "month-picker",
        reference_month_picker_allow_deselect,
        "reference-month-picker-allow-deselect",
        "Allow deselect",
        "Select the current value again to clear it."
    ),
    example!(
        "month-picker",
        reference_month_picker_multiple_dates,
        "reference-month-picker-multiple-dates",
        "Multiple dates",
        "Select more than one value, then select a value again to remove it."
    ),
    example!(
        "month-picker",
        reference_month_picker_dates_range,
        "reference-month-picker-dates-range",
        "Dates range",
        "Choose a start and an end for the range."
    ),
    example!(
        "month-picker",
        reference_month_picker_default_date,
        "reference-month-picker-default-date",
        "Default date",
        "The initial visible period is in 2024."
    ),
    example!(
        "month-picker",
        reference_month_picker_min_and_max_date,
        "reference-month-picker-min-and-max-date",
        "Min and max date",
        "The picker disables selections outside the configured interval."
    ),
    example!(
        "month-picker",
        reference_month_picker_localization,
        "reference-month-picker-localization",
        "Localization",
        "Month and weekday names use the selected locale."
    ),
    example!(
        "month-picker-input",
        reference_month_picker_input_multiple_dates,
        "reference-month-picker-input-multiple-dates",
        "Multiple dates",
        "Select more than one value, then select a value again to remove it."
    ),
    example!(
        "month-picker-input",
        reference_month_picker_input_dates_range,
        "reference-month-picker-input-dates-range",
        "Dates range",
        "Choose a start and an end for the range."
    ),
    example!(
        "month-picker-input",
        reference_month_picker_input_min_and_max_dates,
        "reference-month-picker-input-min-and-max-dates",
        "Min and max dates",
        "The picker disables selections outside the configured interval."
    ),
    example!(
        "month-picker-input",
        reference_month_picker_input_clearable,
        "reference-month-picker-input-clearable",
        "Clearable",
        "Clear the initial selection using the inset button."
    ),
    example!(
        "month-picker-input",
        reference_month_picker_input_disabled_state,
        "reference-month-picker-input-disabled-state",
        "Disabled state",
        "The field displays its disabled appearance and cannot open the picker."
    ),
    example!(
        "month-picker-input",
        reference_month_picker_input_value_format,
        "reference-month-picker-input-value-format",
        "Value format",
        "Display the selected period using localized text."
    ),
    example!(
        "year-picker",
        reference_year_picker_allow_deselect,
        "reference-year-picker-allow-deselect",
        "Allow deselect",
        "Select the current value again to clear it."
    ),
    example!(
        "year-picker",
        reference_year_picker_multiple_dates,
        "reference-year-picker-multiple-dates",
        "Multiple dates",
        "Select more than one value, then select a value again to remove it."
    ),
    example!(
        "year-picker",
        reference_year_picker_dates_range,
        "reference-year-picker-dates-range",
        "Dates range",
        "Choose a start and an end for the range."
    ),
    example!(
        "year-picker",
        reference_year_picker_default_date,
        "reference-year-picker-default-date",
        "Default date",
        "The initial visible period is in 2024."
    ),
    example!(
        "year-picker",
        reference_year_picker_min_and_max_date,
        "reference-year-picker-min-and-max-date",
        "Min and max date",
        "The picker disables selections outside the configured interval."
    ),
    example!(
        "year-picker-input",
        reference_year_picker_input_multiple_dates,
        "reference-year-picker-input-multiple-dates",
        "Multiple dates",
        "Select more than one value, then select a value again to remove it."
    ),
    example!(
        "year-picker-input",
        reference_year_picker_input_dates_range,
        "reference-year-picker-input-dates-range",
        "Dates range",
        "Choose a start and an end for the range."
    ),
    example!(
        "year-picker-input",
        reference_year_picker_input_min_and_max_dates,
        "reference-year-picker-input-min-and-max-dates",
        "Min and max dates",
        "The picker disables selections outside the configured interval."
    ),
    example!(
        "year-picker-input",
        reference_year_picker_input_clearable,
        "reference-year-picker-input-clearable",
        "Clearable",
        "Clear the initial selection using the inset button."
    ),
    example!(
        "year-picker-input",
        reference_year_picker_input_disabled_state,
        "reference-year-picker-input-disabled-state",
        "Disabled state",
        "The field displays its disabled appearance and cannot open the picker."
    ),
    example!(
        "year-picker-input",
        reference_year_picker_input_value_format,
        "reference-year-picker-input-value-format",
        "Value format",
        "Display the selected period using localized text."
    ),
    example!(
        "area-chart",
        area_chart_legend,
        "area-chart-legend",
        "Legend",
        "Toggle individual fruit series in the legend."
    ),
    example!(
        "area-chart",
        area_chart_legend_position,
        "area-chart-legend-position",
        "Legend position",
        "The legend is below the plot."
    ),
    example!(
        "area-chart",
        area_chart_series_labels,
        "area-chart-series-labels",
        "Series labels",
        "The legend and point details use the display labels supplied for the data series."
    ),
    example!(
        "area-chart",
        area_chart_x_and_y_axis_props,
        "area-chart-x-and-y-axis-props",
        "X and Y axis props",
        "The Y axis spans 0 to 5,000 and both axes show their tick marks."
    ),
    example!(
        "area-chart",
        area_chart_points_labels,
        "area-chart-points-labels",
        "Points labels",
        "Values appear beside the corresponding data points."
    ),
    example!(
        "area-chart",
        area_chart_remove_tooltip,
        "area-chart-remove-tooltip",
        "Remove tooltip",
        "Hover over a data point with tooltip display disabled."
    ),
    example!(
        "area-chart",
        area_chart_customize_dots,
        "area-chart-customize-dots",
        "Customize dots",
        "The chart draws 14-pixel markers at each observation."
    ),
    example!(
        "area-chart",
        area_chart_dashed_area_line,
        "area-chart-dashed-area-line",
        "Dashed area line",
        "Series outlines use a dashed stroke."
    ),
    example!(
        "area-chart",
        area_chart_reference_lines,
        "area-chart-reference-lines",
        "Reference lines",
        "Reference lines mark a day and a sales target."
    ),
    example!(
        "area-chart",
        area_chart_reference_areas,
        "area-chart-reference-areas",
        "Reference areas",
        "The highlighted interval covers March 23 through March 25."
    ),
    example!(
        "area-chart",
        area_chart_reference_dots,
        "area-chart-reference-dots",
        "Reference dots",
        "The marker identifies the March 25 peak."
    ),
    example!(
        "area-chart",
        area_chart_stacked_area_chart,
        "area-chart-stacked-area-chart",
        "Stacked area chart",
        "Positive values accumulate for each day."
    ),
    example!(
        "area-chart",
        area_chart_percent_area_chart,
        "area-chart-percent-area-chart",
        "Percent area chart",
        "Each day is normalized to 100 percent before its series are stacked."
    ),
    example!(
        "area-chart",
        area_chart_brush,
        "area-chart-brush",
        "Brush",
        "Move the ends of the range to change the visible dates."
    ),
    example!(
        "area-chart",
        area_chart_sync_multiple_areacharts,
        "area-chart-sync-multiple-areacharts",
        "Sync multiple AreaCharts",
        "Hover or select a point to highlight the same category and series in both charts."
    ),
    example!(
        "line-chart",
        line_chart_legend,
        "line-chart-legend",
        "Legend",
        "Toggle individual fruit series in the legend."
    ),
    example!(
        "line-chart",
        line_chart_legend_position,
        "line-chart-legend-position",
        "Legend position",
        "The legend is below the plot."
    ),
    example!(
        "line-chart",
        line_chart_series_labels,
        "line-chart-series-labels",
        "Series labels",
        "The legend and point details use the display labels supplied for the data series."
    ),
    example!(
        "line-chart",
        line_chart_x_and_y_axis_props,
        "line-chart-x-and-y-axis-props",
        "X and Y axis props",
        "The Y axis spans 0 to 5,000 and both axes show their tick marks."
    ),
    example!(
        "line-chart",
        line_chart_points_labels,
        "line-chart-points-labels",
        "Points labels",
        "Values appear beside the corresponding data points."
    ),
    example!(
        "line-chart",
        line_chart_remove_tooltip,
        "line-chart-remove-tooltip",
        "Remove tooltip",
        "Hover over a data point with tooltip display disabled."
    ),
    example!(
        "line-chart",
        line_chart_customize_dots,
        "line-chart-customize-dots",
        "Customize dots",
        "The chart draws 14-pixel markers at each observation."
    ),
    example!(
        "line-chart",
        line_chart_dashed_line,
        "line-chart-dashed-line",
        "Dashed line",
        "Series outlines use a dashed stroke."
    ),
    example!(
        "line-chart",
        line_chart_reference_lines,
        "line-chart-reference-lines",
        "Reference lines",
        "Reference lines mark a day and a sales target."
    ),
    example!(
        "line-chart",
        line_chart_reference_areas,
        "line-chart-reference-areas",
        "Reference areas",
        "The highlighted interval covers March 23 through March 25."
    ),
    example!(
        "line-chart",
        line_chart_reference_dots,
        "line-chart-reference-dots",
        "Reference dots",
        "The marker identifies the March 25 peak."
    ),
    example!(
        "line-chart",
        line_chart_brush,
        "line-chart-brush",
        "Brush",
        "Move the ends of the range to change the visible dates."
    ),
    example!(
        "line-chart",
        line_chart_sync_multiple_linecharts,
        "line-chart-sync-multiple-linecharts",
        "Sync multiple LineCharts",
        "Hover or select a point to highlight the same category and series in both charts."
    ),
    example!(
        "bar-chart",
        bar_chart_legend,
        "bar-chart-legend",
        "Legend",
        "Toggle individual fruit series in the legend."
    ),
    example!(
        "bar-chart",
        bar_chart_legend_position,
        "bar-chart-legend-position",
        "Legend position",
        "The legend is below the plot."
    ),
    example!(
        "bar-chart",
        bar_chart_series_labels,
        "bar-chart-series-labels",
        "Series labels",
        "The legend and point details use the display labels supplied for the data series."
    ),
    example!(
        "bar-chart",
        bar_chart_x_and_y_axis_props,
        "bar-chart-x-and-y-axis-props",
        "X and Y axis props",
        "The Y axis spans 0 to 5,000 and both axes show their tick marks."
    ),
    example!(
        "bar-chart",
        bar_chart_bar_value_label,
        "bar-chart-bar-value-label",
        "Bar value label",
        "Each bar displays its numeric value."
    ),
    example!(
        "bar-chart",
        bar_chart_remove_tooltip,
        "bar-chart-remove-tooltip",
        "Remove tooltip",
        "Hover over a data point with tooltip display disabled."
    ),
    example!(
        "bar-chart",
        bar_chart_reference_lines,
        "bar-chart-reference-lines",
        "Reference lines",
        "Reference lines mark a day and a sales target."
    ),
    example!(
        "bar-chart",
        bar_chart_reference_areas,
        "bar-chart-reference-areas",
        "Reference areas",
        "The highlighted interval covers March 23 through March 25."
    ),
    example!(
        "bar-chart",
        bar_chart_reference_dots,
        "bar-chart-reference-dots",
        "Reference dots",
        "The marker identifies the March 25 peak."
    ),
    example!(
        "bar-chart",
        bar_chart_stacked_bar_chart,
        "bar-chart-stacked-bar-chart",
        "Stacked bar chart",
        "Positive values accumulate for each day."
    ),
    example!(
        "bar-chart",
        bar_chart_percent_bar_chart,
        "bar-chart-percent-bar-chart",
        "Percent bar chart",
        "Each day is normalized to 100 percent before its series are stacked."
    ),
    example!(
        "bar-chart",
        bar_chart_brush,
        "bar-chart-brush",
        "Brush",
        "Move the ends of the range to change the visible dates."
    ),
    example!(
        "bar-chart",
        bar_chart_sync_multiple_barcharts,
        "bar-chart-sync-multiple-barcharts",
        "Sync multiple BarCharts",
        "Hover or select a point to highlight the same category and series in both charts."
    ),
    example!(
        "composite-chart",
        composite_chart_legend,
        "composite-chart-legend",
        "Legend",
        "Toggle individual fruit series in the legend."
    ),
    example!(
        "composite-chart",
        composite_chart_legend_position,
        "composite-chart-legend-position",
        "Legend position",
        "The legend is below the plot."
    ),
    example!(
        "composite-chart",
        composite_chart_series_labels,
        "composite-chart-series-labels",
        "Series labels",
        "The legend and point details use the display labels supplied for the data series."
    ),
    example!(
        "composite-chart",
        composite_chart_x_and_y_axis_props,
        "composite-chart-x-and-y-axis-props",
        "X and Y axis props",
        "The Y axis spans 0 to 5,000 and both axes show their tick marks."
    ),
    example!(
        "composite-chart",
        composite_chart_points_labels,
        "composite-chart-points-labels",
        "Points labels",
        "Values appear beside the corresponding data points."
    ),
    example!(
        "composite-chart",
        composite_chart_remove_tooltip,
        "composite-chart-remove-tooltip",
        "Remove tooltip",
        "Hover over a data point with tooltip display disabled."
    ),
    example!(
        "composite-chart",
        composite_chart_customize_dots,
        "composite-chart-customize-dots",
        "Customize dots",
        "The chart draws 14-pixel markers at each observation."
    ),
    example!(
        "composite-chart",
        composite_chart_dashed_lines,
        "composite-chart-dashed-lines",
        "Dashed lines",
        "Series outlines use a dashed stroke."
    ),
    example!(
        "composite-chart",
        composite_chart_reference_lines,
        "composite-chart-reference-lines",
        "Reference lines",
        "Reference lines mark a day and a sales target."
    ),
    example!(
        "composite-chart",
        composite_chart_reference_areas,
        "composite-chart-reference-areas",
        "Reference areas",
        "The highlighted interval covers March 23 through March 25."
    ),
    example!(
        "composite-chart",
        composite_chart_reference_dots,
        "composite-chart-reference-dots",
        "Reference dots",
        "The marker identifies the March 25 peak."
    ),
    example!(
        "composite-chart",
        composite_chart_brush,
        "composite-chart-brush",
        "Brush",
        "Move the ends of the range to change the visible dates."
    ),
    example!(
        "composite-chart",
        composite_chart_sync_multiple_charts,
        "composite-chart-sync-multiple-charts",
        "Sync multiple charts",
        "Hover or select a point to highlight the same category and series in both charts."
    ),
    example!(
        "button",
        button_group_orientation,
        "button-group-orientation",
        "Button.Group",
        "Switch between a horizontal and vertical group."
    ),
    example!(
        "button",
        button_sections_position,
        "button-sections-position",
        "Sections position",
        "Choose how the icon, label and trailing section share the available space."
    ),
    example!(
        "button",
        button_compact,
        "button-compact",
        "Compact size",
        "Change the size of a compact button."
    ),
    example!(
        "button",
        button_gradient,
        "button-gradient",
        "Gradient variant",
        "Choose the gradient colors and angle."
    ),
    example!(
        "action-icon",
        action_icon_group_orientation,
        "action-icon-group-orientation",
        "ActionIcon.Group",
        "Change the direction of the icon actions."
    ),
    example!(
        "action-icon",
        action_icon_gradient,
        "action-icon-gradient",
        "Gradient variant",
        "Choose the gradient colors and angle."
    ),
    example!(
        "tabs",
        tabs_vertical_placement,
        "tabs-vertical-placement",
        "Vertical tabs placement",
        "Move the tab list to either side of the content."
    ),
    example!(
        "tabs",
        tabs_position,
        "tabs-position",
        "Tabs position",
        "Adjust how the tabs share their row."
    ),
    example!(
        "grid",
        grid_justify_align,
        "grid-justify-align",
        "Justify and align",
        "Adjust the horizontal and vertical placement of the columns."
    ),
    example!(
        "grid",
        grid_grow,
        "grid-grow",
        "Grow",
        "Let the last row expand to fill the available width."
    ),
    example!(
        "splitter",
        splitter_line_size,
        "splitter-line-size",
        "Line size",
        "Change the thickness of the divider."
    ),
    example!(
        "burger",
        burger_line_size,
        "burger-line-size",
        "Change line size",
        "Adjust the width of the strokes in the menu icon."
    ),
    example!(
        "nav-link",
        nav_link_active,
        "nav-link-active",
        "Active",
        "Change the color and appearance of the active link."
    ),
    example!(
        "popover",
        popover_disabled,
        "popover-disabled",
        "Disabled",
        "Disable the trigger and try opening the popover."
    ),
    example!(
        "tooltip",
        tooltip_color,
        "tooltip-color",
        "Color",
        "Choose a color and hover or focus the trigger."
    ),
    example!(
        "tour",
        tour_overlay_config,
        "tour-overlay-config",
        "Overlay configuration",
        "Configure the overlay and dismissal behavior, then start the tour."
    ),
    example!(
        "menu",
        menu_dropdown_position,
        "menu-dropdown-position",
        "Dropdown position",
        "Open the menu to see its placement, offset and arrow."
    ),
    example!(
        "popover",
        popover_axes,
        "popover-axes",
        "Offset",
        "Adjust the offset along each axis."
    ),
    example!(
        "popover",
        popover_offset,
        "popover-offset",
        "Offset",
        "Choose the placement and adjust the distance from the trigger."
    ),
    example!(
        "popover",
        popover_arrow,
        "popover-arrow",
        "Dropdown arrow",
        "Configure the arrow, then open the preview."
    ),
    example!(
        "tooltip",
        tooltip_axes,
        "tooltip-axes",
        "Offset",
        "Adjust the offset along each axis."
    ),
    example!(
        "tooltip",
        tooltip_offset,
        "tooltip-offset",
        "Offset",
        "Choose the placement and adjust the distance from the trigger."
    ),
    example!(
        "tooltip",
        tooltip_arrow,
        "tooltip-arrow",
        "Arrow",
        "Configure the arrow, then open the preview."
    ),
    example!(
        "overlay",
        overlay_blur,
        "overlay-blur",
        "Blur",
        "Change the blur strength of the overlay."
    ),
    example!(
        "stepper",
        stepper_appearance,
        "stepper-appearance",
        "Color, radius and size",
        "Change the color, radius and size of the steps."
    ),
    example!(
        "stepper",
        stepper_icon_size,
        "stepper-icon-size",
        "Color, radius and size",
        "Change the step indicator diameter."
    ),
    example!(
        "space",
        space_width,
        "space-width",
        "Usage",
        "Set the horizontal space between two labels."
    ),
    example!(
        "menu",
        menu_aligned_labels,
        "menu-aligned-labels",
        "Aligning labels of items with and without indicators",
        "Align ordinary menu actions with items that have check indicators."
    ),
    example!(
        "radar-chart",
        radar_chart_parts,
        "radar-chart-parts",
        "Hide/show chart parts",
        "Show or hide the axes, grid, tooltip and data points."
    ),
    example!(
        "carousel",
        carousel_embla_options,
        "carousel-embla-options",
        "Embla options",
        "Change how slides and controls are laid out, then navigate through the slides."
    ),
    example!(
        "carousel",
        carousel_options,
        "carousel-options",
        "Options",
        "Change how slides and controls are laid out, then navigate through the slides."
    ),
    Example {
        slug: "notifications-system",
        id: "notification-props",
        link: "#notification-props",
        title: "Notification props",
        note: "The notification system uses the same configurable notification widget.",
        source: || include_str!("../showcase_demos/notification_usage.rs"),
        build: super::showcase_demos::notification_usage_widget,
    },
    example!(
        "donut-chart",
        donut_chart_segments_labels,
        "donut-chart-segments-labels",
        "Segments labels",
        "Change how values are labeled in the drawing."
    ),
    example!(
        "donut-chart",
        donut_chart_size_and_thickness,
        "donut-chart-size-and-thickness",
        "Size and thickness",
        "Adjust the size of the drawing."
    ),
    example!(
        "donut-chart",
        donut_chart_padding_angle,
        "donut-chart-padding-angle",
        "Padding angle",
        "Set the angle between neighboring segments."
    ),
    example!(
        "donut-chart",
        donut_chart_segment_color,
        "donut-chart-segment-color",
        "Segment color",
        "Choose the color used to draw the data."
    ),
    example!(
        "donut-chart",
        donut_chart_segments_stroke,
        "donut-chart-segments-stroke",
        "Segments stroke",
        "Set the width of the segment outline."
    ),
    example!(
        "pie-chart",
        pie_chart_segments_labels,
        "pie-chart-segments-labels",
        "Segments labels",
        "Change how values are labeled in the drawing."
    ),
    example!(
        "pie-chart",
        pie_chart_size,
        "pie-chart-size",
        "Size",
        "Change the chart diameter."
    ),
    example!(
        "pie-chart",
        pie_chart_segment_color,
        "pie-chart-segment-color",
        "Segment color",
        "Choose the color used to draw the data."
    ),
    example!(
        "pie-chart",
        pie_chart_segments_stroke,
        "pie-chart-segments-stroke",
        "Segments stroke",
        "Set the width of the segment outline."
    ),
    example!(
        "funnel-chart",
        funnel_chart_segments_labels,
        "funnel-chart-segments-labels",
        "Segments labels",
        "Change how values are labeled in the drawing."
    ),
    example!(
        "funnel-chart",
        funnel_chart_size_and_thickness,
        "funnel-chart-size-and-thickness",
        "Size and thickness",
        "Adjust the size of the drawing."
    ),
    example!(
        "funnel-chart",
        funnel_chart_segment_color,
        "funnel-chart-segment-color",
        "Segment color",
        "Choose the color used to draw the data."
    ),
    example!(
        "funnel-chart",
        funnel_chart_segments_stroke,
        "funnel-chart-segments-stroke",
        "Segments stroke",
        "Set the width of the segment outline."
    ),
    example!(
        "radial-bar-chart",
        radial_bar_chart_change_color,
        "radial-bar-chart-change-color",
        "Change color",
        "Choose the color used to draw the data."
    ),
    example!(
        "bubble-chart",
        bubble_chart_change_color,
        "bubble-chart-change-color",
        "Change color",
        "Choose the color used to draw the data."
    ),
    example!(
        "sunburst-chart",
        sunburst_chart_gap,
        "sunburst-chart-gap",
        "Gap",
        "Adjust the separation between concentric segments."
    ),
    example!(
        "waffle-chart",
        waffle_chart_gap_and_grid_size,
        "waffle-chart-gap-and-grid-size",
        "Gap and grid size",
        "Adjust the number of rows and columns and the space between cells."
    ),
    example!(
        "waffle-chart",
        waffle_chart_fill_direction,
        "waffle-chart-fill-direction",
        "Fill direction",
        "Change the order in which categories fill the grid."
    ),
    example!(
        "waffle-chart",
        waffle_chart_legend,
        "waffle-chart-legend",
        "Legend",
        "Place the legend around the chart."
    ),
    example!(
        "radar-chart",
        radar_chart_change_color,
        "radar-chart-change-color",
        "Change color",
        "Choose the color used to draw the data."
    ),
    example!(
        "heatmap",
        heatmap_rect_size_gap_and_radius,
        "heatmap-rect-size-gap-and-radius",
        "Rect size, gap and radius",
        "Change the geometry of each dated cell."
    ),
    example!(
        "checkbox",
        checkbox_group_config,
        "checkbox-group-config",
        "Checkbox.Group",
        "Edit the label, description and error for the group."
    ),
    example!(
        "radio",
        radio_group_config,
        "radio-group-config",
        "Radio.Group component",
        "Edit the label, description and error for the group."
    ),
    example!(
        "switch",
        switch_group_config,
        "switch-group-config",
        "Switch.Group",
        "Edit the label, description and error for the group."
    ),
    example!(
        "select",
        select_check_icon,
        "select-check-icon",
        "Checked option icon",
        "Open the list to see the checked options, alignment and icon position."
    ),
    example!(
        "multi-select",
        multi_select_check_icon,
        "multi-select-check-icon",
        "Checked option icon",
        "Open the list to see the checked options, alignment and icon position."
    ),
    example!(
        "combobox-popover",
        combobox_popover_check_icon,
        "combobox-popover-check-icon",
        "Check icon",
        "Open the list to see the checked options, alignment and icon position."
    ),
    example!(
        "color-picker",
        color_picker_format,
        "color-picker-format",
        "Color format",
        "Choose the output notation and change the color."
    ),
    example!(
        "color-picker",
        color_picker_size,
        "color-picker-size",
        "Size",
        "Change the size of the picker."
    ),
    example!(
        "slider",
        slider_thumb_size,
        "slider-thumb-size",
        "Thumb size",
        "Change the diameter of the handle."
    ),
    example!(
        "segmented-control",
        segmented_control_color,
        "segmented-control-color",
        "Color",
        "Choose the color of the active segment."
    ),
    example!(
        "input",
        input_wrapper,
        "input-wrapper",
        "Input.Wrapper component",
        "Edit the field text and wrapper size."
    ),
    example!(
        "input",
        input_clear_button,
        "input-clear-button",
        "Input.ClearButton component",
        "Edit the input and use the clear action inside its right section."
    ),
    example!(
        "date-picker",
        date_picker_size,
        "date-picker-size",
        "Size",
        "Change the calendar size. Your selection is retained across sizes."
    ),
    example!(
        "month-picker",
        month_picker_size,
        "month-picker-size",
        "Size",
        "Change the calendar size. Your selection is retained across sizes."
    ),
    example!(
        "year-picker",
        year_picker_size,
        "year-picker-size",
        "Size",
        "Change the calendar size. Your selection is retained across sizes."
    ),
    example!(
        "area-chart",
        area_chart_connect_nulls,
        "area-chart-connect-nulls",
        "Connect nulls",
        "A missing observation in the first series can interrupt the line or connect its neighbors."
    ),
    example!(
        "area-chart",
        area_chart_area_color,
        "area-chart-area-color",
        "Area color",
        "Choose the color used for the first series."
    ),
    example!(
        "area-chart",
        area_chart_stroke_width,
        "area-chart-stroke-width",
        "Stroke width",
        "Adjust the width of the series stroke."
    ),
    example!(
        "area-chart",
        area_chart_fill_opacity,
        "area-chart-fill-opacity",
        "Fill opacity",
        "Adjust the transparency of the filled areas."
    ),
    example!(
        "line-chart",
        line_chart_connect_nulls,
        "line-chart-connect-nulls",
        "Connect nulls",
        "A missing observation in the first series can interrupt the line or connect its neighbors."
    ),
    example!(
        "line-chart",
        line_chart_line_color,
        "line-chart-line-color",
        "Line color",
        "Choose the color used for the first series."
    ),
    example!(
        "line-chart",
        line_chart_stroke_width,
        "line-chart-stroke-width",
        "Stroke width",
        "Adjust the width of the series stroke."
    ),
    example!(
        "bar-chart",
        bar_chart_area_color,
        "bar-chart-area-color",
        "Area color",
        "Choose the color used for the first series."
    ),
    example!(
        "composite-chart",
        composite_chart_chart_color,
        "composite-chart-chart-color",
        "Chart color",
        "Choose the color used for the first series."
    ),
    example!(
        "button",
        button_styles,
        "button-styles",
        "Styles API",
        "Inspect the button parts. Press the button to show its loader."
    ),
    example!(
        "menu",
        menu_styles,
        "menu-styles",
        "Styles API",
        "Open the menu to inspect its items, search field and submenu."
    ),
    example!(
        "menubar",
        menubar_styles,
        "menubar-styles",
        "Styles API",
        "Inspect the menu bar and its File and Edit triggers."
    ),
    example!(
        "pagination",
        pagination_styles,
        "pagination-styles",
        "Styles API",
        "Inspect the page controls and the compact page label."
    ),
    example!(
        "stepper",
        stepper_styles,
        "stepper-styles",
        "Styles API",
        "Inspect the step controls and select a step to change its content."
    ),
    example!(
        "tabs",
        tabs_styles,
        "tabs-styles",
        "Styles API",
        "Inspect the tab list, icons, labels and active panel."
    ),
    example!(
        "composite-chart",
        composite_chart_stroke_width,
        "composite-chart-stroke-width",
        "Stroke width",
        "Adjust the width of the series stroke."
    ),
];
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    EXAMPLES
        .iter()
        .filter(|item| item.slug == slug)
        .map(|item| DetailSection {
            id: item.id,
            title: item.title,
            note: item.note,
            source: (item.source)(),
            example: (item.build)(theme.clone()),
        })
        .collect()
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    EXAMPLES
        .iter()
        .filter(|item| item.slug == slug)
        .map(|item| (item.title, item.link))
        .collect()
}
