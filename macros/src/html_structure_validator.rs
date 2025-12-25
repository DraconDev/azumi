#![allow(dead_code)]

use crate::token_parser::Element;
use crate::token_parser::Node;
use proc_macro2::TokenStream;
use quote::quote_spanned;

/// Rule 10: Component Structure Enforcement
/// Enforces Script -> Content -> Style order
pub fn validate_node_order(nodes: &[Node]) -> Vec<TokenStream> {
    let mut errors = vec![];

    #[derive(PartialEq, PartialOrd, Copy, Clone)]
    enum Phase {
        Script,
        Content,
        Style,
    }

    let mut phase = Phase::Script;

    for node in nodes {
        match node {
            // Script Handling
            Node::Element(elem) if elem.name == "script" => {
                if phase > Phase::Script {
                    let msg = "Order Error: <script> tags must be placed at the top of the component, before any HTML content.";
                    errors.push(quote_spanned! { elem.full_span =>
                        compile_error!(#msg);
                    });
                }
                // Allowed in Script Phase, stays in Script Phase
            }
            // Style Handling
            Node::Element(elem) if elem.name == "style" => {
                // <style> element (e.g., inline or src) - treat as Style phase
                phase = Phase::Style;
            }
            Node::Block(crate::token_parser::Block::Style(_)) => {
                // style! block - treat as Style phase
                phase = Phase::Style;
            }
            // Comments - Ignored, do not change phase
            Node::Comment(_) => {}
            // All other content (HTML elements, text, blocks, etc.)
            _ => {
                if phase == Phase::Style {
                    // Start of content span is tricky without a specific node match,
                    // but we can match individual types if needed or just use a generic span if available.
                    // For now, let's try to get a span from the node if possible
                    let span = match node {
                        Node::Element(e) => e.full_span,
                        Node::Text(t) => t.span,
                        Node::Expression(e) => e.span,
                        Node::Doctype(d) => d.span,
                        Node::Fragment(f) => f.span,
                        Node::Block(b) => match b {
                            crate::token_parser::Block::If(i) => i.span,
                            crate::token_parser::Block::For(f) => f.span,
                            crate::token_parser::Block::Match(m) => m.span,
                            crate::token_parser::Block::Call(c) => c.span,
                            crate::token_parser::Block::Component(c) => c.span,
                            crate::token_parser::Block::Let(l) => l.span,
                            _ => proc_macro2::Span::call_site(), // Should match matches above
                        },
                        _ => proc_macro2::Span::call_site(),
                    };

                    let msg = "Order Error: HTML structure and logic must be placed BEFORE Style blocks. Move this content above the <style> block.";
                    errors.push(quote_spanned! { span =>
                        compile_error!(#msg);
                    });
                } else {
                    // Move to Content phase if we were in Script phase
                    if phase == Phase::Script {
                        phase = Phase::Content;
                    }
                }
            }
        }
    }

    errors
}

/// Rule 1: Tables can only contain specific children
pub fn validate_table_children(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "table" {
        return vec![];
    }

    let valid_table_children = [
        "caption", "colgroup", "thead", "tbody", "tfoot", "tr", "style", "script", "template",
    ];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if !valid_table_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <table>. Tables can only contain: caption, colgroup, thead, tbody, tfoot, tr, style, script, template. Browser will hoist this element outside the table.",
                    child_elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 2: Lists can only contain <li>, script, or template
pub fn validate_list_children(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "ul" && elem.name != "ol" {
        return vec![];
    }

    let valid_list_children = ["li", "script", "template"];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if !valid_list_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <{}>. Lists can only contain <li>, script, or template. This breaks accessibility - screen readers will report 0 items.",
                    child_elem.name, elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 3: Forms cannot be nested
pub fn validate_nested_forms(elem: &Element, is_inside_form: bool) -> Vec<TokenStream> {
    let mut errors = vec![];

    if elem.name == "form" && is_inside_form {
        let msg = "Nested <form> is not allowed. Browsers will delete nested forms, breaking submit logic.";
        errors.push(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    errors
}

/// Rule 4: Buttons cannot contain interactive elements
pub fn validate_button_interactive(elem: &Element, is_inside_button: bool) -> Vec<TokenStream> {
    let mut errors = vec![];

    let interactive_elements = ["a", "button", "input", "select", "textarea", "label"];

    if is_inside_button && interactive_elements.contains(&elem.name.as_str()) {
        let msg = format!(
            "Invalid <{}> inside <button>. Buttons cannot contain interactive elements (a, button, input, select, textarea, label). This creates undefined click behavior.",
            elem.name
        );
        errors.push(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    errors
}

/// Rule 5: Paragraphs cannot contain block-level elements
pub fn validate_paragraph_content(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "p" {
        return vec![];
    }

    // List of block-level elements that cannot be inside a <p>
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p
    let invalid_p_children = [
        "address",
        "article",
        "aside",
        "blockquote",
        "details",
        "div",
        "dl",
        "fieldset",
        "figcaption",
        "figure",
        "footer",
        "form",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "header",
        "hgroup",
        "hr",
        "main",
        "menu",
        "nav",
        "ol",
        "p",
        "pre",
        "section",
        "table",
        "ul",
    ];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if invalid_p_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <p>. Paragraphs cannot contain block-level elements. Browsers will automatically close the <p> tag before this element, breaking your layout.",
                    child_elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 6: Anchors cannot contain other anchors or interactive content
pub fn validate_anchor_nesting(elem: &Element, is_inside_anchor: bool) -> Vec<TokenStream> {
    let mut errors = vec![];

    if elem.name == "a" && is_inside_anchor {
        let msg = "Nested <a> tags are forbidden. Links cannot contain other links.";
        errors.push(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    errors
}

/// Rule 7: Headings cannot contain other headings or block-level content
pub fn validate_heading_content(elem: &Element) -> Vec<TokenStream> {
    let headings = ["h1", "h2", "h3", "h4", "h5", "h6"];
    if !headings.contains(&elem.name.as_str()) {
        return vec![];
    }

    let mut errors = vec![];

    // Headings can only contain phrasing content.
    // We'll check for common block-level elements that are definitely wrong.
    let invalid_heading_children = [
        "div",
        "p",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "ul",
        "ol",
        "li",
        "blockquote",
        "form",
        "table",
        "header",
        "footer",
        "main",
        "section",
        "article",
        "aside",
        "nav",
    ];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if invalid_heading_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <{}>. Headings can only contain phrasing content (text, span, em, strong, etc.), not block-level elements.",
                    child_elem.name, elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 8: Tag names must be valid HTML5 tags or custom elements (with dashes)
pub fn validate_tag_name(elem: &Element) -> Option<TokenStream> {
    let name = &elem.name;

    // Allow custom elements (must contain a dash)
    if name.contains('-') {
        return None;
    }

    // Allow standard HTML5 tags
    // Source: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
    let valid_tags = [
        "a",
        "abbr",
        "address",
        "area",
        "article",
        "aside",
        "audio",
        "b",
        "base",
        "bdi",
        "bdo",
        "blockquote",
        "body",
        "br",
        "button",
        "canvas",
        "caption",
        "cite",
        "code",
        "col",
        "colgroup",
        "data",
        "datalist",
        "dd",
        "del",
        "details",
        "dfn",
        "dialog",
        "div",
        "dl",
        "dt",
        "em",
        "embed",
        "fieldset",
        "figcaption",
        "figure",
        "footer",
        "form",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "head",
        "header",
        "hgroup",
        "hr",
        "html",
        "i",
        "iframe",
        "img",
        "input",
        "ins",
        "kbd",
        "label",
        "legend",
        "li",
        "link",
        "main",
        "map",
        "mark",
        "menu",
        "meta",
        "meter",
        "nav",
        "noscript",
        "object",
        "ol",
        "optgroup",
        "option",
        "output",
        "p",
        "picture",
        "pre",
        "progress",
        "q",
        "rp",
        "rt",
        "ruby",
        "s",
        "samp",
        "script",
        "search",
        "section",
        "select",
        "slot",
        "small",
        "source",
        "span",
        "strong",
        "style",
        "sub",
        "summary",
        "sup",
        "svg",
        "table",
        "tbody",
        "td",
        "template",
        "textarea",
        "tfoot",
        "th",
        "thead",
        "time",
        "title",
        "tr",
        "track",
        "u",
        "ul",
        "var",
        "video",
        "wbr",
        // SVG tags (common ones)
        "path",
        "circle",
        "rect",
        "line",
        "polyline",
        "polygon",
        "text",
        "g",
        "defs",
        "symbol",
        "use",
        "image",
        "clipPath",
        "mask",
        "pattern",
        "linearGradient",
        "radialGradient",
        "stop",
        "animate",
        "animateTransform",
        "mpath",
        "set",
        // MathML (basic)
        "math",
        "mi",
        "mn",
        "mo",
        "ms",
        "mtext",
        "mrow",
        "frac",
        "sqrt",
        "root",
        "table",
        "tr",
        "td",
    ];

    if !valid_tags.contains(&name.as_str()) {
        let msg = format!(
            "Unknown tag <{}>. If this is a custom element, it must contain a dash (e.g., <my-component>). If it's a standard HTML tag, check for typos.",
            name
        );
        return Some(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    None
}

/// Rule 9: Attribute names must be valid HTML attributes, data-*, aria-*, or event handlers
pub fn validate_attribute_name(attr: &crate::token_parser::Attribute) -> Option<TokenStream> {
    let name = &attr.name;

    // 1. Allow ANY attribute containing a hyphen
    // This covers data-*, aria-*, hx-*, x-*, and any future library using hyphenated attributes.
    if name.contains('-') {
        return None;
    }

    // 2. Allow event handlers (on*)
    if name.starts_with("on") {
        return None;
    }

    // 3. Allow XML namespaces (xmlns) - though usually contains colon or is just xmlns
    if name == "xmlns" || name.contains(':') {
        return None;
    }

    // 4. Standard HTML Global Attributes
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
    let global_attributes = [
        "accesskey",
        "autocapitalize",
        "autofocus",
        "class",
        "contenteditable",
        "dir",
        "draggable",
        "enterkeyhint",
        "hidden",
        "id",
        "inputmode",
        "is",
        "itemid",
        "itemprop",
        "itemref",
        "itemscope",
        "itemtype",
        "lang",
        "nonce",
        "part",
        "popover",
        "role",
        "slot",
        "spellcheck",
        "style",
        "tabindex",
        "title",
        "translate",
        "virtualkeyboardpolicy",
    ];

    if global_attributes.contains(&name.as_str()) {
        return None;
    }

    // 9. Specific Element Attributes (Common ones)
    // This is a large list, but essential for strictness.
    // We'll include common attributes for all standard tags.
    let common_attributes = [
        "accept",
        "accept-charset",
        "action",
        "align",
        "allow",
        "alt",
        "async",
        "autocomplete",
        "autoplay",
        "background",
        "bgcolor",
        "border",
        "capture",
        "charset",
        "checked",
        "cite",
        "color",
        "cols",
        "colspan",
        "content",
        "controls",
        "coords",
        "crossorigin",
        "datetime",
        "decoding",
        "default",
        "defer",
        "dirname",
        "disabled",
        "download",
        "enctype",
        "for",
        "form",
        "formaction",
        "formenctype",
        "formmethod",
        "formnovalidate",
        "formtarget",
        "headers",
        "height",
        "high",
        "href",
        "hreflang",
        "http-equiv",
        "integrity",
        "kind",
        "label",
        "list",
        "loading",
        "loop",
        "low",
        "max",
        "maxlength",
        "media",
        "method",
        "min",
        "minlength",
        "multiple",
        "muted",
        "name",
        "novalidate",
        "open",
        "optimum",
        "pattern",
        "placeholder",
        "playsinline",
        "poster",
        "preload",
        "readonly",
        "referrerpolicy",
        "rel",
        "required",
        "reversed",
        "rows",
        "rowspan",
        "sandbox",
        "scope",
        "selected",
        "shape",
        "size",
        "sizes",
        "span",
        "src",
        "srcdoc",
        "srclang",
        "srcset",
        "start",
        "step",
        "target",
        "type",
        "usemap",
        "value",
        "width",
        "wrap",
        // SVG specific (common)
        "viewBox",
        "d",
        "fill",
        "stroke",
        "stroke-width",
        "stroke-linecap",
        "stroke-linejoin",
        "transform",
        "opacity",
        "points",
        "cx",
        "cy",
        "r",
        "rx",
        "ry",
        "x",
        "y",
        "x1",
        "y1",
        "x2",
        "y2",
        "preserveAspectRatio",
    ];

    if common_attributes.contains(&name.as_str()) {
        return None;
    }

    // If we get here, it's an unknown attribute
    let msg = format!(
        "Unknown attribute '{}'. Check for typos. If this is a custom attribute, prefix it with 'data-' (e.g., data-{}).",
        name, name
    );

    Some(quote_spanned! { attr.span =>
        compile_error!(#msg);
    })
}
