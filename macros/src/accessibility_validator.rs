#![allow(dead_code)]

use crate::token_parser::{AttributeValue, Element};
use proc_macro2::TokenStream;
use quote::quote_spanned;

/// Rule 1: Every <img> tag MUST have an alt attribute
pub fn validate_img_alt(elem: &Element) -> Option<TokenStream> {
    if elem.name != "img" {
        return None;
    }

    // Check if alt attribute exists
    let has_alt = elem.attrs.iter().any(|attr| attr.name == "alt");

    if !has_alt {
        Some(quote_spanned! { elem.span =>
            compile_error!("<img> is missing 'alt' attribute. Add alt=\"description\" or alt=\"\" for decorative images.");
        })
    } else {
        None
    }
}

/// Rule 2: The type attribute on <input> and <button> must be valid
pub fn validate_input_type(elem: &Element) -> Option<TokenStream> {
    // Only validate input and button elements
    if elem.name != "input" && elem.name != "button" {
        return None;
    }

    // Find type attribute
    let type_attr = elem.attrs.iter().find(|attr| attr.name == "type")?;

    // Only validate static values (not dynamic expressions)
    if let AttributeValue::Static(type_value) = &type_attr.value {
        let valid = match elem.name.as_str() {
            "input" => is_valid_input_type(type_value),
            "button" => is_valid_button_type(type_value),
            _ => return None,
        };

        if !valid {
            let suggestion = suggest_type_correction(type_value, elem.name == "input");
            let msg = format!(
                "Invalid <{}> type=\"{}\". {}",
                elem.name, type_value, suggestion
            );

            return Some(quote_spanned! { type_attr.span =>
                compile_error!(#msg);
            });
        }
    }

    None
}

/// Rule 3: ARIA role values must be valid
pub fn validate_aria_roles(elem: &Element) -> Option<TokenStream> {
    // Find role attribute
    let role_attr = elem.attrs.iter().find(|attr| attr.name == "role")?;

    // Only validate static values
    if let AttributeValue::Static(role_value) = &role_attr.value {
        if !is_valid_aria_role(role_value) {
            let msg = format!(
                "Invalid role=\"{}\". Must be a valid WAI-ARIA role. See: https://www.w3.org/TR/wai-aria-1.2/#role_definitions",
                role_value
            );

            return Some(quote_spanned! { role_attr.span =>
                compile_error!(#msg);
            });
        }
    }

    None
}

/// Rule 4: Buttons must have content OR aria-label OR title
pub fn validate_button_content(elem: &crate::token_parser::Element) -> Option<TokenStream> {
    if elem.name != "button" {
        return None;
    }

    // Check if button has text content (children)
    let has_content = !elem.children.is_empty();

    // Check if button has aria-label or title attribute
    let has_aria_label = elem.attrs.iter().any(|attr| attr.name == "aria-label");
    let has_title = elem.attrs.iter().any(|attr| attr.name == "title");

    if !has_content && !has_aria_label && !has_title {
        Some(quote_spanned! { elem.span =>
            compile_error!("<button> has no accessible text. Add text content, aria-label=\"...\", or title=\"...\".");
        })
    } else {
        None
    }
}

// Helper: Valid input types (HTML5)
fn is_valid_input_type(type_value: &str) -> bool {
    matches!(
        type_value,
        "text"
            | "password"
            | "email"
            | "number"
            | "tel"
            | "url"
            | "search"
            | "date"
            | "time"
            | "datetime-local"
            | "month"
            | "week"
            | "color"
            | "file"
            | "hidden"
            | "checkbox"
            | "radio"
            | "submit"
            | "reset"
            | "button"
            | "range"
            | "image"
    )
}

// Helper: Valid button types
fn is_valid_button_type(type_value: &str) -> bool {
    matches!(type_value, "button" | "submit" | "reset")
}

// Helper: Suggest corrections for common typos
fn suggest_type_correction(type_value: &str, is_input: bool) -> String {
    match type_value {
        "txt" => "Did you mean 'text'?".to_string(),
        "sumbit" => "Did you mean 'submit'?".to_string(),
        "buton" => "Did you mean 'button'?".to_string(),
        "num" => "Did you mean 'number'?".to_string(),
        "mail" => "Did you mean 'email'?".to_string(),
        _ => {
            if is_input {
                "Valid types: text, password, email, number, tel, url, search, date, time, checkbox, radio, submit, reset, button, etc.".to_string()
            } else {
                "Valid types: button, submit, reset".to_string()
            }
        }
    }
}

// Helper: Valid ARIA roles (WAI-ARIA 1.2)
fn is_valid_aria_role(role_value: &str) -> bool {
    matches!(
        role_value,
        // Widget roles
        "button" | "checkbox" | "gridcell" | "link" | "menuitem" | "menuitemcheckbox"
            | "menuitemradio" | "option" | "progressbar" | "radio" | "scrollbar"
            | "searchbox" | "slider" | "spinbutton" | "switch"
            | "tab" | "tabpanel" | "textbox" | "treeitem"
            // Composite widget roles
            | "combobox" | "grid" | "listbox" | "menu" | "menubar" | "radiogroup"
            | "tablist" | "tree" | "treegrid"
            // Document structure roles
            | "application" | "article" | "cell" | "columnheader" | "definition"
            | "directory" | "document" | "feed" | "figure" | "group" | "heading"
            | "img" | "list" | "listitem" | "math" | "none" | "note" | "presentation"
            | "row" | "rowgroup" | "rowheader" | "separator" | "table" | "term"
            | "toolbar" | "tooltip"
            // Landmark roles
            | "banner" | "complementary" | "contentinfo" | "form" | "main"
            | "navigation" | "region" | "search"
            // Live region roles
            | "alert" | "log" | "marquee" | "status" | "timer"
            // Window roles
            | "alertdialog" | "dialog"
    )
}

/// Rule 5: Links with target="_blank" must have rel="noopener" (security)
pub fn validate_anchor_target_blank(elem: &Element) -> Option<TokenStream> {
    if elem.name != "a" {
        return None;
    }

    let target_attr = elem.attrs.iter().find(|attr| attr.name == "target");

    if let Some(target) = target_attr {
        if let AttributeValue::Static(val) = &target.value {
            if val == "_blank" {
                // Check for rel attribute
                let rel_attr = elem.attrs.iter().find(|attr| attr.name == "rel");

                let has_noopener = if let Some(rel) = rel_attr {
                    if let AttributeValue::Static(rel_val) = &rel.value {
                        rel_val.contains("noopener") || rel_val.contains("noreferrer")
                    } else {
                        // Dynamic rel attribute - assume it's handled or warn?
                        // For now, let's be strict about static values.
                        false
                    }
                } else {
                    false
                };

                if !has_noopener {
                    return Some(quote_spanned! { target.span =>
                        compile_error!("Security Risk: Links with target=\"_blank\" must have rel=\"noopener\" or rel=\"noreferrer\" to prevent Reverse Tabnabbing attacks.");
                    });
                }
            }
        }
    }

    None
}

/// Rule 6: <iframe> must have a title attribute
pub fn validate_iframe_title(
    elem: &crate::token_parser::Element,
) -> Option<proc_macro2::TokenStream> {
    if elem.name != "iframe" {
        return None;
    }

    let has_title = elem.attrs.iter().any(|attr| attr.name == "title");

    if !has_title {
        Some(quote_spanned! { elem.span =>
            compile_error!("<iframe> tags must have a 'title' attribute for accessibility.");
        })
    } else {
        None
    }
}
