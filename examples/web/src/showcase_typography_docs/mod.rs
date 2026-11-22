mod code_block;
mod code_colors;
mod code_inline;
mod highlight_accents;
mod highlight_case;
mod highlight_empty;
mod highlight_literals;
mod highlight_mark_style;
mod highlight_multiple;
mod highlight_per_term;
mod highlight_text_props;
mod highlight_whole_words;
mod list_icons;
mod list_nested;
mod list_order;
mod list_reversed;
mod list_spacing;
mod list_start;
mod list_values;
mod mark_colors;
mod mark_highlight;
mod mark_wrapping;
mod periodic_data;
mod quote_attribution;
mod quote_color;
mod quote_icon;
mod table_borders;
mod table_caption_foot {
    use super::periodic_data::element_table;
    include!("table_caption_foot.rs");
}
mod table_data_rows {
    use super::periodic_data::element_table;
    include!("table_data_rows.rs");
}
mod table_horizontal_scroll {
    use super::periodic_data::element_table;
    include!("table_horizontal_scroll.rs");
}
mod table_max_height {
    use super::periodic_data::element_table;
    include!("table_max_height.rs");
}
mod table_native_scroll {
    use super::periodic_data::element_table;
    include!("table_native_scroll.rs");
}
mod table_numbers_play;
mod table_scroll;
pub(crate) mod table_selection;
mod table_spacing;
mod table_sticky {
    use super::periodic_data::element_table;
    include!("table_sticky.rs");
}
mod table_structure;
mod table_vertical;
mod table_virtual;
mod text_clamp_play;
mod text_gradient_play;
mod text_inline;
mod text_metrics;
mod text_reactive;
mod text_truncate_play;
mod text_truncation;
mod text_wrap_play;
mod title_clamp_play;
mod title_levels;
mod title_presentation;
mod title_truncation;
mod title_wrap_play;
mod typography_article;
mod typography_scope;
mod typography_styles;
mod typography_table;
use super::showcase_examples::DetailSection;
pub fn sections(slug: &str, theme: sapid::Theme) -> Vec<DetailSection> {
    match slug {
        "blockquote" => vec![
            DetailSection {
                id: "quote-attribution",
                title: "Attribution and a real source link",
                note: "The quote and attribution are separate slots. Citation accepts text or an ordinary anchor; no source URL is guessed or fetched by Blockquote.",
                example: Box::new(quote_attribution::demo(theme.clone())),
                source: include_str!("quote_attribution.rs"),
            },
            DetailSection {
                id: "quote-icon",
                title: "An optional passive icon",
                note: "Place an existing finite vector icon beside the text in the content slot. The icon does not become a second quotation or an action.",
                example: Box::new(quote_icon::demo(theme.clone())),
                source: include_str!("quote_icon.rs"),
            },
            DetailSection {
                id: "quote-color",
                title: "Text wrap",
                note: "Compare normal line wrapping and balanced lines for the same quotation. Change the viewport width to compare the line endings.",
                example: Box::new(quote_color::demo(theme.clone())),
                source: include_str!("quote_color.rs"),
            },
        ],
        "code" => vec![
            DetailSection {
                id: "code-inline",
                title: "Inline code and ordinary text",
                note: "Code is semantic phrasing content. It can sit among inline text without introducing an editor, a syntax parser or a new interactive control.",
                example: Box::new(code_inline::demo(theme.clone())),
                source: include_str!("code_inline.rs"),
            },
            DetailSection {
                id: "code-block",
                title: "Block presentation and local horizontal scrolling",
                note: "Explicit block display and Pre whitespace preserve line breaks. Long lines scroll inside this code box instead of widening the document. This is ordinary code text; use CodeHighlight for syntax tokens.",
                example: Box::new(code_block::demo(theme.clone())),
                source: include_str!("code_block.rs"),
            },
            DetailSection {
                id: "code-colors",
                title: "Readable colors and syntax highlighting boundary",
                note: "Plain Code applies a readable theme foreground/background and escapes source text. Custom colors are explicit. Syntax highlighting is a separate component and page.",
                example: Box::new(code_colors::demo(theme.clone())),
                source: include_str!("code_colors.rs"),
            },
        ],
        "highlight" => vec![
            DetailSection {
                id: "highlight-whole-words",
                title: "Whole word matching",
                note: "Compare whole-word and substring matches for the same text.",
                example: Box::new(highlight_whole_words::demo(theme.clone())),
                source: include_str!("highlight_whole_words.rs"),
            },
            DetailSection {
                id: "highlight-accents",
                title: "Accent-sensitive matching",
                note: "Compare a literal match with a match that ignores combining accents.",
                example: Box::new(highlight_accents::demo(theme.clone())),
                source: include_str!("highlight_accents.rs"),
            },
            DetailSection {
                id: "highlight-per-term",
                title: "Custom colors per term",
                note: "Each matched term uses a distinct background.",
                example: Box::new(highlight_per_term::demo(theme.clone())),
                source: include_str!("highlight_per_term.rs"),
            },
            DetailSection {
                id: "highlight-mark-style",
                title: "Change highlight styles",
                note: "Compare default marks with an emphasized match.",
                example: Box::new(highlight_mark_style::demo(theme.clone())),
                source: include_str!("highlight_mark_style.rs"),
            },
            DetailSection {
                id: "highlight-text-props",
                title: "Text props",
                note: "The enclosing typography controls the size, weight and alignment of highlighted text.",
                example: Box::new(highlight_text_props::demo(theme.clone())),
                source: include_str!("highlight_text_props.rs"),
            },
            DetailSection {
                id: "highlight-multiple",
                title: "Multiple matches and overlapping queries",
                note: "Queries are finite literal strings. The earliest match wins; at the same start the longest query wins. Repeated matches become actual Mark ranges with the original text preserved.",
                example: Box::new(highlight_multiple::demo(theme.clone())),
                source: include_str!("highlight_multiple.rs"),
            },
            DetailSection {
                id: "highlight-case",
                title: "Exact case and UTF-8 matching",
                note: "Compare exact matching with ignore_ascii_case and whole_words. Both preserve the source spelling. Whole words use Unicode letters, numbers and underscores as word characters. Accented characters and Chinese text still match literally.",
                example: Box::new(highlight_case::demo(theme.clone())),
                source: include_str!("highlight_case.rs"),
            },
            DetailSection {
                id: "highlight-literals",
                title: "Literal punctuation and escaped source text",
                note: "Punctuation is matched literally. Marking <Button>, .* or [label] never treats source content as HTML or a regex, and the surrounding string is preserved.",
                example: Box::new(highlight_literals::demo(theme.clone())),
                source: include_str!("highlight_literals.rs"),
            },
            DetailSection {
                id: "highlight-empty",
                title: "No match and wrapping long prose",
                note: "No match leaves ordinary inline text, with no empty highlight control. Matching is mount-time and bounded to 64 KiB text and 32 queries; a changed search configuration belongs to a newly mounted content owner.",
                example: Box::new(highlight_empty::demo(theme.clone())),
                source: include_str!("highlight_empty.rs"),
            },
        ],
        "list" => vec![
            DetailSection {
                id: "list-start",
                title: "Start from specific number",
                note: "The visible ordinals follow the ordered-list settings.",
                example: Box::new(list_start::demo(theme.clone())),
                source: include_str!("list_start.rs"),
            },
            DetailSection {
                id: "list-reversed",
                title: "Reversed numbering",
                note: "The visible ordinals follow the ordered-list settings.",
                example: Box::new(list_reversed::demo(theme.clone())),
                source: include_str!("list_reversed.rs"),
            },
            DetailSection {
                id: "list-values",
                title: "Custom item values",
                note: "The visible ordinals follow the ordered-list settings.",
                example: Box::new(list_values::demo(theme.clone())),
                source: include_str!("list_values.rs"),
            },
            DetailSection {
                id: "list-order",
                title: "Ordered and unordered lists",
                note: "TextList uses real ordered/unordered document parts and ListItem membership. A display list does not acquire selection, a grid cursor or a virtual business source.",
                example: Box::new(list_order::demo(theme.clone())),
                source: include_str!("list_order.rs"),
            },
            DetailSection {
                id: "list-nested",
                title: "Nested lists and real list membership",
                note: "Nest TextList inside TextListItem. Each list retains its own bounded item collection and native list semantics; indentation follows ordinary document layout.",
                example: Box::new(list_nested::demo(theme.clone())),
                source: include_str!("list_nested.rs"),
            },
            DetailSection {
                id: "list-icons",
                title: "Passive icons beside text",
                note: "Finite vector icons can appear inside an item without turning it into a control. Native list markers are retained; the icon is content rather than an undocumented marker-replacement API.",
                example: Box::new(list_icons::demo(theme.clone())),
                source: include_str!("list_icons.rs"),
            },
            DetailSection {
                id: "list-spacing",
                title: "Item spacing and long wrapping content",
                note: "Item margins provide explicit row spacing. Multi-line text stays within the list indentation, with marker alignment supplied by the actual list layout.",
                example: Box::new(list_spacing::demo(theme.clone())),
                source: include_str!("list_spacing.rs"),
            },
        ],
        "mark" => vec![
            DetailSection {
                id: "mark-colors",
                title: "Theme colors and explicit alternatives",
                note: "Mark is passive inline text with a readable theme background and foreground. Custom backgrounds should set their corresponding foreground explicitly.",
                example: Box::new(mark_colors::demo(theme.clone())),
                source: include_str!("mark_colors.rs"),
            },
            DetailSection {
                id: "mark-wrapping",
                title: "Inline wrapping and selection",
                note: "Long marked content wraps as ordinary phrasing text. The complete original text stays in the document, so selection can span plain and marked fragments.",
                example: Box::new(mark_wrapping::demo(theme.clone())),
                source: include_str!("mark_wrapping.rs"),
            },
            DetailSection {
                id: "mark-highlight",
                title: "Explicit Mark and literal Highlight",
                note: "Use Mark when the application chooses the exact marked fragment. Highlight locates finite literal queries and emits the same semantic Mark kind; neither component edits or parses the source as HTML.",
                example: Box::new(mark_highlight::demo(theme.clone())),
                source: include_str!("mark_highlight.rs"),
            },
        ],
        "text" => vec![
            DetailSection {
                id: "text-gradient-play",
                title: "Gradient variant",
                note: "Choose two color stops and change the gradient direction.",
                example: Box::new(text_gradient_play::demo(theme.clone())),
                source: include_str!("text_gradient_play.rs"),
            },
            DetailSection {
                id: "text-truncate-play",
                title: "Truncate",
                note: "Choose which end of the line stays visible.",
                example: Box::new(text_truncate_play::demo(theme.clone())),
                source: include_str!("text_truncate_play.rs"),
            },
            DetailSection {
                id: "text-clamp-play",
                title: "Line clamp",
                note: "Limit the number of visible lines.",
                example: Box::new(text_clamp_play::demo(theme.clone())),
                source: include_str!("text_clamp_play.rs"),
            },
            DetailSection {
                id: "text-wrap-play",
                title: "Text wrap",
                note: "Compare normal and balanced line breaks.",
                example: Box::new(text_wrap_play::demo(theme.clone())),
                source: include_str!("text_wrap_play.rs"),
            },
            DetailSection {
                id: "text-metrics",
                title: "Font sizes, weights and colors",
                note: "Theme size controls typography metrics. Explicit font_size, line_height, weight and color override their own fields, leaving the remaining theme declarations intact.",
                example: Box::new(text_metrics::demo(theme.clone())),
                source: include_str!("text_metrics.rs"),
            },
            DetailSection {
                id: "text-truncation",
                title: "Single-line truncation and a line limit",
                note: "truncate uses a single-line ellipsis within an explicit width. line_clamp limits visible lines while keeping the complete semantic content; neither option rewrites the text source. Browser supported; the current native Apace profile explicitly rejects ellipsis and line clamp.",
                example: Box::new(text_truncation::demo(theme.clone())),
                source: include_str!("text_truncation.rs"),
            },
            DetailSection {
                id: "text-inline",
                title: "Inline text and container layout",
                note: "inline selects semantic phrasing content instead of a paragraph. Containers own flex, wrapping and alignment; Text controls typography and does not become an interactive link or button.",
                example: Box::new(text_inline::demo(theme.clone())),
                source: include_str!("text_inline.rs"),
            },
            DetailSection {
                id: "text-reactive",
                title: "Reactive content and explicit appearance",
                note: "Changing content keeps the same paragraph and updates its full semantic text. unstyled omits theme decoration while preserving explicit style fields.",
                example: Box::new(text_reactive::demo(theme.clone())),
                source: include_str!("text_reactive.rs"),
            },
        ],
        "title" => vec![
            DetailSection {
                id: "title-wrap-play",
                title: "Text wrap",
                note: "Compare normal and balanced line breaks.",
                example: Box::new(title_wrap_play::demo(theme.clone())),
                source: include_str!("title_wrap_play.rs"),
            },
            DetailSection {
                id: "title-clamp-play",
                title: "Line clamp",
                note: "Limit the number of visible lines.",
                example: Box::new(title_clamp_play::demo(theme.clone())),
                source: include_str!("title_clamp_play.rs"),
            },
            DetailSection {
                id: "title-levels",
                title: "Six semantic heading levels",
                note: "The constructor selects h1 through h6. Visual typography can be adjusted independently; a smaller-looking heading keeps its actual semantic level.",
                example: Box::new(title_levels::demo(theme.clone())),
                source: include_str!("title_levels.rs"),
            },
            DetailSection {
                id: "title-presentation",
                title: "Typography independent of heading level",
                note: "font_size, line_height and weight change presentation without changing the heading kind. Use levels for document structure and explicit metrics for visual hierarchy.",
                example: Box::new(title_presentation::demo(theme.clone())),
                source: include_str!("title_presentation.rs"),
            },
            DetailSection {
                id: "title-truncation",
                title: "Truncation preserves complete heading content",
                note: "Single-line truncation and a two-line limit change only the visible extent. The full original text and semantic heading level remain available to accessibility consumers. Browser supported; the current native Apace profile explicitly rejects ellipsis and line clamp.",
                example: Box::new(title_truncation::demo(theme.clone())),
                source: include_str!("title_truncation.rs"),
            },
        ],
        "table" => vec![
            DetailSection {
                id: "table-data-rows",
                title: "data prop",
                note: "Table rows are built from element data.",
                example: Box::new(table_data_rows::demo(theme.clone())),
                source: concat!(
                    include_str!("table_data_rows.rs"),
                    "\n",
                    include_str!("periodic_data.rs")
                ),
            },
            DetailSection {
                id: "table-caption-foot",
                title: "Caption and tfoot",
                note: "Column titles are repeated below the data.",
                example: Box::new(table_caption_foot::demo(theme.clone())),
                source: concat!(
                    include_str!("table_caption_foot.rs"),
                    "\n",
                    include_str!("periodic_data.rs")
                ),
            },
            DetailSection {
                id: "table-sticky",
                title: "Sticky header",
                note: "Scroll vertically to keep the column headings in view.",
                example: Box::new(table_sticky::demo(theme.clone())),
                source: concat!(
                    include_str!("table_sticky.rs"),
                    "\n",
                    include_str!("periodic_data.rs")
                ),
            },
            DetailSection {
                id: "table-horizontal-scroll",
                title: "Scroll container",
                note: "The table scrolls horizontally at narrow widths.",
                example: Box::new(table_horizontal_scroll::demo(theme.clone())),
                source: concat!(
                    include_str!("table_horizontal_scroll.rs"),
                    "\n",
                    include_str!("periodic_data.rs")
                ),
            },
            DetailSection {
                id: "table-native-scroll",
                title: "Native scroll container",
                note: "The browser provides the horizontal scrollbar.",
                example: Box::new(table_native_scroll::demo(theme.clone())),
                source: concat!(
                    include_str!("table_native_scroll.rs"),
                    "\n",
                    include_str!("periodic_data.rs")
                ),
            },
            DetailSection {
                id: "table-max-height",
                title: "Scroll container with max height",
                note: "The longer table scrolls within a 200px viewport.",
                example: Box::new(table_max_height::demo(theme.clone())),
                source: concat!(
                    include_str!("table_max_height.rs"),
                    "\n",
                    include_str!("periodic_data.rs")
                ),
            },
            DetailSection {
                id: "table-virtual",
                title: "Example: Table virtualization",
                note: "Scroll through 2,000 records while the table mounts a bounded window of rows.",
                example: Box::new(table_virtual::demo(theme.clone())),
                source: include_str!("table_virtual.rs"),
            },
            DetailSection {
                id: "table-numbers-play",
                title: "Tabular numbers",
                note: "Compare digit widths in the numeric column.",
                example: Box::new(table_numbers_play::demo(theme.clone())),
                source: include_str!("table_numbers_play.rs"),
            },
            DetailSection {
                id: "table-structure",
                title: "Caption, headers and footer",
                note: "Column and row headers retain actual associations. Caption and a separate footer describe the same table; cells are ordinary data rather than a selection grid.",
                example: Box::new(table_structure::demo(theme.clone())),
                source: include_str!("table_structure.rs"),
            },
            DetailSection {
                id: "table-scroll",
                title: "Local horizontal scrolling and sticky headings",
                note: "A bounded ScrollArea owns keyboard and pointer scrolling. The table keeps an explicit minimum width, so narrow screens scroll locally. Sticky headers use the same current table and an explicit offset. Both scroll axes are explicit; the native profile supports ordinary local scrolling but explicitly rejects Sticky.",
                example: Box::new(table_scroll::demo(theme.clone())),
                source: include_str!("table_scroll.rs"),
            },
            DetailSection {
                id: "table-spacing",
                title: "Compact and spacious cell spacing",
                note: "Horizontal and vertical spacing are independent finite pixel settings. The same table semantics and header relationships survive both presentations.",
                example: Box::new(table_spacing::demo(theme.clone())),
                source: include_str!("table_spacing.rs"),
            },
            DetailSection {
                id: "table-borders",
                title: "Striping, hover and independent borders",
                note: "Striping and hover are presentation choices. Table, row and column borders are independent; tabular numerals affect typography without inventing a selected-row model.",
                example: Box::new(table_borders::demo(theme.clone())),
                source: include_str!("table_borders.rs"),
            },
            DetailSection {
                id: "table-vertical",
                title: "Vertical key/value table",
                note: "Row header cells label their adjacent values. A vertical key/value presentation is still a real table and does not require selection or a data-grid controller.",
                example: Box::new(table_vertical::demo(theme.clone())),
                source: include_str!("table_vertical.rs"),
            },
            DetailSection {
                id: "table-selection",
                title: "Application-owned row selection",
                note: "Selection is an explicit Checkbox in a real data cell. The application owns the boolean, callback and feedback; passive Table does not silently add a grid cursor or selectable-row controller.",
                example: Box::new(table_selection::demo(theme.clone())),
                source: include_str!("table_selection.rs"),
            },
        ],
        "typography" => vec![
            DetailSection {
                id: "typography-styles",
                title: "All styles demo",
                note: "The same typography scope styles headings, paragraphs, lists and quotations.",
                example: Box::new(typography_styles::demo(theme.clone())),
                source: include_str!("typography_styles.rs"),
            },
            DetailSection {
                id: "typography-article",
                title: "An article with real document structure",
                note: "Headings, paragraphs, lists, a quotation and a descriptive link use ordinary semantic widgets. Typography supplies layout and an explicit builder context, without parsing arbitrary HTML.",
                example: Box::new(typography_article::demo(theme.clone())),
                source: include_str!("typography_article.rs"),
            },
            DetailSection {
                id: "typography-scope",
                title: "Explicit theme scope and default styling",
                note: "content receives TypographyContext for text/title/anchor builders. Other components explicitly receive cx.theme and cx.size. fill retains children as authored; it does not silently rewrite a nested widget theme or install global HTML styles.",
                example: Box::new(typography_scope::demo(theme.clone())),
                source: include_str!("typography_scope.rs"),
            },
            DetailSection {
                id: "typography-table",
                title: "Tabular details inside an article",
                note: "A real Table can appear among rich article blocks. The local ScrollArea owns horizontal overflow; the surrounding content keeps its normal wrapping width.",
                example: Box::new(typography_table::demo(theme.clone())),
                source: include_str!("typography_table.rs"),
            },
        ],
        _ => Vec::new(),
    }
}
pub fn navigation(slug: &str) -> Vec<(&'static str, &'static str)> {
    match slug {
        "blockquote" => vec![
            ("Attribution and a real source link", "#quote-attribution"),
            ("An optional passive icon", "#quote-icon"),
            ("Text wrap", "#quote-color"),
        ],
        "code" => vec![
            ("Inline code and ordinary text", "#code-inline"),
            (
                "Block presentation and local horizontal scrolling",
                "#code-block",
            ),
            (
                "Readable colors and syntax highlighting boundary",
                "#code-colors",
            ),
        ],
        "highlight" => vec![
            ("Whole word matching", "#highlight-whole-words"),
            ("Accent-sensitive matching", "#highlight-accents"),
            ("Custom colors per term", "#highlight-per-term"),
            ("Change highlight styles", "#highlight-mark-style"),
            ("Text props", "#highlight-text-props"),
            (
                "Multiple matches and overlapping queries",
                "#highlight-multiple",
            ),
            ("Exact case and UTF-8 matching", "#highlight-case"),
            (
                "Literal punctuation and escaped source text",
                "#highlight-literals",
            ),
            ("No match and wrapping long prose", "#highlight-empty"),
        ],
        "list" => vec![
            ("Start from specific number", "#list-start"),
            ("Reversed numbering", "#list-reversed"),
            ("Custom item values", "#list-values"),
            ("Ordered and unordered lists", "#list-order"),
            ("Nested lists and real list membership", "#list-nested"),
            ("Passive icons beside text", "#list-icons"),
            ("Item spacing and long wrapping content", "#list-spacing"),
        ],
        "mark" => vec![
            ("Theme colors and explicit alternatives", "#mark-colors"),
            ("Inline wrapping and selection", "#mark-wrapping"),
            ("Explicit Mark and literal Highlight", "#mark-highlight"),
        ],
        "text" => vec![
            ("Gradient variant", "#text-gradient-play"),
            ("Truncate", "#text-truncate-play"),
            ("Line clamp", "#text-clamp-play"),
            ("Text wrap", "#text-wrap-play"),
            ("Font sizes, weights and colors", "#text-metrics"),
            (
                "Single-line truncation and a line limit",
                "#text-truncation",
            ),
            ("Inline text and container layout", "#text-inline"),
            ("Reactive content and explicit appearance", "#text-reactive"),
        ],
        "title" => vec![
            ("Text wrap", "#title-wrap-play"),
            ("Line clamp", "#title-clamp-play"),
            ("Six semantic heading levels", "#title-levels"),
            (
                "Typography independent of heading level",
                "#title-presentation",
            ),
            (
                "Truncation preserves complete heading content",
                "#title-truncation",
            ),
        ],
        "table" => vec![
            ("data prop", "#table-data-rows"),
            ("Caption and tfoot", "#table-caption-foot"),
            ("Sticky header", "#table-sticky"),
            ("Scroll container", "#table-horizontal-scroll"),
            ("Native scroll container", "#table-native-scroll"),
            ("Scroll container with max height", "#table-max-height"),
            ("Example: Table virtualization", "#table-virtual"),
            ("Tabular numbers", "#table-numbers-play"),
            ("Caption, headers and footer", "#table-structure"),
            (
                "Local horizontal scrolling and sticky headings",
                "#table-scroll",
            ),
            ("Compact and spacious cell spacing", "#table-spacing"),
            ("Striping, hover and independent borders", "#table-borders"),
            ("Vertical key/value table", "#table-vertical"),
            ("Application-owned row selection", "#table-selection"),
        ],
        "typography" => vec![
            ("All styles demo", "#typography-styles"),
            (
                "An article with real document structure",
                "#typography-article",
            ),
            (
                "Explicit theme scope and default styling",
                "#typography-scope",
            ),
            ("Tabular details inside an article", "#typography-table"),
        ],
        _ => Vec::new(),
    }
}
