mod accordion_composed;
mod accordion_custom_styles;
mod accordion_icons;
mod accordion_labels;
mod accordion_no_collapse;
mod accordion_plus;
mod accordion_style_selectors;
mod accordion_transition_none;
mod accordion_transition_slow;
mod accordion_unstyled;
mod timeline_alternate;
mod timeline_opposite;
mod timeline_props_play;
use super::showcase_examples::DetailSection;
mod accordion_content;
mod accordion_controlled;
mod accordion_dynamic;
mod accordion_variants;
mod tabs_content;
mod tabs_controlled;
mod tabs_dynamic;
mod tabs_orientation;
mod tabs_variants;
mod timeline_markers;
mod timeline_progress;
mod timeline_semantics;

pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    macro_rules! section {
        ($module:ident, $id:literal, $title:literal, $note:literal) => {
            DetailSection {
                id: $id,
                title: $title,
                note: $note,
                example: Box::new($module::demo(theme.clone())),
                source: include_str!(concat!(stringify!($module), ".rs")),
            }
        };
    }
    match slug {
        "accordion" => vec![
            DetailSection {
                id: "accordion-custom-styles",
                title: "Custom accordion styles",
                note: "The expanded fruit lifts above the surrounding controls.",
                example: Box::new(accordion_custom_styles::demo(theme.clone())),
                source: include_str!("accordion_custom_styles.rs"),
            },
            DetailSection {
                id: "accordion-plus",
                title: "Change chevron",
                note: "Expand an item to rotate its plus icon.",
                example: Box::new(accordion_plus::demo(theme.clone())),
                source: include_str!("accordion_plus.rs"),
            },
            DetailSection {
                id: "accordion-style-selectors",
                title: "Component Styles API",
                note: "Hover or focus a selector to outline the corresponding component element.",
                example: Box::new(accordion_style_selectors::demo(theme.clone())),
                source: include_str!("accordion_style_selectors.rs"),
            },
            DetailSection {
                id: "accordion-transition-none",
                title: "Without transition",
                note: "Open and close a panel with a transition duration of 0 milliseconds.",
                example: Box::new(accordion_transition_none::demo(theme.clone())),
                source: include_str!("accordion_transition_none.rs"),
            },
            DetailSection {
                id: "accordion-transition-slow",
                title: "Slow transition",
                note: "Open and close a panel with a transition duration of 500 milliseconds.",
                example: Box::new(accordion_transition_slow::demo(theme.clone())),
                source: include_str!("accordion_transition_slow.rs"),
            },
            DetailSection {
                id: "accordion-labels",
                title: "Custom control label",
                note: "Each control contains an avatar, a title and a description.",
                example: Box::new(accordion_labels::demo(theme.clone())),
                source: include_str!("accordion_labels.rs"),
            },
            DetailSection {
                id: "accordion-icons",
                title: "With icons",
                note: "Each disclosure keeps its own leading icon and direction indicator.",
                example: Box::new(accordion_icons::demo(theme.clone())),
                source: include_str!("accordion_icons.rs"),
            },
            DetailSection {
                id: "accordion-composed",
                title: "Compose controls",
                note: "The adjacent action runs independently of disclosure activation.",
                example: Box::new(accordion_composed::demo(theme.clone())),
                source: include_str!("accordion_composed.rs"),
            },
            DetailSection {
                id: "accordion-no-collapse",
                title: "Disable collapse",
                note: "Select another question to switch panels while keeping one answer open.",
                example: Box::new(accordion_no_collapse::demo(theme.clone())),
                source: include_str!("accordion_no_collapse.rs"),
            },
            DetailSection {
                id: "accordion-unstyled",
                title: "Unstyled Accordion",
                note: "Expand each answer using the unstyled control.",
                example: Box::new(accordion_unstyled::demo(theme.clone())),
                source: include_str!("accordion_unstyled.rs"),
            },
            section!(
                accordion_variants,
                "accordion-variants",
                "Variants and disclosure icons",
                "Default uses full-width headings and separators; Contained, Filled and Separated configure item surfaces. with_chevron(false) omits the passive disclosure graphic. A custom trigger_content factory preserves the native button and its label. Both static and keyed item APIs use the same presentation."
            ),
            section!(
                accordion_controlled,
                "accordion-controlled",
                "Disabled items",
                "Bananas is disabled. Apples and Broccoli remain available."
            ),
            section!(
                accordion_content,
                "accordion-content",
                "Retain or retire panel content",
                "ContentPolicy::KeepMounted hides a panel while preserving its editor and Owner. Unmount, the default, retires child state when closed. Type into both drafts, close and reopen the sections to compare. Closing content with actual focus restores the heading trigger."
            ),
            section!(
                accordion_dynamic,
                "accordion-dynamic",
                "Keyed sections",
                "items(source, factory) reconciles stable keys with at most 256 live items. Factories run for new keys; retained keys keep state and physical identity. This example adds an account section without rebuilding the existing questions. heading_level and regions configure semantic headings and optional panel regions."
            ),
        ],
        "tabs" => vec![
            section!(
                tabs_variants,
                "tabs-variants",
                "Default, outline and pills",
                "TabsVariant configures the shared static/keyed skin. Default has a baseline and selected indicator; Outline uses bordered triggers; Pills uses a filled selection. grow(true) distributes horizontal triggers across the available width. Each trigger remains a real tab with selected state and roving focus."
            ),
            section!(
                tabs_orientation,
                "tabs-orientation",
                "Vertical and RTL navigation",
                "orientation changes both physical layout and keyboard navigation. Vertical tabs place panels beside the tab list and use Up/Down. direction(Rtl) reverses horizontal arrows. The link between each tab and its panel stays stable."
            ),
            section!(
                tabs_controlled,
                "tabs-controlled",
                "Controlled value and manual activation",
                "value/on_value_change proposes selection without writing to a controlled source. In Manual activation, arrows move focus and Enter/Space select. Automatic activation selects on focus. Home/End and loop_focus use enabled items; disabled triggers cannot activate. Program changes are silent."
            ),
            section!(
                tabs_content,
                "tabs-content",
                "Keep mounted and unmount",
                "KeepMounted is the default and preserves panel drafts. Unmount retires each inactive panel's child Owner. Select another tab and return to compare the two editors. Focus is restored to the selected available trigger when a panel containing actual focus is hidden."
            ),
            section!(
                tabs_dynamic,
                "tabs-dynamic",
                "Dynamic keyed tabs",
                "tabs(source, factory) reconciles at most 256 current keys. Reordering keeps retained tab and editor identities; removing the active local item selects a surviving enabled item. Controlled state follows the explicit missing-key contract. No visited-tab cache is retained."
            ),
        ],
        "timeline" => vec![
            section!(
                timeline_props_play,
                "timeline-props-play",
                "Line and bullet props",
                "Adjust the event markers, connector and active position."
            ),
            DetailSection {
                id: "timeline-opposite",
                title: "Opposite content",
                note: "Timestamps occupy the column opposite the event description.",
                example: Box::new(timeline_opposite::demo(theme.clone())),
                source: include_str!("timeline_opposite.rs"),
            },
            DetailSection {
                id: "timeline-alternate",
                title: "Alternate items",
                note: "Timeline items alternate across the central line.",
                example: Box::new(timeline_alternate::demo(theme.clone())),
                source: include_str!("timeline_alternate.rs"),
            },
            section!(
                timeline_progress,
                "timeline-progress",
                "Reactive progress",
                "progress accepts Pending, Active(key) or Complete. Earlier items and their connecting lines become completed, while the active item is current. External updates preserve the list and its child state. Unknown active keys report a configuration error while retaining the accepted presentation."
            ),
            section!(
                timeline_markers,
                "timeline-markers",
                "Custom markers, bullet size and line width",
                "marker mounts passive or ordinary Widget content in a centered node. bullet_size accepts 8–64 CSS pixels, line_width accepts 0–8, and both reject non-finite values before mounting. Default nodes and connectors require no custom stylesheet; the final item has no trailing connector."
            ),
            section!(
                timeline_semantics,
                "timeline-semantics",
                "Headings, descriptions and state labels",
                "The timeline retains ordered-list and list-item semantics while drawing its own nodes. heading_level sets title semantics, description and timestamp add supporting content, and state_labels localizes completed/current/upcoming text. Displayed progress never implies an interactive selection."
            ),
        ],
        _ => Vec::new(),
    }
}

pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "accordion" => vec![
            ("Custom accordion styles", "#accordion-custom-styles"),
            ("Change chevron", "#accordion-plus"),
            ("Component Styles API", "#accordion-style-selectors"),
            ("Without transition", "#accordion-transition-none"),
            ("Slow transition", "#accordion-transition-slow"),
            ("Custom control label", "#accordion-labels"),
            ("With icons", "#accordion-icons"),
            ("Compose controls", "#accordion-composed"),
            ("Disable collapse", "#accordion-no-collapse"),
            ("Unstyled Accordion", "#accordion-unstyled"),
            ("Variants", "#accordion-variants"),
            ("Controlled state", "#accordion-controlled"),
            ("Content lifetime", "#accordion-content"),
            ("Keyed sections", "#accordion-dynamic"),
        ],
        "tabs" => vec![
            ("Variants", "#tabs-variants"),
            ("Orientation", "#tabs-orientation"),
            ("Controlled selection", "#tabs-controlled"),
            ("Content lifetime", "#tabs-content"),
            ("Keyed tabs", "#tabs-dynamic"),
        ],
        "timeline" => vec![
            ("Line and bullet props", "#timeline-props-play"),
            ("Opposite content", "#timeline-opposite"),
            ("Alternate items", "#timeline-alternate"),
            ("Progress", "#timeline-progress"),
            ("Markers", "#timeline-markers"),
            ("Semantics", "#timeline-semantics"),
        ],
        _ => Vec::new(),
    }
}
