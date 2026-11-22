mod code_copy_button;
mod code_custom_controls;
mod code_expand;
mod code_first_indent;
mod code_inline;
mod code_tabs_detected;
mod code_tabs_icons;
mod json_all_expanded;
mod json_chevrons;
mod json_expand_controls;
mod json_expand_depth;
mod json_highlight_items;
mod json_line_numbers;
mod json_long_strings;
mod json_root_name;
mod json_sort_keys;
mod json_types;
mod lightbox_all_features;
mod lightbox_custom_slides;
mod lightbox_custom_toolbar;
mod lightbox_loop;
mod lightbox_no_animation;
mod lightbox_outside;
mod lightbox_slide_transition;
mod lightbox_swipe;
mod lightbox_thumbnails;
mod lightbox_transition;
mod lightbox_video;
mod modal_content;
mod modal_custom_confirm;
mod modal_dynamic;
mod modal_stacked;
mod notice_custom_render;
mod notice_custom_style;
mod notice_priority;
mod notice_progress;
mod notice_progress_position;
mod notice_promise;
mod notice_store;
mod spotlight_fuzzy;
mod spotlight_limit;
mod spotlight_scroll;
use super::showcase_examples::DetailSection;
mod code_highlight_files;
mod code_highlight_languages;
mod code_highlight_lines;
mod code_highlight_long_lines;
mod code_highlight_theme;
mod json_viewer_copy;
mod json_viewer_expansion;
mod json_viewer_paths;
mod json_viewer_updates;
mod lightbox_controlled;
mod lightbox_metadata;
mod lightbox_navigation;
mod lightbox_zoom;
mod modals_manager_closing;
mod modals_manager_focus;
mod modals_manager_metadata;
mod modals_manager_queue;
mod navigation_progress_cancel;
mod navigation_progress_concurrency;
mod navigation_progress_finish;
mod navigation_progress_replacement;
mod notifications_system_loading;
mod notifications_system_position;
mod notifications_system_stack;
mod notifications_system_timeouts;
mod notifications_system_updates;
mod spotlight_controlled;
mod spotlight_groups;
mod spotlight_search;
mod spotlight_shortcut;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "code-highlight" => vec![
            DetailSection {
                id: "code-tabs-icons",
                title: "Tabs with icons",
                note: "Switch files using the icon beside each filename.",
                example: Box::new(code_tabs_icons::demo(theme.clone())),
                source: include_str!("code_tabs_icons.rs"),
            },
            DetailSection {
                id: "code-tabs-detected",
                title: "Tabs icons based on file name",
                note: "Choose a source file to see its type-specific icon and highlighted code.",
                example: Box::new(code_tabs_detected::demo(theme.clone())),
                source: include_str!("code_tabs_detected.rs"),
            },
            DetailSection {
                id: "code-first-indent",
                title: "First line indentation",
                note: "Compare the displayed indentation on the first line.",
                example: Box::new(code_first_indent::demo(theme.clone())),
                source: include_str!("code_first_indent.rs"),
            },
            DetailSection {
                id: "code-custom-controls",
                title: "Custom controls",
                note: "Switch the code version or copy the active listing.",
                example: Box::new(code_custom_controls::demo(theme.clone())),
                source: include_str!("code_custom_controls.rs"),
            },
            DetailSection {
                id: "code-copy-button",
                title: "Copy button",
                note: "Copy the displayed source and check the clipboard feedback.",
                example: Box::new(code_copy_button::demo(theme.clone())),
                source: include_str!("code_copy_button.rs"),
            },
            DetailSection {
                id: "code-expand",
                title: "Expandable code",
                note: "Show or hide the rest of the listing.",
                example: Box::new(code_expand::demo(theme.clone())),
                source: include_str!("code_expand.rs"),
            },
            DetailSection {
                id: "code-inline",
                title: "Inline code",
                note: "Read the highlighted expression within a sentence.",
                example: Box::new(code_inline::demo(theme.clone())),
                source: include_str!("code_inline.rs"),
            },
            DetailSection {
                id: "extensions-code-highlight-languages",
                title: "Rust, JSON and plain text",
                note: "These are bounded lexical highlighters. A language change preserves the exact source; highlighting neither compiles Rust nor validates JSON.",
                example: Box::new(code_highlight_languages::demo(theme.clone())),
                source: include_str!("code_highlight_languages.rs"),
            },
            DetailSection {
                id: "extensions-code-highlight-theme",
                title: "Theme and unstyled ink",
                note: "The active page theme supplies syntax colors; unstyled removes ink while retaining selectable whitespace and scrolling.",
                example: Box::new(code_highlight_theme::demo(theme.clone())),
                source: include_str!("code_highlight_theme.rs"),
            },
            DetailSection {
                id: "extensions-code-highlight-lines",
                title: "Line numbers and highlighted lines",
                note: "Numbers are a separate inert gutter. start_line changes displayed numbers; highlighted_lines uses relative one-based lines.",
                example: Box::new(code_highlight_lines::demo(theme.clone())),
                source: include_str!("code_highlight_lines.rs"),
            },
            DetailSection {
                id: "extensions-code-highlight-long_lines",
                title: "Long lines stay inside the code viewport",
                note: "A long literal scrolls locally. The page and neighboring content keep their width; the complete source remains selectable.",
                example: Box::new(code_highlight_long_lines::demo(theme.clone())),
                source: include_str!("code_highlight_long_lines.rs"),
            },
            DetailSection {
                id: "extensions-code-highlight-files",
                title: "Multiple files and copy feedback",
                note: "The application owns the current file; changing it replaces only the accepted source. Copy feedback is beside its action and never claims success before the host result.",
                example: Box::new(code_highlight_files::demo(theme.clone())),
                source: include_str!("code_highlight_files.rs"),
            },
        ],
        "json-viewer" => vec![
            DetailSection {
                id: "json-types",
                title: "Type badges and size",
                note: "Open a container to inspect its size and the type of each value.",
                example: Box::new(json_types::demo(theme.clone())),
                source: include_str!("json_types.rs"),
            },
            DetailSection {
                id: "json-long-strings",
                title: "Collapse long strings",
                note: "Expand either long string independently.",
                example: Box::new(json_long_strings::demo(theme.clone())),
                source: include_str!("json_long_strings.rs"),
            },
            DetailSection {
                id: "json-highlight-items",
                title: "Highlight items",
                note: "Inspect additions and removals at their exact JSON paths.",
                example: Box::new(json_highlight_items::demo(theme.clone())),
                source: include_str!("json_highlight_items.rs"),
            },
            DetailSection {
                id: "json-sort-keys",
                title: "Sort keys",
                note: "Object keys are sorted at every level. Array items retain their source order.",
                example: Box::new(json_sort_keys::demo(theme.clone())),
                source: include_str!("json_sort_keys.rs"),
            },
            DetailSection {
                id: "json-line-numbers",
                title: "Line numbers",
                note: "Expand and collapse an object to see the visible line numbers update.",
                example: Box::new(json_line_numbers::demo(theme.clone())),
                source: include_str!("json_line_numbers.rs"),
            },
            DetailSection {
                id: "json-chevrons",
                title: "Chevrons",
                note: "Use each chevron to expand or collapse its object.",
                example: Box::new(json_chevrons::demo(theme.clone())),
                source: include_str!("json_chevrons.rs"),
            },
            DetailSection {
                id: "json-expand-depth",
                title: "Default expand depth",
                note: "Inspect the structure of the project configuration.",
                example: Box::new(json_expand_depth::demo(theme.clone())),
                source: include_str!("json_expand_depth.rs"),
            },
            DetailSection {
                id: "json-root-name",
                title: "Root name",
                note: "Inspect the structure of the project configuration.",
                example: Box::new(json_root_name::demo(theme.clone())),
                source: include_str!("json_root_name.rs"),
            },
            DetailSection {
                id: "json-all-expanded",
                title: "All expanded",
                note: "Inspect the structure of the project configuration.",
                example: Box::new(json_all_expanded::demo(theme.clone())),
                source: include_str!("json_all_expanded.rs"),
            },
            DetailSection {
                id: "json-expand-controls",
                title: "Expand all and collapse all",
                note: "Inspect the structure of the project configuration.",
                example: Box::new(json_expand_controls::demo(theme.clone())),
                source: include_str!("json_expand_controls.rs"),
            },
            DetailSection {
                id: "extensions-json-viewer-expansion",
                title: "Controlled expansion and refusal",
                note: "Expanded JSON pointers are read-only input. A callback proposes the next paths and the application may refuse them.",
                example: Box::new(json_viewer_expansion::demo(theme.clone())),
                source: include_str!("json_viewer_expansion.rs"),
            },
            DetailSection {
                id: "extensions-json-viewer-paths",
                title: "JSON pointers, escaping and keyboard navigation",
                note: "A slash is ~1 and a tilde is ~0 in JSON pointers. Tree mode uses real TreeItem keyboard navigation; the displayed values remain ordinary text.",
                example: Box::new(json_viewer_paths::demo(theme.clone())),
                source: include_str!("json_viewer_paths.rs"),
            },
            DetailSection {
                id: "extensions-json-viewer-copy",
                title: "Copy the complete JSON source",
                note: "Collapsed branches do not truncate copying. Feedback is a real host result; the browser acceptance injects an isolated clipboard provider.",
                example: Box::new(json_viewer_copy::demo(theme.clone())),
                source: include_str!("json_viewer_copy.rs"),
            },
            DetailSection {
                id: "extensions-json-viewer-updates",
                title: "Search and accepted data updates",
                note: "Publish a complete valid snapshot. Searching highlights current values; updating release preserves the current paths and mounted editor.",
                example: Box::new(json_viewer_updates::demo(theme.clone())),
                source: include_str!("json_viewer_updates.rs"),
            },
        ],
        "notifications-system" => vec![
            DetailSection {
                id: "notice-priority",
                title: "Priority",
                note: "Show a low-priority notice, then a high-priority notice. Dismiss the urgent notice to reveal the first.",
                example: Box::new(notice_priority::demo(theme.clone())),
                source: include_str!("notice_priority.rs"),
            },
            DetailSection {
                id: "notice-custom-style",
                title: "Customize notification styles",
                note: "Show the default and error notices to compare their accents.",
                example: Box::new(notice_custom_style::demo(theme.clone())),
                source: include_str!("notice_custom_style.rs"),
            },
            DetailSection {
                id: "notice-promise",
                title: "Promise",
                note: "Save the document and follow the same notice through its success or error result.",
                example: Box::new(notice_promise::demo(theme.clone())),
                source: include_str!("notice_promise.rs"),
            },
            DetailSection {
                id: "notice-progress",
                title: "Auto close progress",
                note: "Watch the remaining lifetime decrease until the notification closes.",
                example: Box::new(notice_progress::demo(theme.clone())),
                source: include_str!("notice_progress.rs"),
            },
            DetailSection {
                id: "notice-progress-position",
                title: "Auto close progress position",
                note: "Choose which edge shows the remaining lifetime.",
                example: Box::new(notice_progress_position::demo(theme.clone())),
                source: include_str!("notice_progress_position.rs"),
            },
            DetailSection {
                id: "notice-custom-render",
                title: "Custom notification rendering",
                note: "Open a custom message card, then reply or dismiss it.",
                example: Box::new(notice_custom_render::demo(theme.clone())),
                source: include_str!("notice_custom_render.rs"),
            },
            DetailSection {
                id: "notice-store",
                title: "Subscribe to notifications state",
                note: "Show several notifications, then dismiss one and inspect the live queue.",
                example: Box::new(notice_store::demo(theme.clone())),
                source: include_str!("notice_store.rs"),
            },
            DetailSection {
                id: "extensions-notifications-system-stack",
                title: "Stacking and bounded queues",
                note: "The local queue admits eight messages and displays two. Dismissing a visible row promotes the next request; repeated keys replace the current lifetime.",
                example: Box::new(notifications_system_stack::demo(theme.clone())),
                source: include_str!("notifications_system_stack.rs"),
            },
            DetailSection {
                id: "extensions-notifications-system-updates",
                title: "Update, dismiss and application actions",
                note: "Updating a live token preserves its notification row. Dismiss retires the request; the Undo action executes an application command after removing its row.",
                example: Box::new(notifications_system_updates::demo(theme.clone())),
                source: include_str!("notifications_system_updates.rs"),
            },
            DetailSection {
                id: "extensions-notifications-system-position",
                title: "Viewport placement belongs to the application",
                note: "Affix positions this one local Toaster at the lower right. Ordinary layout remains the default; there is no global notifications singleton.",
                example: Box::new(notifications_system_position::demo(theme.clone())),
                source: include_str!("notifications_system_position.rs"),
            },
            DetailSection {
                id: "extensions-notifications-system-timeouts",
                title: "Timeout, pause and resume",
                note: "This example explicitly supplies a monotonic browser clock and cancellable timer. Hover, focus or Pause preserve remaining time; queued deadlines begin only after promotion. Cold SSR/native previews install a pending clock.",
                example: Box::new(notifications_system_timeouts::demo(theme.clone())),
                source: include_str!("notifications_system_timeouts.rs"),
            },
            DetailSection {
                id: "extensions-notifications-system-loading",
                title: "Inline loading is independent of the queue",
                note: "An inline Notification can show a busy operation and block dismissal until completion. Toaster owns requests and deadlines; its update command updates title/message, rather than claiming a loading API it does not expose.",
                example: Box::new(notifications_system_loading::demo(theme.clone())),
                source: include_str!("notifications_system_loading.rs"),
            },
        ],
        "spotlight" => vec![
            DetailSection {
                id: "spotlight-limit",
                title: "Limit prop",
                note: "Search the catalog and show at most three matching actions.",
                example: Box::new(spotlight_limit::demo(theme.clone())),
                source: include_str!("spotlight_limit.rs"),
            },
            DetailSection {
                id: "spotlight-fuzzy",
                title: "Fuzzy search",
                note: "Find commands using the letters of their names in order.",
                example: Box::new(spotlight_fuzzy::demo(theme.clone())),
                source: include_str!("spotlight_fuzzy.rs"),
            },
            DetailSection {
                id: "spotlight-scroll",
                title: "Scrollable actions list",
                note: "Scroll the list or use the arrow keys to reach more actions.",
                example: Box::new(spotlight_scroll::demo(theme.clone())),
                source: include_str!("spotlight_scroll.rs"),
            },
            DetailSection {
                id: "extensions-spotlight-search",
                title: "Search, keywords and empty results",
                note: "Search labels, descriptions and keywords. Try fresh for Create note, then an unmatched query for the actual empty state. Arrow keys and Enter keep input focus until execution.",
                example: Box::new(spotlight_search::demo(theme.clone())),
                source: include_str!("spotlight_search.rs"),
            },
            DetailSection {
                id: "extensions-spotlight-groups",
                title: "Grouped actions and disabled commands",
                note: "Workspace and Preferences are real result groups. Disabled Archive workspace is visible but cannot execute; descriptions explain each command.",
                example: Box::new(spotlight_groups::demo(theme.clone())),
                source: include_str!("spotlight_groups.rs"),
            },
            DetailSection {
                id: "extensions-spotlight-shortcut",
                title: "Explicitly scoped Ctrl/Cmd+K",
                note: "Focus the Keyboard shortcut target and use Ctrl/Cmd+K. Registration belongs to this live physical ancestor; ordinary editors and composition keep their shortcut guards. No global listener is installed.",
                example: Box::new(spotlight_shortcut::demo(theme.clone())),
                source: include_str!("spotlight_shortcut.rs"),
            },
            DetailSection {
                id: "extensions-spotlight-controlled",
                title: "Controlled opening and search refusal",
                note: "Open and search are separate typed proposals. Initially opening is refused; accepting changes permits the same retained palette to open and search.",
                example: Box::new(spotlight_controlled::demo(theme.clone())),
                source: include_str!("spotlight_controlled.rs"),
            },
        ],
        "lightbox" => vec![
            DetailSection {
                id: "lightbox-video",
                title: "Video slides",
                note: "Play the local lake film and navigate to a still image.",
                example: Box::new(lightbox_video::demo(theme.clone())),
                source: include_str!("lightbox_video.rs"),
            },
            DetailSection {
                id: "lightbox-swipe",
                title: "Swipe to close",
                note: "Swipe down to close the gallery.",
                example: Box::new(lightbox_swipe::demo(theme.clone())),
                source: include_str!("lightbox_swipe.rs"),
            },
            DetailSection {
                id: "lightbox-transition",
                title: "Open and close transition",
                note: "Select the effect for opening and closing the gallery.",
                example: Box::new(lightbox_transition::demo(theme.clone())),
                source: include_str!("lightbox_transition.rs"),
            },
            DetailSection {
                id: "lightbox-no-animation",
                title: "Disable animations",
                note: "Open and close the gallery instantly.",
                example: Box::new(lightbox_no_animation::demo(theme.clone())),
                source: include_str!("lightbox_no_animation.rs"),
            },
            DetailSection {
                id: "lightbox-custom-toolbar",
                title: "Custom toolbar",
                note: "Open image details from the toolbar and navigate between photos.",
                example: Box::new(lightbox_custom_toolbar::demo(theme.clone())),
                source: include_str!("lightbox_custom_toolbar.rs"),
            },
            DetailSection {
                id: "lightbox-custom-slides",
                title: "Custom slides",
                note: "Navigate from an image to interactive slide content.",
                example: Box::new(lightbox_custom_slides::demo(theme.clone())),
                source: include_str!("lightbox_custom_slides.rs"),
            },
            DetailSection {
                id: "lightbox-slide-transition",
                title: "Slide transition",
                note: "Navigate between slides to see the entrance effect.",
                example: Box::new(lightbox_slide_transition::demo(theme.clone())),
                source: include_str!("lightbox_slide_transition.rs"),
            },
            DetailSection {
                id: "lightbox-outside",
                title: "Click outside to close",
                note: "Toggle whether the backdrop dismisses the open gallery.",
                example: Box::new(lightbox_outside::demo(theme.clone())),
                source: include_str!("lightbox_outside.rs"),
            },
            DetailSection {
                id: "lightbox-thumbnails",
                title: "Thumbnails",
                note: "Open the gallery and navigate between the photographs.",
                example: Box::new(lightbox_thumbnails::demo(theme.clone())),
                source: include_str!("lightbox_thumbnails.rs"),
            },
            DetailSection {
                id: "lightbox-loop",
                title: "Loop navigation",
                note: "Open the gallery and navigate between the photographs.",
                example: Box::new(lightbox_loop::demo(theme.clone())),
                source: include_str!("lightbox_loop.rs"),
            },
            DetailSection {
                id: "lightbox-all-features",
                title: "All features",
                note: "Open the gallery and navigate between the photographs.",
                example: Box::new(lightbox_all_features::demo(theme.clone())),
                source: include_str!("lightbox_all_features.rs"),
            },
            DetailSection {
                id: "extensions-lightbox-navigation",
                title: "Image navigation and boundaries",
                note: "Open either thumbnail, then use Previous/Next or focus Image navigation and press Arrow/Home/End. Boundaries disable the matching controls; image alternatives and captions remain separate.",
                example: Box::new(lightbox_navigation::demo(theme.clone())),
                source: include_str!("lightbox_navigation.rs"),
            },
            DetailSection {
                id: "extensions-lightbox-zoom",
                title: "Zoom, fit and local scrolling",
                note: "Zoom increases the current image from 100% to 400% inside its viewport. Fit resets the current zoom; this is finite stepped zoom, without a pinch-gesture promise.",
                example: Box::new(lightbox_zoom::demo(theme.clone())),
                source: include_str!("lightbox_zoom.rs"),
            },
            DetailSection {
                id: "extensions-lightbox-metadata",
                title: "Stable keys, alternatives and captions",
                note: "Change the lake caption before opening. The same image key selects the same photo; the current accepted caption appears in the gallery. The two local photos are fictional generated samples.",
                example: Box::new(lightbox_metadata::demo(theme.clone())),
                source: include_str!("lightbox_metadata.rs"),
            },
            DetailSection {
                id: "extensions-lightbox-controlled",
                title: "Controlled view and close refusal",
                note: "The application may refuse image/zoom changes. This example refuses the first close proposal once, then accepts the next close so the modal always has an exit. Focus restoration belongs to the shared Layer host.",
                example: Box::new(lightbox_controlled::demo(theme.clone())),
                source: include_str!("lightbox_controlled.rs"),
            },
        ],
        "navigation-progress" => vec![
            DetailSection {
                id: "extensions-navigation-progress-replacement",
                title: "Replacing a route invalidates its old token",
                note: "Start twice, then try to publish the previous route. The old token cannot change the retained current bar. Applications must also cancel their previous future.",
                example: Box::new(navigation_progress_replacement::demo(theme.clone())),
                source: include_str!("navigation_progress_replacement.rs"),
            },
            DetailSection {
                id: "extensions-navigation-progress-cancel",
                title: "Cancellation and a new operation",
                note: "Cancel hides the current bar and removes its numeric value. A later Start creates a new lifetime; no arbitrary network future is implicitly cancelled.",
                example: Box::new(navigation_progress_cancel::demo(theme.clone())),
                source: include_str!("navigation_progress_cancel.rs"),
            },
            DetailSection {
                id: "extensions-navigation-progress-finish",
                title: "Determinate progress and completion",
                note: "Reports accept finite percentages. Finish stores 100% and immediately hides the retained bar; there is no automatic increment or completion delay.",
                example: Box::new(navigation_progress_finish::demo(theme.clone())),
                source: include_str!("navigation_progress_finish.rs"),
            },
            DetailSection {
                id: "extensions-navigation-progress-concurrency",
                title: "One controller, one current operation",
                note: "Reports and Settings share this local controller: the latest start wins. Use independent controllers for unrelated concurrent work rather than aggregating their percentages implicitly.",
                example: Box::new(navigation_progress_concurrency::demo(theme.clone())),
                source: include_str!("navigation_progress_concurrency.rs"),
            },
        ],
        "modals-manager" => vec![
            DetailSection {
                id: "modal-stacked",
                title: "Multiple opened modals",
                note: "Open another step and close it to return to the first.",
                example: Box::new(modal_stacked::demo(theme.clone())),
                source: include_str!("modal_stacked.rs"),
            },
            DetailSection {
                id: "modal-dynamic",
                title: "Dynamic content and the modals manager",
                note: "Watch the title and contents change in the open dialog.",
                example: Box::new(modal_dynamic::demo(theme.clone())),
                source: include_str!("modal_dynamic.rs"),
            },
            DetailSection {
                id: "modal-custom-confirm",
                title: "Confirm modal",
                note: "Review a destructive action and choose whether to confirm it.",
                example: Box::new(modal_custom_confirm::demo(theme.clone())),
                source: include_str!("modal_custom_confirm.rs"),
            },
            DetailSection {
                id: "modal-content",
                title: "Content modals",
                note: "Open a dialog with form content and edit its input.",
                example: Box::new(modal_content::demo(theme.clone())),
                source: include_str!("modal_content.rs"),
            },
            DetailSection {
                id: "extensions-modals-manager-queue",
                title: "Sequential editor and confirmation",
                note: "Only the first request constructs its content. Continue closes the editor and reveals the queued confirmation; Cancel is the safe initial focus.",
                example: Box::new(modals_manager_queue::demo(theme.clone())),
                source: include_str!("modals_manager_queue.rs"),
            },
            DetailSection {
                id: "extensions-modals-manager-metadata",
                title: "Update metadata while preserving the editor",
                note: "Type a draft, then update the dialog title from inside the modal. Metadata changes retain the content Owner, input identity, focus and selection.",
                example: Box::new(modals_manager_metadata::demo(theme.clone())),
                source: include_str!("modals_manager_metadata.rs"),
            },
            DetailSection {
                id: "extensions-modals-manager-closing",
                title: "Close queued or current requests",
                note: "Remove waiting confirmation before closing the editor, or close the whole current sequence. Tokens are local to this manager; completed requests cannot be completed twice.",
                example: Box::new(modals_manager_closing::demo(theme.clone())),
                source: include_str!("modals_manager_closing.rs"),
            },
            DetailSection {
                id: "extensions-modals-manager-focus",
                title: "Focus and final restoration",
                note: "Open the form from its button, edit it, then save or press Escape. The shared modal Layer contains focus while open and restores the live original opener after the final request.",
                example: Box::new(modals_manager_focus::demo(theme.clone())),
                source: include_str!("modals_manager_focus.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "code-highlight" => vec![
            ("Tabs with icons", "#code-tabs-icons"),
            ("Tabs icons based on file name", "#code-tabs-detected"),
            ("First line indentation", "#code-first-indent"),
            ("Custom controls", "#code-custom-controls"),
            ("Copy button", "#code-copy-button"),
            ("Expandable code", "#code-expand"),
            ("Inline code", "#code-inline"),
            (
                "Rust, JSON and plain text",
                "#extensions-code-highlight-languages",
            ),
            ("Theme and unstyled ink", "#extensions-code-highlight-theme"),
            (
                "Line numbers and highlighted lines",
                "#extensions-code-highlight-lines",
            ),
            (
                "Long lines stay inside the code viewport",
                "#extensions-code-highlight-long_lines",
            ),
            (
                "Multiple files and copy feedback",
                "#extensions-code-highlight-files",
            ),
        ],
        "json-viewer" => vec![
            ("Type badges and size", "#json-types"),
            ("Collapse long strings", "#json-long-strings"),
            ("Highlight items", "#json-highlight-items"),
            ("Sort keys", "#json-sort-keys"),
            ("Line numbers", "#json-line-numbers"),
            ("Chevrons", "#json-chevrons"),
            ("Default expand depth", "#json-expand-depth"),
            ("Root name", "#json-root-name"),
            ("All expanded", "#json-all-expanded"),
            ("Expand all and collapse all", "#json-expand-controls"),
            (
                "Controlled expansion and refusal",
                "#extensions-json-viewer-expansion",
            ),
            (
                "JSON pointers, escaping and keyboard navigation",
                "#extensions-json-viewer-paths",
            ),
            (
                "Copy the complete JSON source",
                "#extensions-json-viewer-copy",
            ),
            (
                "Search and accepted data updates",
                "#extensions-json-viewer-updates",
            ),
        ],
        "notifications-system" => vec![
            ("Priority", "#notice-priority"),
            ("Customize notification styles", "#notice-custom-style"),
            ("Promise", "#notice-promise"),
            ("Auto close progress", "#notice-progress"),
            ("Auto close progress position", "#notice-progress-position"),
            ("Custom notification rendering", "#notice-custom-render"),
            ("Subscribe to notifications state", "#notice-store"),
            (
                "Stacking and bounded queues",
                "#extensions-notifications-system-stack",
            ),
            (
                "Update, dismiss and application actions",
                "#extensions-notifications-system-updates",
            ),
            (
                "Viewport placement belongs to the application",
                "#extensions-notifications-system-position",
            ),
            (
                "Timeout, pause and resume",
                "#extensions-notifications-system-timeouts",
            ),
            (
                "Inline loading is independent of the queue",
                "#extensions-notifications-system-loading",
            ),
        ],
        "spotlight" => vec![
            ("Limit prop", "#spotlight-limit"),
            ("Fuzzy search", "#spotlight-fuzzy"),
            ("Scrollable actions list", "#spotlight-scroll"),
            (
                "Search, keywords and empty results",
                "#extensions-spotlight-search",
            ),
            (
                "Grouped actions and disabled commands",
                "#extensions-spotlight-groups",
            ),
            (
                "Explicitly scoped Ctrl/Cmd+K",
                "#extensions-spotlight-shortcut",
            ),
            (
                "Controlled opening and search refusal",
                "#extensions-spotlight-controlled",
            ),
        ],
        "lightbox" => vec![
            ("Video slides", "#lightbox-video"),
            ("Swipe to close", "#lightbox-swipe"),
            ("Open and close transition", "#lightbox-transition"),
            ("Disable animations", "#lightbox-no-animation"),
            ("Custom toolbar", "#lightbox-custom-toolbar"),
            ("Custom slides", "#lightbox-custom-slides"),
            ("Slide transition", "#lightbox-slide-transition"),
            ("Click outside to close", "#lightbox-outside"),
            ("Thumbnails", "#lightbox-thumbnails"),
            ("Loop navigation", "#lightbox-loop"),
            ("All features", "#lightbox-all-features"),
            (
                "Image navigation and boundaries",
                "#extensions-lightbox-navigation",
            ),
            ("Zoom, fit and local scrolling", "#extensions-lightbox-zoom"),
            (
                "Stable keys, alternatives and captions",
                "#extensions-lightbox-metadata",
            ),
            (
                "Controlled view and close refusal",
                "#extensions-lightbox-controlled",
            ),
        ],
        "navigation-progress" => vec![
            (
                "Replacing a route invalidates its old token",
                "#extensions-navigation-progress-replacement",
            ),
            (
                "Cancellation and a new operation",
                "#extensions-navigation-progress-cancel",
            ),
            (
                "Determinate progress and completion",
                "#extensions-navigation-progress-finish",
            ),
            (
                "One controller, one current operation",
                "#extensions-navigation-progress-concurrency",
            ),
        ],
        "modals-manager" => vec![
            ("Multiple opened modals", "#modal-stacked"),
            ("Dynamic content and the modals manager", "#modal-dynamic"),
            ("Confirm modal", "#modal-custom-confirm"),
            ("Content modals", "#modal-content"),
            (
                "Sequential editor and confirmation",
                "#extensions-modals-manager-queue",
            ),
            (
                "Update metadata while preserving the editor",
                "#extensions-modals-manager-metadata",
            ),
            (
                "Close queued or current requests",
                "#extensions-modals-manager-closing",
            ),
            (
                "Focus and final restoration",
                "#extensions-modals-manager-focus",
            ),
        ],
        _ => Vec::new(),
    }
}
