# Measured Spoiler

`spoiler(key).content(text).max_height(120.0)` clips selectable text and an optional image, and
provides a real named disclosure button outside the clipped viewport. It accepts
reactive text up to 64KiB and positive finite mount-time height. Text remains in
one original paragraph/text node during expansion; it is not rewritten into a
truncated business value. `.illustration(image)` places a Sapid Image before the text inside the measured body. Image load and resize use the same height observer. The content API accepts passive text and images so clipped controls cannot remain in the Tab sequence.

Presentation settles immediately with no expansion animation, including reduced
motion. Short content hides the action; changed content remeasures the current
viewport. `data-display-pages.md` describes the article, threshold, custom-label,
controlled refusal and content-change public examples.

`expanded` is controlled; `default_expanded` supplies local state only without a
source. `on_expanded_change` proposes changes before generic activation observers.
External source changes remain silent. `labels(more,less)` requires nonempty
single-line names. Common theme/size/class/style/unstyled/name/enabled/focus/ref
methods apply to the root. `viewport_ref` and `trigger_ref` identify stable parts.
Unstyled preserves structural clipping and explicit overrides.

A boxed inner view observes real content height. Overflow determines whether the
action shell is visible; content changes, font/layout changes and width changes
use the same Resize protocol. Before the first measurement SSR keeps an action
for nonempty text or an illustration; it cannot predict pixels. If content shrinks while the action
is focused, the component requests focus on the stable programmatic text box
before hiding its action. There are no timers, polling loops or layout reads in
reactive patches. The mounted image and text retain their identities across disclosure changes.

Invalid source updates preserve the last accepted paragraph and can recover.
Owner cleanup retires handlers, bindings and references; late events cannot
resurrect them. `tests/spoiler.rs` verifies state, callback order, measurements,
controlled rejection, bounds/invalid input, text recovery, focus intent, SSR
escaping and retirement. `spoiler-acceptance.js` and the native fixture verify
actual geometry, element/text identity, keyboard/pointer, focus after shrink,
four themes and teardown. Actual passed host runs are recorded separately in the
D-drive reports, not inferred from test-file existence.

On 2026-10-24 the focused Chromium 148.0.7778.96 run passed all eight four-theme
CSR/hydration cases with errors=[] (`D:/BuildCache/sapid/spoiler-browser.json`).
The native four-theme geometry/identity test passed in `spoiler-native.log`.
