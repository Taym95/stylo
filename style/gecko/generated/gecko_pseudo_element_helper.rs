/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Autogenerated file created by components/style/binding_tools/regen_atoms.py, DO NOT EDIT DIRECTLY */

/*
 * This file contains a helper macro invocation to aid Gecko's style system
 * pseudo-element integration.
 *
 * This file is NOT INTENDED to be compiled as a standalone module.
 *
 * Also, it guarantees the property that normal pseudo-elements are processed
 * before anonymous boxes.
 *
 * Expected usage is as follows:
 *
 * ```
 * fn have_to_use_pseudo_elements() {
 *     macro_rules pseudo_element! {
 *         ($pseudo_str_with_colon:expr, $pseudo_atom:expr, $is_anon_box:true) => {{
 *             // Stuff stuff stuff.
 *         }}
 *     }
 *     include!("path/to/helper.rs")
 * }
 * ```
 *
 */
{
    pseudo_element!(":after",
                    atom!(":after"),
                    false);
    pseudo_element!(":before",
                    atom!(":before"),
                    false);
    pseudo_element!(":backdrop",
                    atom!(":backdrop"),
                    false);
    pseudo_element!(":cue",
                    atom!(":cue"),
                    false);
    pseudo_element!(":first-letter",
                    atom!(":first-letter"),
                    false);
    pseudo_element!(":first-line",
                    atom!(":first-line"),
                    false);
    pseudo_element!(":-moz-selection",
                    atom!(":-moz-selection"),
                    false);
    pseudo_element!(":-moz-focus-inner",
                    atom!(":-moz-focus-inner"),
                    false);
    pseudo_element!(":-moz-focus-outer",
                    atom!(":-moz-focus-outer"),
                    false);
    pseudo_element!(":-moz-list-bullet",
                    atom!(":-moz-list-bullet"),
                    false);
    pseudo_element!(":-moz-list-number",
                    atom!(":-moz-list-number"),
                    false);
    pseudo_element!(":-moz-math-anonymous",
                    atom!(":-moz-math-anonymous"),
                    false);
    pseudo_element!(":-moz-number-wrapper",
                    atom!(":-moz-number-wrapper"),
                    false);
    pseudo_element!(":-moz-number-text",
                    atom!(":-moz-number-text"),
                    false);
    pseudo_element!(":-moz-number-spin-box",
                    atom!(":-moz-number-spin-box"),
                    false);
    pseudo_element!(":-moz-number-spin-up",
                    atom!(":-moz-number-spin-up"),
                    false);
    pseudo_element!(":-moz-number-spin-down",
                    atom!(":-moz-number-spin-down"),
                    false);
    pseudo_element!(":-moz-progress-bar",
                    atom!(":-moz-progress-bar"),
                    false);
    pseudo_element!(":-moz-range-track",
                    atom!(":-moz-range-track"),
                    false);
    pseudo_element!(":-moz-range-progress",
                    atom!(":-moz-range-progress"),
                    false);
    pseudo_element!(":-moz-range-thumb",
                    atom!(":-moz-range-thumb"),
                    false);
    pseudo_element!(":-moz-meter-bar",
                    atom!(":-moz-meter-bar"),
                    false);
    pseudo_element!(":-moz-placeholder",
                    atom!(":-moz-placeholder"),
                    false);
    pseudo_element!(":placeholder",
                    atom!(":placeholder"),
                    false);
    pseudo_element!(":-moz-color-swatch",
                    atom!(":-moz-color-swatch"),
                    false);
    pseudo_element!(":-moz-text",
                    atom!(":-moz-text"),
                    true);
    pseudo_element!(":-moz-oof-placeholder",
                    atom!(":-moz-oof-placeholder"),
                    true);
    pseudo_element!(":-moz-first-letter-continuation",
                    atom!(":-moz-first-letter-continuation"),
                    true);
    pseudo_element!(":-moz-block-inside-inline-wrapper",
                    atom!(":-moz-block-inside-inline-wrapper"),
                    true);
    pseudo_element!(":-moz-mathml-anonymous-block",
                    atom!(":-moz-mathml-anonymous-block"),
                    true);
    pseudo_element!(":-moz-xul-anonymous-block",
                    atom!(":-moz-xul-anonymous-block"),
                    true);
    pseudo_element!(":-moz-hframeset-border",
                    atom!(":-moz-hframeset-border"),
                    true);
    pseudo_element!(":-moz-vframeset-border",
                    atom!(":-moz-vframeset-border"),
                    true);
    pseudo_element!(":-moz-line-frame",
                    atom!(":-moz-line-frame"),
                    true);
    pseudo_element!(":-moz-button-content",
                    atom!(":-moz-button-content"),
                    true);
    pseudo_element!(":-moz-cell-content",
                    atom!(":-moz-cell-content"),
                    true);
    pseudo_element!(":-moz-dropdown-list",
                    atom!(":-moz-dropdown-list"),
                    true);
    pseudo_element!(":-moz-fieldset-content",
                    atom!(":-moz-fieldset-content"),
                    true);
    pseudo_element!(":-moz-frameset-blank",
                    atom!(":-moz-frameset-blank"),
                    true);
    pseudo_element!(":-moz-display-comboboxcontrol-frame",
                    atom!(":-moz-display-comboboxcontrol-frame"),
                    true);
    pseudo_element!(":-moz-html-canvas-content",
                    atom!(":-moz-html-canvas-content"),
                    true);
    pseudo_element!(":-moz-inline-table",
                    atom!(":-moz-inline-table"),
                    true);
    pseudo_element!(":-moz-table",
                    atom!(":-moz-table"),
                    true);
    pseudo_element!(":-moz-table-cell",
                    atom!(":-moz-table-cell"),
                    true);
    pseudo_element!(":-moz-table-column-group",
                    atom!(":-moz-table-column-group"),
                    true);
    pseudo_element!(":-moz-table-column",
                    atom!(":-moz-table-column"),
                    true);
    pseudo_element!(":-moz-table-wrapper",
                    atom!(":-moz-table-wrapper"),
                    true);
    pseudo_element!(":-moz-table-row-group",
                    atom!(":-moz-table-row-group"),
                    true);
    pseudo_element!(":-moz-table-row",
                    atom!(":-moz-table-row"),
                    true);
    pseudo_element!(":-moz-canvas",
                    atom!(":-moz-canvas"),
                    true);
    pseudo_element!(":-moz-pagebreak",
                    atom!(":-moz-pagebreak"),
                    true);
    pseudo_element!(":-moz-page",
                    atom!(":-moz-page"),
                    true);
    pseudo_element!(":-moz-pagecontent",
                    atom!(":-moz-pagecontent"),
                    true);
    pseudo_element!(":-moz-page-sequence",
                    atom!(":-moz-page-sequence"),
                    true);
    pseudo_element!(":-moz-scrolled-content",
                    atom!(":-moz-scrolled-content"),
                    true);
    pseudo_element!(":-moz-scrolled-canvas",
                    atom!(":-moz-scrolled-canvas"),
                    true);
    pseudo_element!(":-moz-scrolled-page-sequence",
                    atom!(":-moz-scrolled-page-sequence"),
                    true);
    pseudo_element!(":-moz-column-content",
                    atom!(":-moz-column-content"),
                    true);
    pseudo_element!(":-moz-viewport",
                    atom!(":-moz-viewport"),
                    true);
    pseudo_element!(":-moz-viewport-scroll",
                    atom!(":-moz-viewport-scroll"),
                    true);
    pseudo_element!(":-moz-anonymous-flex-item",
                    atom!(":-moz-anonymous-flex-item"),
                    true);
    pseudo_element!(":-moz-anonymous-grid-item",
                    atom!(":-moz-anonymous-grid-item"),
                    true);
    pseudo_element!(":-moz-ruby",
                    atom!(":-moz-ruby"),
                    true);
    pseudo_element!(":-moz-ruby-base",
                    atom!(":-moz-ruby-base"),
                    true);
    pseudo_element!(":-moz-ruby-base-container",
                    atom!(":-moz-ruby-base-container"),
                    true);
    pseudo_element!(":-moz-ruby-text",
                    atom!(":-moz-ruby-text"),
                    true);
    pseudo_element!(":-moz-ruby-text-container",
                    atom!(":-moz-ruby-text-container"),
                    true);
    pseudo_element!(":-moz-tree-column",
                    atom!(":-moz-tree-column"),
                    true);
    pseudo_element!(":-moz-tree-row",
                    atom!(":-moz-tree-row"),
                    true);
    pseudo_element!(":-moz-tree-separator",
                    atom!(":-moz-tree-separator"),
                    true);
    pseudo_element!(":-moz-tree-cell",
                    atom!(":-moz-tree-cell"),
                    true);
    pseudo_element!(":-moz-tree-indentation",
                    atom!(":-moz-tree-indentation"),
                    true);
    pseudo_element!(":-moz-tree-line",
                    atom!(":-moz-tree-line"),
                    true);
    pseudo_element!(":-moz-tree-twisty",
                    atom!(":-moz-tree-twisty"),
                    true);
    pseudo_element!(":-moz-tree-image",
                    atom!(":-moz-tree-image"),
                    true);
    pseudo_element!(":-moz-tree-cell-text",
                    atom!(":-moz-tree-cell-text"),
                    true);
    pseudo_element!(":-moz-tree-checkbox",
                    atom!(":-moz-tree-checkbox"),
                    true);
    pseudo_element!(":-moz-tree-progressmeter",
                    atom!(":-moz-tree-progressmeter"),
                    true);
    pseudo_element!(":-moz-tree-drop-feedback",
                    atom!(":-moz-tree-drop-feedback"),
                    true);
    pseudo_element!(":-moz-svg-marker-anon-child",
                    atom!(":-moz-svg-marker-anon-child"),
                    true);
    pseudo_element!(":-moz-svg-outer-svg-anon-child",
                    atom!(":-moz-svg-outer-svg-anon-child"),
                    true);
    pseudo_element!(":-moz-svg-foreign-content",
                    atom!(":-moz-svg-foreign-content"),
                    true);
    pseudo_element!(":-moz-svg-text",
                    atom!(":-moz-svg-text"),
                    true);
}
