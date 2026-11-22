mod box_composition;
mod box_reactive;
mod box_style;
mod collapse_controlled;
mod collapse_horizontal;
mod collapse_intrinsic;
mod collapse_lifetime;
mod collapse_motion;
mod collapse_nested;
mod divider_labels;
mod divider_long;
mod divider_patterns;
mod divider_sizes;
mod divider_vertical;
mod focus_initial;
mod focus_nested;
mod focus_policies;
mod hidden_description;
mod hidden_focus;
mod hidden_name;
mod marquee_direction;
pub(crate) mod marquee_logos;
mod marquee_fade {
    use super::marquee_logos::logo_strip;
    include!("marquee_fade.rs");
}
mod marquee_hover {
    use super::marquee_logos::logo_strip;
    include!("marquee_hover.rs");
}
mod marquee_rows {
    use super::marquee_logos::logo_strip;
    include!("marquee_rows.rs");
}
mod marquee_speed;
mod marquee_static;
mod marquee_style_play;
mod marquee_vertical {
    use super::marquee_logos::logo_strip;
    include!("marquee_vertical.rs");
}
mod paper_card;
mod paper_spacing;
mod paper_surfaces;
mod portal_availability;
mod portal_destinations;
mod portal_enabled;
mod scroll_autosize;
mod scroll_axes;
mod scroll_boundaries;
mod scroll_disable_horizontal;
mod scroll_horizontal;
mod scroll_into_view;
mod scroll_observed;
mod scroll_offsets;
mod scroll_policies;
mod scroll_popover;
mod scroll_position;
mod scroll_side_play;
mod scroll_start;
mod toolbar_auto_width {
    use crate::showcase_demos::{ToolbarExample, format_toolbar};
    include!("toolbar_auto_width.rs");
}
mod toolbar_contrast {
    use crate::showcase_demos::{ToolbarExample, format_toolbar};
    include!("toolbar_contrast.rs");
}
mod toolbar_direction;
mod toolbar_disabled {
    use crate::showcase_demos::{ToolbarExample, format_toolbar};
    include!("toolbar_disabled.rs");
}
mod toolbar_groups {
    use crate::showcase_demos::{ToolbarExample, format_toolbar};
    include!("toolbar_groups.rs");
}
mod toolbar_keyboard;
mod toolbar_overflow;
mod toolbar_style_play;
use super::showcase_examples::DetailSection;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "box" => vec![
            DetailSection {
                id: "box-style",
                title: "Explicit independent style declarations",
                note: "Box is a real layout widget with explicit style and children. Padding, radius, border and background are independent declarations; it is not a React component-prop adapter.",
                example: Box::new(box_style::demo(theme.clone())),
                source: include_str!("box_style.rs"),
            },
            DetailSection {
                id: "box-composition",
                title: "Composition with real actions",
                note: "Use a real Button for activation and an ordinary Box for layout. A class or color does not change the semantic kind into a button or a link.",
                example: Box::new(box_composition::demo(theme.clone())),
                source: include_str!("box_composition.rs"),
            },
            DetailSection {
                id: "box-reactive",
                title: "Reactive appearance and the HTML layer",
                note: "An explicit reactive Style changes the same box. Box belongs to the widget layer; raw HTML/CSS templates and intrinsic elements remain a separate display layer with their own contract.",
                example: Box::new(box_reactive::demo(theme.clone())),
                source: include_str!("box_reactive.rs"),
            },
        ],
        "collapse" => vec![
            DetailSection {
                id: "collapse-horizontal",
                title: "Horizontal orientation",
                note: "Expand the blue panel along its horizontal axis.",
                example: Box::new(collapse_horizontal::demo(theme.clone())),
                source: include_str!("collapse_horizontal.rs"),
            },
            DetailSection {
                id: "collapse-nested",
                title: "Example: nested Collapse components",
                note: "Each disclosure keeps independent open state.",
                example: Box::new(collapse_nested::demo(theme.clone())),
                source: include_str!("collapse_nested.rs"),
            },
            DetailSection {
                id: "collapse-controlled",
                title: "Controlled opening and refused proposals",
                note: "The button controls the open state and updates its label when the panel expands.",
                example: Box::new(collapse_controlled::demo(theme.clone())),
                source: include_str!("collapse_controlled.rs"),
            },
            DetailSection {
                id: "collapse-intrinsic",
                title: "Changing intrinsic content height",
                note: "Height presentation measures actual content. Changing a paragraph or viewport can retarget an active transition; no fixed open-height guess or per-frame application Cage is required.",
                example: Box::new(collapse_intrinsic::demo(theme.clone())),
                source: include_str!("collapse_intrinsic.rs"),
            },
            DetailSection {
                id: "collapse-motion",
                title: "Duration, reduced motion and actual completion",
                note: "Toggle the panel to run the disclosure transition.",
                example: Box::new(collapse_motion::demo(theme.clone())),
                source: include_str!("collapse_motion.rs"),
            },
            DetailSection {
                id: "collapse-lifetime",
                title: "Retained drafts, unmount and focus",
                note: "KeepMounted retains the real editor through accepted closes. Unmount retires content after the matching closing terminal event. Closing is logically hidden/inert immediately and returns focus when it contained the current editor.",
                example: Box::new(collapse_lifetime::demo(theme.clone())),
                source: include_str!("collapse_lifetime.rs"),
            },
        ],
        "divider" => vec![
            DetailSection {
                id: "divider-sizes",
                title: "Sizes",
                note: "Compare five rule thicknesses.",
                example: Box::new(divider_sizes::demo(theme.clone())),
                source: include_str!("divider_sizes.rs"),
            },
            DetailSection {
                id: "divider-patterns",
                title: "Solid, dashed and dotted rules",
                note: "Patterns are real independent background/border declarations. Thickness changes the finite physical stroke; dashed and dotted use the shared typed border styles with actual native raster coverage.",
                example: Box::new(divider_patterns::demo(theme.clone())),
                source: include_str!("divider_patterns.rs"),
            },
            DetailSection {
                id: "divider-labels",
                title: "Start, center and end labels",
                note: "Labeled rules compose ordinary visible text and decorative lines in a legal wrapper. The low-level passive separator remains an empty intrinsic node; the composition does not force children into it. Labels are reactive, nonempty single-line strings up to 256 UTF-8 bytes.",
                example: Box::new(divider_labels::demo(theme.clone())),
                source: include_str!("divider_labels.rs"),
            },
            DetailSection {
                id: "divider-vertical",
                title: "Vertical orientation",
                note: "Six labels demonstrate vertical separators from one to five pixels thick.",
                example: Box::new(divider_vertical::demo(theme.clone())),
                source: include_str!("divider_vertical.rs"),
            },
            DetailSection {
                id: "divider-long",
                title: "Long labels and controlled text",
                note: "Long prose wraps inside the label while line parts keep a finite minimum extent. Invalid reactive text preserves the accepted label and can recover without replacing its nodes.",
                example: Box::new(divider_long::demo(theme.clone())),
                source: include_str!("divider_long.rs"),
            },
        ],
        "focus-trap" => vec![
            DetailSection {
                id: "focus-initial",
                title: "Explicit initial focus and return focus",
                note: "Activating the trap moves focus to the second input. Deactivating it returns focus to the toggle.",
                example: Box::new(focus_initial::demo(theme.clone())),
                source: include_str!("focus_initial.rs"),
            },
            DetailSection {
                id: "focus-policies",
                title: "Dynamic focus policies",
                note: "Looping, containment and restoration are independent current configuration flags. These controls stay inside the active scope, so they remain reachable by keyboard.",
                example: Box::new(focus_policies::demo(theme.clone())),
                source: include_str!("focus_policies.rs"),
            },
            DetailSection {
                id: "focus-nested",
                title: "Nested focus scopes",
                note: "The inner scope temporarily owns focus and restores its current invoking action when closed. The outer scope remains active until its own Done action; no modal or outside inertness is implied.",
                example: Box::new(focus_nested::demo(theme.clone())),
                source: include_str!("focus_nested.rs"),
            },
        ],
        "marquee" => vec![
            DetailSection {
                id: "marquee-hover",
                title: "Pause on hover",
                note: "Move the pointer over the message to pause it and leave to resume.",
                example: Box::new(marquee_hover::demo(theme.clone())),
                source: concat!(
                    include_str!("marquee_hover.rs"),
                    "\n",
                    include_str!("marquee_logos.rs")
                ),
            },
            DetailSection {
                id: "marquee-vertical",
                title: "Vertical orientation",
                note: "The clipped track scrolls from bottom to top.",
                example: Box::new(marquee_vertical::demo(theme.clone())),
                source: concat!(
                    include_str!("marquee_vertical.rs"),
                    "\n",
                    include_str!("marquee_logos.rs")
                ),
            },
            DetailSection {
                id: "marquee-rows",
                title: "Multiple rows",
                note: "The two tracks move in opposite directions.",
                example: Box::new(marquee_rows::demo(theme.clone())),
                source: concat!(
                    include_str!("marquee_rows.rs"),
                    "\n",
                    include_str!("marquee_logos.rs")
                ),
            },
            DetailSection {
                id: "marquee-fade",
                title: "Fade edges",
                note: "Compare the same message with and without faded edges.",
                example: Box::new(marquee_fade::demo(theme.clone())),
                source: concat!(
                    include_str!("marquee_fade.rs"),
                    "\n",
                    include_str!("marquee_logos.rs")
                ),
            },
            DetailSection {
                id: "marquee-style-play",
                title: "Customization",
                note: "Adjust the scrolling direction, spacing, timing and hover behavior.",
                example: Box::new(marquee_style_play::demo(theme.clone())),
                source: include_str!("marquee_style_play.rs"),
            },
            DetailSection {
                id: "marquee-direction",
                title: "Left and right direction",
                note: "The finite text-only marquee moves in the selected direction. A semantic original remains separate from decorative repeated copies; this API does not accept arbitrary logo/widget children.",
                example: Box::new(marquee_direction::demo(theme.clone())),
                source: include_str!("marquee_direction.rs"),
            },
            DetailSection {
                id: "marquee-speed",
                title: "Period and measured repeated text",
                note: "period_millis controls a full measured cycle rather than a guessed pixels-per-second value. A changed period updates the current presentation without a per-frame application timer.",
                example: Box::new(marquee_speed::demo(theme.clone())),
                source: include_str!("marquee_speed.rs"),
            },
            DetailSection {
                id: "marquee-static",
                title: "Pause, explicit static output and Reduce",
                note: "paused stops the current presentation; animated false requests static output. The browser also honors reduced motion and the native host has an explicit motion preference. The text is not a repeating live announcement.",
                example: Box::new(marquee_static::demo(theme.clone())),
                source: include_str!("marquee_static.rs"),
            },
        ],
        "paper" => vec![
            DetailSection {
                id: "paper-surfaces",
                title: "Border and shadow choices",
                note: "Border and shadow are independent surface declarations. The same ordinary content can use a quiet outline or a raised surface without changing its semantic kind.",
                example: Box::new(paper_surfaces::demo(theme.clone())),
                source: include_str!("paper_surfaces.rs"),
            },
            DetailSection {
                id: "paper-spacing",
                title: "Radius and padding",
                note: "Radius and padding use explicit finite pixel Style fields. They do not change document hierarchy or make Paper interactive.",
                example: Box::new(paper_spacing::demo(theme.clone())),
                source: include_str!("paper_spacing.rs"),
            },
            DetailSection {
                id: "paper-card",
                title: "Paper and Card composition",
                note: "Paper is a surface with arbitrary children. Card adds explicit header/body/footer and heading conventions; either can contain a separate real action.",
                example: Box::new(paper_card::demo(theme.clone())),
                source: include_str!("paper_card.rs"),
            },
        ],
        "portal" => vec![
            DetailSection {
                id: "portal-destinations",
                title: "Explicit destinations and retained editing",
                note: "Portal renders into a registered current target, not an implicit global document body. Switching the target moves the same editor and its owner; application state and physical editing identity remain intact.",
                example: Box::new(portal_destinations::demo(theme.clone())),
                source: include_str!("portal_destinations.rs"),
            },
            DetailSection {
                id: "portal-availability",
                title: "Missing target and recovery",
                note: "Temporarily remove the destination. The source remains owned and parks its content; restoring the matching target recovers the same editor. The callback reports current target loss, not a history of visited targets.",
                example: Box::new(portal_availability::demo(theme.clone())),
                source: include_str!("portal_availability.rs"),
            },
            DetailSection {
                id: "portal-enabled",
                title: "Explicitly enabled source and bounded ownership",
                note: "enabled controls whether the current source is attached. Targets and sources belong to an explicit context and current live owners; missing targets do not create arbitrary HTML containers or grant document-wide access.",
                example: Box::new(portal_enabled::demo(theme.clone())),
                source: include_str!("portal_enabled.rs"),
            },
        ],
        "scroll-area" => vec![
            DetailSection {
                id: "scroll-horizontal",
                title: "Horizontal scrolling",
                note: "The content is wider than the scroll viewport.",
                example: Box::new(scroll_horizontal::demo(theme.clone())),
                source: include_str!("scroll_horizontal.rs"),
            },
            DetailSection {
                id: "scroll-disable-horizontal",
                title: "Disable horizontal scrolling",
                note: "The text wraps inside the viewport and scrolls vertically.",
                example: Box::new(scroll_disable_horizontal::demo(theme.clone())),
                source: include_str!("scroll_disable_horizontal.rs"),
            },
            DetailSection {
                id: "scroll-into-view",
                title: "Scroll element into view",
                note: "Use the next and previous controls to bring the highlighted row into view.",
                example: Box::new(scroll_into_view::demo(theme.clone())),
                source: include_str!("scroll_into_view.rs"),
            },
            DetailSection {
                id: "scroll-popover",
                title: "ScrollArea.Autosize with Popover",
                note: "Open the results and scroll inside the bounded popup.",
                example: Box::new(scroll_popover::demo(theme.clone())),
                source: include_str!("scroll_popover.rs"),
            },
            DetailSection {
                id: "scroll-observed",
                title: "Subscribe to scroll position changes",
                note: "Scroll the reading region to update its measured position.",
                example: Box::new(scroll_observed::demo(theme.clone())),
                source: include_str!("scroll_observed.rs"),
            },
            DetailSection {
                id: "scroll-boundaries",
                title: "Scroll boundary callbacks",
                note: "Each colored counter records a return to its own edge of the two-axis viewport.",
                example: Box::new(scroll_boundaries::demo(theme.clone())),
                source: include_str!("scroll_boundaries.rs"),
            },
            DetailSection {
                id: "scroll-start",
                title: "Start scroll position",
                note: "The article opens partway down, at an offset of 250 pixels.",
                example: Box::new(scroll_start::demo(theme.clone())),
                source: include_str!("scroll_start.rs"),
            },
            DetailSection {
                id: "scroll-autosize",
                title: "ScrollArea.Autosize",
                note: "Add and remove paragraphs. The reading area grows until its maximum height of 300 pixels.",
                example: Box::new(scroll_autosize::demo(theme.clone())),
                source: include_str!("scroll_autosize.rs"),
            },
            DetailSection {
                id: "scroll-side-play",
                title: "Vertical scrollbar position",
                note: "Move the scrollbar and reserve space on the same side.",
                example: Box::new(scroll_side_play::demo(theme.clone())),
                source: include_str!("scroll_side_play.rs"),
            },
            DetailSection {
                id: "scroll-axes",
                title: "Vertical, horizontal and both axes",
                note: "Axes are explicit. A wide content box plus Both allows real horizontal keyboard movement; a definite viewport height provides independent vertical overflow.",
                example: Box::new(scroll_axes::demo(theme.clone())),
                source: include_str!("scroll_axes.rs"),
            },
            DetailSection {
                id: "scroll-policies",
                title: "Scrollbar policy and size",
                note: "Five policies use real scrollbars. Never retains scrolling. Hover/Scroll use an explicit 300 ms browser timer; native applications supply their own local clock.",
                example: Box::new(scroll_policies::demo(theme.clone())),
                source: include_str!("scroll_policies.rs"),
            },
            DetailSection {
                id: "scroll-offsets",
                title: "Offset size",
                note: "The vertical scrollbar is six pixels wide and the content leaves a twenty-pixel gutter.",
                example: Box::new(scroll_offsets::demo(theme.clone())),
                source: include_str!("scroll_offsets.rs"),
            },
            DetailSection {
                id: "scroll-position",
                title: "Scroll to position",
                note: "Use the three buttons to move to the bottom, center or top of the reading area.",
                example: Box::new(scroll_position::demo(theme.clone())),
                source: include_str!("scroll_position.rs"),
            },
        ],
        "toolbar" => vec![
            DetailSection {
                id: "toolbar-contrast",
                title: "Auto contrast",
                note: "Compare fixed and automatically selected foreground colors on a light fill.",
                example: Box::new(toolbar_contrast::demo(theme.clone())),
                source: concat!(
                    include_str!("toolbar_contrast.rs"),
                    "\n",
                    include_str!("toolbar_format.rs")
                ),
            },
            DetailSection {
                id: "toolbar-disabled",
                title: "Disabled state",
                note: "Toggle the toolbar's enabled state and try a command.",
                example: Box::new(toolbar_disabled::demo(theme.clone())),
                source: concat!(
                    include_str!("toolbar_disabled.rs"),
                    "\n",
                    include_str!("toolbar_format.rs")
                ),
            },
            DetailSection {
                id: "toolbar-auto-width",
                title: "Auto width",
                note: "The toolbar fits its actions and keeps related commands together.",
                example: Box::new(toolbar_auto_width::demo(theme.clone())),
                source: concat!(
                    include_str!("toolbar_auto_width.rs"),
                    "\n",
                    include_str!("toolbar_format.rs")
                ),
            },
            DetailSection {
                id: "toolbar-style-play",
                title: "Size, orientation, variant and color",
                note: "Set the toolbar appearance and toggle formatting or alignment.",
                example: Box::new(toolbar_style_play::demo(theme.clone())),
                source: include_str!("toolbar_style_play.rs"),
            },
            DetailSection {
                id: "toolbar-groups",
                title: "Compact groups, pressed state and passive icons",
                note: "Toolbar accepts real Buttons, including controlled pressed state and passive graphic content. Group boundaries remain named; icon-only actions retain accessible names without using font-glyph substitutes.",
                example: Box::new(toolbar_groups::demo(theme.clone())),
                source: concat!(
                    include_str!("toolbar_groups.rs"),
                    "\n",
                    include_str!("toolbar_format.rs")
                ),
            },
            DetailSection {
                id: "toolbar-keyboard",
                title: "Roving focus, disabled items and wrap",
                note: "The toolbar owns one automatic Tab stop and current arrow/Home/End navigation. Disabled items are skipped. Horizontal groups wrap at narrow widths while labels remain on one line.",
                example: Box::new(toolbar_keyboard::demo(theme.clone())),
                source: include_str!("toolbar_keyboard.rs"),
            },
            DetailSection {
                id: "toolbar-direction",
                title: "Vertical layout and RTL navigation",
                note: "Orientation selects the navigation axis; Direction selects logical horizontal behavior. Vertical layout does not invent a horizontal keyboard policy, and explicit tab stops remain owned by the toolbar.",
                example: Box::new(toolbar_direction::demo(theme.clone())),
                source: include_str!("toolbar_direction.rs"),
            },
            DetailSection {
                id: "toolbar-overflow",
                title: "Explicit More composition",
                note: "Toolbar wraps current groups by default. Arbitrary movable controls belong to the separate OverflowList controller; this example composes a compact toolbar and an explicit finite More region rather than hiding a second toolbar controller.",
                example: Box::new(toolbar_overflow::demo(theme.clone())),
                source: include_str!("toolbar_overflow.rs"),
            },
        ],
        "visually-hidden" => vec![
            DetailSection {
                id: "hidden-name",
                title: "An icon action with an accessible text name",
                note: "VisuallyHidden supplies plain text without an in-flow box or hidden/inert state. An actual Button owns keyboard and activation; the decorative finite drawing has no competing alternative.",
                example: Box::new(hidden_name::demo(theme.clone())),
                source: include_str!("hidden_name.rs"),
            },
            DetailSection {
                id: "hidden-description",
                title: "Explicit description relations",
                note: "A hidden plain-text node can be the target of a real described_by relationship. The button keeps its visible name; supplemental text does not replace that name.",
                example: Box::new(hidden_description::demo(theme.clone())),
                source: include_str!("hidden_description.rs"),
            },
            DetailSection {
                id: "hidden-focus",
                title: "Focusable links must reveal focus",
                note: "VisuallyHidden accepts plain text and cannot contain an invisible interactive link. A separate real anchor keeps its Tab stop and changes its own clipped presentation on FocusChanged so keyboard focus becomes visible.",
                example: Box::new(hidden_focus::demo(theme.clone())),
                source: include_str!("hidden_focus.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "box" => vec![
            ("Explicit independent style declarations", "#box-style"),
            ("Composition with real actions", "#box-composition"),
            ("Reactive appearance and the HTML layer", "#box-reactive"),
        ],
        "collapse" => vec![
            ("Horizontal orientation", "#collapse-horizontal"),
            ("Example: nested Collapse components", "#collapse-nested"),
            (
                "Controlled opening and refused proposals",
                "#collapse-controlled",
            ),
            ("Changing intrinsic content height", "#collapse-intrinsic"),
            (
                "Duration, reduced motion and actual completion",
                "#collapse-motion",
            ),
            ("Retained drafts, unmount and focus", "#collapse-lifetime"),
        ],
        "divider" => vec![
            ("Sizes", "#divider-sizes"),
            ("Solid, dashed and dotted rules", "#divider-patterns"),
            ("Start, center and end labels", "#divider-labels"),
            ("Vertical orientation", "#divider-vertical"),
            ("Long labels and controlled text", "#divider-long"),
        ],
        "focus-trap" => vec![
            ("Explicit initial focus and return focus", "#focus-initial"),
            ("Dynamic focus policies", "#focus-policies"),
            ("Nested focus scopes", "#focus-nested"),
        ],
        "marquee" => vec![
            ("Pause on hover", "#marquee-hover"),
            ("Vertical orientation", "#marquee-vertical"),
            ("Multiple rows", "#marquee-rows"),
            ("Fade edges", "#marquee-fade"),
            ("Customization", "#marquee-style-play"),
            ("Left and right direction", "#marquee-direction"),
            ("Period and measured repeated text", "#marquee-speed"),
            (
                "Pause, explicit static output and Reduce",
                "#marquee-static",
            ),
        ],
        "paper" => vec![
            ("Border and shadow choices", "#paper-surfaces"),
            ("Radius and padding", "#paper-spacing"),
            ("Paper and Card composition", "#paper-card"),
        ],
        "portal" => vec![
            (
                "Explicit destinations and retained editing",
                "#portal-destinations",
            ),
            ("Missing target and recovery", "#portal-availability"),
            (
                "Explicitly enabled source and bounded ownership",
                "#portal-enabled",
            ),
        ],
        "scroll-area" => vec![
            ("Horizontal scrolling", "#scroll-horizontal"),
            ("Disable horizontal scrolling", "#scroll-disable-horizontal"),
            ("Scroll element into view", "#scroll-into-view"),
            ("ScrollArea.Autosize with Popover", "#scroll-popover"),
            ("Subscribe to scroll position changes", "#scroll-observed"),
            ("Scroll boundary callbacks", "#scroll-boundaries"),
            ("Start scroll position", "#scroll-start"),
            ("ScrollArea.Autosize", "#scroll-autosize"),
            ("Vertical scrollbar position", "#scroll-side-play"),
            ("Vertical, horizontal and both axes", "#scroll-axes"),
            ("Scrollbar policy and size", "#scroll-policies"),
            ("Offsets and nested scrolling", "#scroll-offsets"),
            ("Explicit current-position commands", "#scroll-position"),
        ],
        "toolbar" => vec![
            ("Auto contrast", "#toolbar-contrast"),
            ("Disabled state", "#toolbar-disabled"),
            ("Auto width", "#toolbar-auto-width"),
            (
                "Size, orientation, variant and color",
                "#toolbar-style-play",
            ),
            (
                "Compact groups, pressed state and passive icons",
                "#toolbar-groups",
            ),
            ("Roving focus, disabled items and wrap", "#toolbar-keyboard"),
            ("Vertical layout and RTL navigation", "#toolbar-direction"),
            ("Explicit More composition", "#toolbar-overflow"),
        ],
        "visually-hidden" => vec![
            (
                "An icon action with an accessible text name",
                "#hidden-name",
            ),
            ("Explicit description relations", "#hidden-description"),
            ("Focusable links must reveal focus", "#hidden-focus"),
        ],
        _ => Vec::new(),
    }
}
