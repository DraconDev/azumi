// Force rebuild 3
mod component;

mod accessibility_validator;
mod action;
mod css;
mod css_validator;
mod head;
mod html_structure_validator;
mod live;
#[cfg(feature = "schema")]
mod schema;
mod style;
mod test_spacing;
mod token_parser;
mod asset_rewriter;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::spanned::Spanned;

#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
    head::expand_head(input)
}

#[cfg(feature = "schema")]
#[proc_macro_derive(Schema, attributes(schema))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    schema::derive_schema(input)
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    component::expand_component(item)
}

#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    action::expand_action(item)
}

/// Azumi Live - Compiler-driven optimistic UI
///
/// Marks a struct as a live component state. The macro:
/// - Adds Serialize/Deserialize derives
/// - Generates `to_scope()` helper method
///
/// Use with `#[azumi::live_impl]` on the impl block to enable
/// automatic prediction generation.
#[proc_macro_attribute]
pub fn live(attr: TokenStream, item: TokenStream) -> TokenStream {
    live::expand_live(attr, item)
}

/// Azumi Live Impl - Method analysis for optimistic UI
///
/// Analyzes methods in an impl block for predictable mutations:
/// - `self.field = literal` → SetLiteral prediction
/// - `self.field = !self.field` → Toggle prediction  
/// - `self.field += value` → Add prediction
/// - `self.field -= value` → Sub prediction
///
/// Generates Axum action handlers automatically.
#[proc_macro_attribute]
pub fn live_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    live::expand_live_impl(attr, item)
}

struct NodesWrapper(Vec<token_parser::Node>);

impl Parse for NodesWrapper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        token_parser::parse_nodes(input).map(NodesWrapper)
    }
}

/// Validates that a style attribute only contains CSS custom properties (--variables).
/// Returns Ok(()) if valid, Err(error_message) if invalid.
fn validate_style_only_css_vars(style_value: &str) -> Result<(), String> {
    // Parse style value: "prop: value; prop2: value2"
    for part in style_value.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        
        // Split by first ':'
        if let Some(colon_pos) = part.find(':') {
            let prop_name = part[..colon_pos].trim();
            
            // Property must start with --
            if !prop_name.starts_with("--") {
                return Err(format!(
                    "Invalid style property '{}'. Only CSS variables are allowed in inline styles.\n\
                    \n\
                    Allowed:\n\
                    - CSS Variables: style={{ --custom-color: \"red\" }}\n\
                    - CSS Variables (string): style=\"--custom-color: red\"\n\
                    \n\
                    For standard properties like '{}', please use CSS classes in the <style> block.",
                    prop_name, prop_name
                ));
            }
        }
    }
    Ok(())
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as token_parser::HtmlInput);
    let mut nodes = input.nodes;

    // Auto-scope Asset Paths (Rewrites /img/logo.png -> /assets/logo.a8b9.png)
    asset_rewriter::rewrite_nodes(&mut nodes);

    // 1. Process styles (hoist <style> tags)
    let (style_bindings, scoped_css, global_css) = process_styles(&nodes);

    // 2. CSS dependencies are no longer collected for external files
    let css_deps: Vec<proc_macro2::TokenStream> = Vec::new();

    // 3. Generate HTML string construction code
    let html_construction = generate_nodes(&nodes);

    // 4. Generate bind validation checks
    let mut validation_checks = Vec::new();
    collect_bind_checks(&nodes, &mut validation_checks);

    let expanded = quote! {
        {
            // Inject style bindings (hoisted)
            #style_bindings

            // CSS dependency tracking (forces recompile when CSS changes)
            #(#css_deps)*

            // Validation block (compile-time only)
            const _: () = {
                #(#validation_checks)*
            };

            // Runtime HTML generation
            azumi::from_fn(move |f| {
                // Inject global CSS first (unscoped)
                // CSS Injection is now handled by generated code (smart injection into <head>)
                if false {
                   // Legacy code removed
                }
                #html_construction
            })
        }
    };

    TokenStream::from(expanded)
}

fn extract_styles(nodes: Vec<token_parser::Node>) -> (Vec<token_parser::StyleBlock>, Vec<token_parser::Node>) {
    let mut styles = Vec::new();
    let mut other_nodes = Vec::new();

    for node in nodes {
        match node {
            token_parser::Node::Block(token_parser::Block::Style(style)) => {
                styles.push(style);
            }
            _ => other_nodes.push(node),
        }
    }

    (styles, other_nodes)
}

fn process_styles(nodes: &[token_parser::Node]) -> (proc_macro2::TokenStream, String, String) {
    let mut bindings = proc_macro2::TokenStream::new();
    let mut scoped_css = String::new();
    let mut global_css = String::new();



    for node in nodes {
        match node {
            token_parser::Node::Block(token_parser::Block::Style(style_block)) => {

                if style_block.is_global {
                    // Global styles: validate but don't scope, but DO generate bindings
                    let output = style::process_global_style_macro(style_block.content.clone());
                    bindings.extend(output.bindings);
                    global_css.push_str(&output.css);
                } else {
                    // Scoped styles: normal processing
                    let output = style::process_style_macro(style_block.content.clone());
                    bindings.extend(output.bindings);
                    scoped_css.push_str(&output.css);
                }
            }
            token_parser::Node::Element(elem) => {
                let (child_bindings, child_scoped, child_global) = process_styles(&elem.children);
                bindings.extend(child_bindings);
                scoped_css.push_str(&child_scoped);
                global_css.push_str(&child_global);
            }
            token_parser::Node::Fragment(frag) => {
                let (child_bindings, child_scoped, child_global) = process_styles(&frag.children);
                bindings.extend(child_bindings);
                scoped_css.push_str(&child_scoped);
                global_css.push_str(&child_global);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    let (b, s, g) = process_styles(&if_block.then_branch);
                    bindings.extend(b);
                    scoped_css.push_str(&s);
                    global_css.push_str(&g);
                    if let Some(else_branch) = &if_block.else_branch {
                        let (b, s, g) = process_styles(else_branch);
                        bindings.extend(b);
                        scoped_css.push_str(&s);
                        global_css.push_str(&g);
                    }
                }
                token_parser::Block::For(for_block) => {
                    let (b, s, g) = process_styles(&for_block.body);
                    bindings.extend(b);
                    scoped_css.push_str(&s);
                    global_css.push_str(&g);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        let (b, s, g) = process_styles(&arm.body);
                        bindings.extend(b);
                        scoped_css.push_str(&s);
                        global_css.push_str(&g);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    (bindings, scoped_css, global_css)
}

fn collect_bind_checks(nodes: &[token_parser::Node], checks: &mut Vec<proc_macro2::TokenStream>) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                // If this element has bind={Struct}, generate a check function
                if let Some(struct_path) = &elem.bind_struct {
                    let mut field_accesses = Vec::new();
                    collect_input_names(&elem.children, struct_path, &mut field_accesses);

                    if !field_accesses.is_empty() {
                        let check_fn_name =
                            quote::format_ident!("azumi_bind_check_{}", checks.len());

                        let check_block = quote! {
                            #[allow(unused_variables, non_snake_case)]
                            fn #check_fn_name(data: &#struct_path) {
                                #(#field_accesses)*
                            }
                        };
                        checks.push(check_block);
                    }
                }
                // Recurse
                collect_bind_checks(&elem.children, checks);
            }
            token_parser::Node::Fragment(frag) => {
                collect_bind_checks(&frag.children, checks);
            }
            token_parser::Node::Block(block) => {
                // Handle blocks (if, for, match, etc.) by recursing into their bodies
                match block {
                    token_parser::Block::If(if_block) => {
                        collect_bind_checks(&if_block.then_branch, checks);
                        if let Some(else_branch) = &if_block.else_branch {
                            collect_bind_checks(else_branch, checks);
                        }
                    }
                    token_parser::Block::For(for_block) => {
                        collect_bind_checks(&for_block.body, checks);
                    }
                    token_parser::Block::Match(match_block) => {
                        for arm in &match_block.arms {
                            collect_bind_checks(&arm.body, checks);
                        }
                    }
                    token_parser::Block::Call(call_block) => {
                        collect_bind_checks(&call_block.children, checks);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn collect_input_names(
    nodes: &[token_parser::Node],
    bind_struct: &syn::Path,
    errors: &mut Vec<proc_macro2::TokenStream>,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "input" || elem.name == "textarea" || elem.name == "select" {
                    // Find "name" attribute
                    for attr in &elem.attrs {
                        if attr.name == "name" {
                            if let token_parser::AttributeValue::Static(name_str) = &attr.value {
                                // Use the span of the attribute value for better error reporting
                                let span = attr.value_span.unwrap_or(attr.span);
                                let parts: Vec<&str> = name_str.split('.').collect();

                                // Validate each part is a valid Rust identifier
                                let mut all_valid = true;
                                for part in &parts {
                                    if !is_valid_identifier(part) {
                                        all_valid = false;
                                        let error_msg = format!(
                                            "Invalid field name '{}' in form binding. Field names must be valid Rust identifiers (no spaces or special characters).",
                                            part
                                        );
                                        errors.push(quote_spanned! {span=>
                                            compile_error!(#error_msg);
                                        });
                                        break;
                                    }
                                }

                                if !all_valid {
                                    continue;
                                }

                                // Generate identifiers for the field path
                                let field_idents: Vec<proc_macro2::Ident> = parts
                                    .iter()
                                    .map(|s| proc_macro2::Ident::new(s, span))
                                    .collect();

                                // Generate validation that field exists on struct
                                errors.push(quote! {
                                    let _ = &data.#(#field_idents).*;
                                });
                            }
                        }
                    }
                }
                // Recurse (e.g. input inside label or div)
                collect_input_names(&elem.children, bind_struct, errors);
            }
            token_parser::Node::Fragment(frag) => {
                collect_input_names(&frag.children, bind_struct, errors);
            }
            _ => {}
        }
    }
}

// Helper function to check if a string is a valid Rust identifier
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    // First character must be alphabetic or underscore
    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    // Remaining characters must be alphanumeric or underscore
    for c in chars {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }

    true
}

fn generate_nodes(nodes: &[token_parser::Node]) -> proc_macro2::TokenStream {
    let body = generate_body(nodes);
    quote! {
        #body
        Ok(())
    }
}

/// Strip outer quotes from string literals for cleaner text rendering
/// If the user wants literal quotes, they should use raw strings like r#""Hello""#
fn strip_outer_quotes(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.len() >= 2
        && ((trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\'')))
    {
        return trimmed[1..trimmed.len() - 1].to_string();
    }
    s.to_string()
}

#[derive(Clone, PartialEq, Debug)]
enum Context {
    Normal,
    Script,
}

#[derive(Clone, Debug)]
struct GenerationContext {
    mode: Context,
    scope_id: Option<String>,
    /// All valid class names (before scoping) for auto-scoping static strings
    valid_classes: std::collections::HashSet<String>,
    /// All valid ID names for auto-scoping static strings  
    valid_ids: std::collections::HashSet<String>,
}

impl GenerationContext {
    fn normal() -> Self {
        Self {
            mode: Context::Normal,
            scope_id: None,
            valid_classes: std::collections::HashSet::new(),
            valid_ids: std::collections::HashSet::new(),
        }
    }

    fn with_scope(scope_id: String, valid_classes: std::collections::HashSet<String>, valid_ids: std::collections::HashSet<String>) -> Self {
        Self {
            mode: Context::Normal,
            scope_id: Some(scope_id),
            valid_classes,
            valid_ids,
        }
    }

    fn with_mode(&self, mode: Context) -> Self {
        Self {
            mode,
            scope_id: self.scope_id.clone(),
            valid_classes: self.valid_classes.clone(),
            valid_ids: self.valid_ids.clone(),
        }
    }
}

/// Collect ALL CSS content from all <style> tags in the component
/// Returns (global_css, scoped_css)
fn collect_all_styles(nodes: &[token_parser::Node]) -> (String, String) {
    let mut global_css = String::new();
    let mut scoped_css = String::new();
    collect_styles_recursive(nodes, &mut global_css, &mut scoped_css);
    (global_css, scoped_css)
}

fn collect_styles_recursive(
    nodes: &[token_parser::Node],
    global_css: &mut String,
    scoped_css: &mut String,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "style" {
                    // Extract CSS from this style tag
                    if let Some(_src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        // External CSS is banned, do nothing here.
                        // Validation will catch it.
                    } else {
                        // Inline content - treat as scoped
                        for child in &elem.children {
                            if let token_parser::Node::Text(text) = child {
                                scoped_css.push_str(&text.content);
                                scoped_css.push('\n');
                            }
                        }
                    }
                } else {
                    // Recurse into children
                    collect_styles_recursive(&elem.children, global_css, scoped_css);
                }
            }
            token_parser::Node::Fragment(frag) => {
                collect_styles_recursive(&frag.children, global_css, scoped_css);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::Style(style_block) => {
                    // Handle <style> blocks - reconstruct CSS from TokenStream
                    let css_content =
                        crate::style::reconstruct_css_from_tokens(style_block.content.clone());
                    if style_block.is_global {
                        global_css.push_str(&css_content);
                        global_css.push('\n');
                    } else {
                        scoped_css.push_str(&css_content);
                        scoped_css.push('\n');
                    }
                }
                token_parser::Block::If(if_block) => {
                    collect_styles_recursive(&if_block.then_branch, global_css, scoped_css);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_styles_recursive(else_branch, global_css, scoped_css);
                    }
                }
                token_parser::Block::For(for_block) => {
                    collect_styles_recursive(&for_block.body, global_css, scoped_css);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_styles_recursive(&arm.body, global_css, scoped_css);
                    }
                }
                token_parser::Block::Call(call_block) => {
                    collect_styles_recursive(&call_block.children, global_css, scoped_css);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn generate_body(nodes: &[token_parser::Node]) -> proc_macro2::TokenStream {
    // Pass 0: CSS Validation - Revolutionary compile-time CSS type checking!
    let css_validation_errors = css_validator::validate_component_css(nodes);

    // If there are compile errors from CSS validation, emit them
    if !css_validation_errors.is_empty() {
        return css_validation_errors;
    }

    // Pass 0.1: Validate Node Order (Script -> HTML -> Style) - Enforced at top level
    let order_errors = html_structure_validator::validate_node_order(nodes);
    if !order_errors.is_empty() {
        let mut tokens = proc_macro2::TokenStream::new();
        for err in order_errors {
            tokens.extend(err);
        }
        return tokens;
    }

    // Collect all CSS content first (needed for validation and scoping)
    let (global_css, scoped_css) = collect_all_styles(nodes);

    // Pass 0.5: Strict CSS Validation (New Feature)
    // Extract valid selectors from the SCOPED CSS only (global CSS is opt-out)
    let (valid_classes, valid_ids) = crate::css::extract_selectors(&scoped_css);

    // Validate nodes against strict rules - COMPILE ERRORS
    let style_validation_errors =
        validate_nodes(nodes, &valid_classes, &valid_ids, !scoped_css.is_empty());
    if !style_validation_errors.is_empty() {
        return style_validation_errors;
    }

    // Pass 1: Check if component has any style tags
    let has_global = !global_css.is_empty();
    let has_scoped = !scoped_css.is_empty();

    if has_global || has_scoped {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Generate scoped CSS and ID
        let (scoped_output, scope_id) = if has_scoped {
            let mut hasher = DefaultHasher::new();
            scoped_css.hash(&mut hasher);
            let hash = hasher.finish();
            let scope_id = format!("s{:x}", hash);
            (crate::css::scope_css(&scoped_css, &scope_id), Some(scope_id))
        } else {
            (String::new(), None)
        };

        // Construct CSS string to inject
        let css_to_inject = if has_global {
            if has_scoped {
                format!("<style>{}</style><style data-azumi-internal=\"true\">{}</style>", global_css, scoped_output)
            } else {
                format!("<style>{}</style>", global_css)
            }
        } else {
             format!("<style data-azumi-internal=\"true\">{}</style>", scoped_output)
        };

        // Attempt to inject into AST (find <head>)
        // We need a mutable nodes vector
        let mut working_nodes = nodes.to_vec();
        let injected = inject_css_into_head(&mut working_nodes, &css_to_inject);

        // Generate body with context (scope ID etc)
        // If injected, working_nodes has the styles. If not, we prepend.
        let ctx = if let Some(sid) = scope_id {
            GenerationContext::with_scope(sid, valid_classes.clone(), valid_ids.clone())
        } else {
            GenerationContext::normal()
        };
        
        let body_content = generate_body_with_context(&working_nodes, &ctx);

        if injected {
            // CSS successfully injected into <head>
            body_content
        } else {
            // Fallback: Prepend (only for fragments/components without head)
            quote! {
                write!(f, "{}", #css_to_inject)?;
                #body_content
            }
        }
    } else {
        generate_body_with_context(nodes, &GenerationContext::normal())
    }
}

// Helper to inject CSS into <head>
fn inject_css_into_head(nodes: &mut Vec<token_parser::Node>, css: &str) -> bool {
    for node in nodes.iter_mut() {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "head" {
                    // Found <head>! Inject CSS as first child
                    // Create Node::Text with quoted string for generate_body_with_context
                    let content = format!("{:?}", css); // quote the string
                    let text_node = token_parser::Node::Text(token_parser::Text {
                        content,
                        span: elem.span,
                    });
                    elem.children.insert(0, text_node);
                    return true;
                }
                // Recurse
                if inject_css_into_head(&mut elem.children, css) {
                    return true;
                }
            }
            token_parser::Node::Fragment(frag) => {
                 if inject_css_into_head(&mut frag.children, css) {
                    return true;
                }
            }
            token_parser::Node::Block(token_parser::Block::If(if_block)) => {
                 if inject_css_into_head(&mut if_block.then_branch, css) { return true; }
                 if let Some(else_branch) = &mut if_block.else_branch {
                     if inject_css_into_head(else_branch, css) { return true; }
                 }
            }
            _ => {}
        }
    }
    false
}

fn validate_nodes(
    nodes: &[token_parser::Node],
    valid_classes: &std::collections::HashSet<String>,
    valid_ids: &std::collections::HashSet<String>,
    has_scoped_css: bool,
) -> proc_macro2::TokenStream {
    use quote::quote_spanned;
    let mut errors = vec![];

    fn collect_errors_recursive(
        nodes: &[token_parser::Node],
        valid_classes: &std::collections::HashSet<String>,
        valid_ids: &std::collections::HashSet<String>,
        has_scoped_css: bool,
        errors: &mut Vec<proc_macro2::TokenStream>,
        is_inside_form: bool,
        is_inside_button: bool,
        is_inside_anchor: bool,
    ) {
        for node in nodes {
            match node {
                token_parser::Node::Element(elem) => {
                    for attr in &elem.attrs {
                        let name = &attr.name;

                        // Rule 1: Check inline styles
                    if name == "style" {
                        match &attr.value {
                            token_parser::AttributeValue::Static(_) => {
                                let error_span = attr.value_span.unwrap_or(attr.span);
                                errors.push(quote_spanned! { error_span =>
                                    compile_error!("Static style attributes are banned (e.g. style=\"...\"). Use style={ --prop: value } instead.");
                                });
                            }
                            token_parser::AttributeValue::StyleDsl(_) => {
                                // Style DSL is allowed!
                            }
                            token_parser::AttributeValue::Dynamic(_) => {
                                // Dynamic styles are allowed (we trust the user or warn later)
                            }
                            _ => {}
                        }
                    }

                        // Rule 2: Check class existence - COMPILE ERROR
                        if name == "class" {
                            match &attr.value {
                                token_parser::AttributeValue::Static(_) => {
                                    let error_span = attr.value_span.unwrap_or(attr.span);
                                    errors.push(quote_spanned! { error_span =>
                                        compile_error!("Static class attributes are banned (e.g. class=\"...\"). Use class={variable_name} instead.");
                                    });
                                }
                                token_parser::AttributeValue::Dynamic(tokens) => {
                                    // Check if variable name matches an ID but is used in class
                                    if let Ok(ident) = syn::parse2::<syn::Ident>(tokens.clone()) {
                                        let var_name = ident.to_string();
                                        if valid_ids.contains(&var_name)
                                            && !valid_classes.contains(&var_name)
                                        {
                                            let msg = format!(
                                                "Variable '{}' refers to an ID selector (#{}) but is used in 'class' attribute. Did you mean to use 'id={}'?",
                                                var_name, var_name, var_name
                                            );
                                            errors.push(quote_spanned! { ident.span() =>
                                                compile_error!(#msg);
                                            });
                                        }
                                    }
                                }
                                token_parser::AttributeValue::StyleDsl(_) => {
                                    // Style DSL is not valid for class attribute
                                    errors.push(quote_spanned! { attr.span =>
                                        compile_error!("Style DSL syntax { --var: val } is only allowed in 'style' attribute.");
                                    });
                                }
                                _ => {}
                            }
                        }

                        // Rule 3: Check ID existence - COMPILE ERROR
                        if name == "id" {
                            match &attr.value {
                                token_parser::AttributeValue::Static(_) => {
                                    let error_span = attr.value_span.unwrap_or(attr.span);
                                    errors.push(quote_spanned! { error_span =>
                                        compile_error!("Static id attributes are banned (e.g. id=\"...\"). Use id={variable_name} instead.");
                                    });
                                }
                                token_parser::AttributeValue::Dynamic(tokens) => {
                                    // Check if variable name matches a Class but is used in ID
                                    if let Ok(ident) = syn::parse2::<syn::Ident>(tokens.clone()) {
                                        let var_name = ident.to_string();
                                        if valid_classes.contains(&var_name)
                                            && !valid_ids.contains(&var_name)
                                        {
                                            let msg = format!(
                                                "Variable '{}' refers to a Class selector (.{}) but is used in 'id' attribute. Did you mean to use 'class={}'?",
                                                var_name, var_name, var_name
                                            );
                                            errors.push(quote_spanned! { ident.span() =>
                                                compile_error!(#msg);
                                            });
                                        }
                                    }
                                }
                                token_parser::AttributeValue::StyleDsl(_) => {
                                    // Style DSL is not valid for id attribute
                                    errors.push(quote_spanned! { attr.span =>
                                        compile_error!("Style DSL syntax { --var: val } is only allowed in 'style' attribute.");
                                    });
                                }
                                _ => {}
                            }
                        }

                        // Rule 4: Validate attribute name (Strict HTML)
                        if let Some(err) = html_structure_validator::validate_attribute_name(attr) {
                            errors.push(err);
                        }

                        // Rule 5: Validate az-on target IDs (only for static IDs)
                        // Note: This validation only works for static ID strings like id="foo".
                        // Dynamic IDs like id={expr} cannot be validated at compile time.
                        if name == "az-on" {
                            if let token_parser::AttributeValue::Dynamic(tokens) = &attr.value {
                                let dsl = tokens.to_string();
                                // Parse "-> #target"
                                if let Some(idx) = dsl.find("->") {
                                    let rest = &dsl[idx + 2..];
                                    // TokenStream::to_string() adds spaces around punctuation (e.g. "# target")
                                    // We need to strip them to get the ID
                                    let target = rest.replace(" ", "");
                                    if target.starts_with('#') {
                                        let id_name = &target[1..];
                                        // Only validate if we have valid_ids to check against
                                        // If the target ID is not in our list, it might be:
                                        // 1. A dynamic ID (id={expr}) - can't validate
                                        // 2. An ID in a different component - can't validate
                                        // 3. A typo - we want to catch this
                                        //
                                        // We only error if we have CSS (meaning we collected IDs)
                                        // AND the ID is not found. If there's no CSS, we can't
                                        // know if an ID is valid or not.
                                        if !valid_ids.is_empty() && !valid_ids.contains(id_name) {
                                            // Only emit error if this looks like a typo
                                            // Skip validation for IDs that might be dynamic
                                            let msg = format!(
                                                "az-on target ID '{}' not found in CSS. If using a dynamic id={{expr}}, this validation cannot run. Otherwise, define #{} in your CSS.",
                                                id_name, id_name
                                            );
                                            // Use value_span to point to the attribute value
                                            let error_span = attr.value_span.unwrap_or(attr.span);
                                            errors.push(quote_spanned! { error_span =>
                                                compile_error!(#msg);
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Accessibility validation
                    if let Some(err) = accessibility_validator::validate_img_alt(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_input_type(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_aria_roles(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_button_content(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_anchor_target_blank(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_iframe_title(elem) {
                        errors.push(err);
                    }

                    // HTML Structure validation
                    if let Some(err) = html_structure_validator::validate_tag_name(elem) {
                        errors.push(err);
                    }
                    errors.extend(html_structure_validator::validate_table_children(elem));
                    errors.extend(html_structure_validator::validate_list_children(elem));
                    errors.extend(html_structure_validator::validate_nested_forms(
                        elem,
                        is_inside_form,
                    ));
                    errors.extend(html_structure_validator::validate_button_interactive(
                        elem,
                        is_inside_button,
                    ));
                    errors.extend(html_structure_validator::validate_paragraph_content(elem));
                    errors.extend(html_structure_validator::validate_anchor_nesting(
                        elem,
                        is_inside_anchor,
                    ));
                    errors.extend(html_structure_validator::validate_heading_content(elem));

                    // Update context for recursion
                    let new_is_inside_form = is_inside_form || elem.name.as_str() == "form";
                    let new_is_inside_button = is_inside_button || elem.name == "button";
                    let new_is_inside_anchor = is_inside_anchor || elem.name == "a";

                    // Recurse with updated context
                    collect_errors_recursive(
                        &elem.children,
                        valid_classes,
                        valid_ids,
                        has_scoped_css,
                        errors,
                        new_is_inside_form,
                        new_is_inside_button,
                        new_is_inside_anchor,
                    );
                }
                token_parser::Node::Fragment(frag) => {
                    collect_errors_recursive(
                        &frag.children,
                        valid_classes,
                        valid_ids,
                        has_scoped_css,
                        errors,
                        is_inside_form,
                        is_inside_button,
                        is_inside_anchor,
                    );
                }
                token_parser::Node::Block(block) => match block {
                    token_parser::Block::If(if_block) => {
                        collect_errors_recursive(
                            &if_block.then_branch,
                            valid_classes,
                            valid_ids,
                            has_scoped_css,
                            errors,
                            is_inside_form,
                            is_inside_button,
                            is_inside_anchor,
                        );
                        if let Some(else_branch) = &if_block.else_branch {
                            collect_errors_recursive(
                                else_branch,
                                valid_classes,
                                valid_ids,
                                has_scoped_css,
                                errors,
                                is_inside_form,
                                is_inside_button,
                                is_inside_anchor,
                            );
                        }
                    }
                    token_parser::Block::For(for_block) => {
                        collect_errors_recursive(
                            &for_block.body,
                            valid_classes,
                            valid_ids,
                            has_scoped_css,
                            errors,
                            is_inside_form,
                            is_inside_button,
                            is_inside_anchor,
                        );
                    }
                    token_parser::Block::Match(match_block) => {
                        for arm in &match_block.arms {
                            collect_errors_recursive(
                                &arm.body,
                                valid_classes,
                                valid_ids,
                                has_scoped_css,
                                errors,
                                is_inside_form,
                                is_inside_button,
                                is_inside_anchor,
                            );
                        }
                    }
                    token_parser::Block::Call(call_block) => {
                        collect_errors_recursive(
                            &call_block.children,
                            valid_classes,
                            valid_ids,
                            has_scoped_css,
                            errors,
                            is_inside_form,
                            is_inside_button,
                            is_inside_anchor,
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    collect_errors_recursive(
        nodes,
        valid_classes,
        valid_ids,
        has_scoped_css,
        &mut errors,
        false,
        false,
        false,
    );

    if errors.is_empty() {
        quote! {}
    } else {
        quote! { #(#errors)* }
    }
}

fn generate_body_with_context(
    nodes: &[token_parser::Node],
    ctx: &GenerationContext,
) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    for node in nodes {
        let chunk = match node {
            token_parser::Node::Element(elem) => {
                let name = &elem.name;

                // Skip <style> tags - they're already processed in generate_body
                if name == "style" {
                    continue;
                }

                // Determine context for children
                let child_context = if name == "script" {
                    ctx.with_mode(Context::Script)
                } else {
                    ctx.clone()
                };

                // Generate children
                let children_code = generate_body_with_context(&elem.children, &child_context);

                // Generate attributes
                let mut attr_code = proc_macro2::TokenStream::new();

                for attr in &elem.attrs {
                    let attr_name = &attr.name;

                    // Handle style attribute - ONLY CSS custom properties allowed
                    if attr_name == "style" {
                        match &attr.value {
                            token_parser::AttributeValue::Static(_) => {
                                let span = attr.value_span.unwrap_or(attr.span);
                                let err = syn::Error::new(span, "Static style attributes are banned (e.g. style=\"...\"). Use style={ --prop: value } instead.");
                                attr_code.extend(err.to_compile_error());
                                continue;
                            }
                            token_parser::AttributeValue::Dynamic(expr) => {
                                // Check for string literals -> BAN
                                if let Ok(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(_), .. })) = syn::parse2::<syn::Expr>(expr.clone()) {
                                    let err = syn::Error::new_spanned(expr, "String literals in style attribute are banned. Use Style DSL style={ --prop: val }.");
                                    attr_code.extend(err.to_compile_error());
                                } else {
                                    attr_code.extend(quote! {
                                        write!(f, " style=\"{}\"", azumi::Escaped(&(#expr)))?;
                                    });
                                }
                            }
                            token_parser::AttributeValue::StyleDsl(props) => {
                                // Generate optimized style string construction
                                // style="--var1: val1; --var2: val2"
                                let mut format_string = String::new();
                                let mut args = Vec::new();

                                for (i, (prop, value_tokens)) in props.iter().enumerate() {
                                    if i > 0 {
                                        format_string.push_str("; ");
                                    }
                                    format_string.push_str(prop);
                                    format_string.push_str(": {}");
                                    
                                    // Check if value is a string literal to strip quotes
                                    // This allows style={ --color: "red" } to output --color: red
                                    // instead of --color: "red" which is invalid CSS
                                    let value_expr = if let Ok(lit) = syn::parse2::<syn::Lit>(value_tokens.clone()) {
                                        if let syn::Lit::Str(s) = lit {
                                            let s_val = s.value();
                                            quote! { #s_val }
                                        } else {
                                            quote! { #value_tokens }
                                        }
                                    } else {
                                        quote! { #value_tokens }
                                    };
                                    
                                    args.push(value_expr);
                                }

                                attr_code.extend(quote! {
                                    write!(f, " style=\"{}\"", format_args!(#format_string, #(#args),*))?;
                                });
                            }
                            token_parser::AttributeValue::None => {
                                // Empty style is fine
                            }
                        }
                        continue;
                    }

                    // Handle class attribute - AUTO-SCOPE static strings
                    if attr_name == "class" {
                        match &attr.value {
                            token_parser::AttributeValue::Static(_) => {
                                let span = attr.value_span.unwrap_or(attr.span);
                                let err = syn::Error::new(span, "Static class attributes are banned (e.g. class=\"...\"). Use class={variable_name} instead.");
                                attr_code.extend(err.to_compile_error());
                                continue;
                            }
                            token_parser::AttributeValue::Dynamic(tokens) => {
                                // Dynamic class - "Expression List" support
                                // Allows: class={ "my-class" active_state } -> "my-class-s123 active_val"
                                use syn::parse::Parser;
                                
                                let parser = |input: syn::parse::ParseStream| -> syn::Result<Vec<syn::Expr>> {
                                    let mut exprs = Vec::new();
                                    while !input.is_empty() {
                                        exprs.push(input.parse()?);
                                    }
                                    Ok(exprs)
                                };

                                if let Ok(expr_list) = parser.parse2(tokens.clone()) {
                                    attr_code.extend(quote! { write!(f, " class=\"")?; });
                                    
                                    for (i, expr) in expr_list.iter().enumerate() {
                                        // BAN String Literals in class list
                                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(_), .. }) = expr {
                                            let err = syn::Error::new_spanned(expr, "String literals in class attribute are banned. Use variables.");
                                            attr_code.extend(err.to_compile_error());
                                            continue;
                                        }

                                        if i > 0 {
                                            attr_code.extend(quote! { write!(f, " ")?; });
                                        }

                                        // Strict Mode: Just output the expression.
                                        // No auto-scoping of string literals.
                                        // No magic mapping.
                                        // Variables generated by css binding will handle scoping.
                                        attr_code.extend(quote! {
                                            write!(f, "{}", azumi::Escaped(&(#expr)))?;
                                        });
                                    }
                                    
                                    attr_code.extend(quote! { write!(f, "\"")?; });
                                    continue;
                                }
                                // Fallback to generic handler if parsing fails (unlikely, but safe)
                            }
                            _ => {}
                        }
                    }

                    // Handle id attribute - IDs are NOT scoped (they remain as-is)
                    // But we still need to handle them specially for static values
                    if attr_name == "id" {
                        match &attr.value {
                            token_parser::AttributeValue::Static(_) => {
                                let span = attr.value_span.unwrap_or(attr.span);
                                let err = syn::Error::new(span, "Static id attributes are banned (e.g. id=\"...\"). Use id={variable_name} instead.");
                                attr_code.extend(err.to_compile_error());
                                continue;
                            }
                            token_parser::AttributeValue::Dynamic(tokens) => {
                                // BAN String Literals in id
                                if let Ok(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(_), .. })) = syn::parse2::<syn::Expr>(tokens.clone()) {
                                    let err = syn::Error::new_spanned(tokens, "String literals in id attribute are banned. Use variables.");
                                    attr_code.extend(err.to_compile_error());
                                    continue;
                                }
                            }
                            _ => {}
                        }
                    }

                    // Handle az-on DSL: az-on={click call foo} -> az-on="click call foo"
                    if attr_name == "az-on" {
                        match &attr.value {
                            token_parser::AttributeValue::Dynamic(tokens) => {
                                // Convert tokens to string literal
                                let dsl_string = tokens.to_string();
                                // We might want to validate the DSL here later
                                attr_code.extend(quote! {
                                    write!(f, " az-on=\"{}\"", #dsl_string)?;
                                });
                                continue;
                            }
                            token_parser::AttributeValue::Static(val) => {
                                attr_code.extend(quote! {
                                    write!(f, " az-on=\"{}\"", #val)?;
                                });
                                continue;
                            }
                            _ => {}
                        }
                    }

                    // Handle on:event syntax (e.g. on:click={state.increment})
                    if attr_name.starts_with("on:") {
                        let event = &attr_name[3..];
                        if let token_parser::AttributeValue::Dynamic(tokens) = &attr.value {
                            // Parse the expression: state.method
                            // We expect a path like `state.method`
                            if let Ok(expr) = syn::parse2::<syn::Expr>(tokens.clone()) {
                                if let syn::Expr::Field(field_expr) = expr {
                                    // Extract base (state) and member (method)
                                    let base = &field_expr.base;
                                    let method = &field_expr.member;

                                    if let syn::Member::Named(method_ident) = method {
                                        let method_name = method_ident.to_string();

                                        // Generate az-on attribute
                                        // We need the scope ID to target the update
                                        // If we are inside a scope (ctx.scope_id is set), we use that
                                        // But wait, ctx.scope_id is for CSS scoping!
                                        // We need the *runtime* scope ID from the az-scope attribute
                                        // For now, let's assume the target is the current element's closest scope
                                        // which we can target with a special selector or just let the runtime handle it?
                                        // The runtime looks for closest [az-scope] if no target specified!
                                        // So "click call method" implies target=closest scope.
                                        
                                        // However, we need to know if we should generate data-predict.
                                        // We can check if the base type implements LiveState and has a prediction for this method.
                                        // But we can't check types at macro expansion time easily.
                                        // Instead, we can generate code that checks it at runtime or use a helper trait.
                                        
                                        // Let's generate a runtime helper call:
                                        // azumi::render_event(f, "click", &state, "method")?;
                                        
                                        // Use the base expression to infer the type for LiveState trait
                                        attr_code.extend(quote! {
                                            {
                                                // Use the base expression to infer the type
                                                fn get_predictions<T: azumi::LiveState>(_: &T) -> &'static [(&'static str, &'static str)] {
                                                    T::predictions()
                                                }
                                                let predictions = get_predictions(#base);
                                                let prediction = predictions.iter()
                                                    .find(|(m, _)| *m == #method_name)
                                                    .map(|(_, p)| *p)
                                                    .unwrap_or("");
                                                
                                                if !prediction.is_empty() {
                                                    write!(f, " data-predict=\"{}\"", prediction)?;
                                                }
                                                
                                                // Generate az-on with namespaced action
                                                // Default target is the closest scope (implied by missing -> target)
                                                fn get_struct_name<T: azumi::LiveState>(_: &T) -> &'static str {
                                                    T::struct_name()
                                                }
                                                let struct_name = get_struct_name(#base);
                                                write!(f, " az-on=\"{} call {}/{}\"", #event, struct_name, #method_name)?;
                                            }
                                        });
                                        continue;
                                    }
                                }
                            }
                        }
                    }

                    match &attr.value {
                        token_parser::AttributeValue::Static(val) => {
                            attr_code.extend(quote! {
                                write!(f, " {}=\"{}\"", #attr_name, azumi::Escaped(#val))?;
                            });
                        }
                        token_parser::AttributeValue::Dynamic(expr) => {
                            attr_code.extend(quote! {
                                write!(f, " {}=\"{}\"", #attr_name, azumi::Escaped(&(#expr)))?;
                            });
                        }
                        token_parser::AttributeValue::StyleDsl(props) => {
                            // Style DSL is only valid for 'style' attribute, which is handled above.
                            // If we reach here, it means it was used on a non-style attribute, which is invalid.
                            // But we should handle it gracefully or error.
                            // Since we can't emit compile_error! easily inside this loop without returning TokenStream,
                            // we'll just ignore it or emit a warning via println (which is visible during build).
                            // Ideally, validation pass should have caught this.
                        }
                        token_parser::AttributeValue::None => {
                            // Boolean attribute
                            attr_code.extend(quote! {
                                write!(f, " {}", #attr_name)?;
                            });
                        }
                    }
                }

                // Auto-optimization for <img> tags
                if name == "img" {
                    // Check if attributes were already present in the source
                    let has_loading = elem.attrs.iter().any(|a| a.name == "loading");
                    let has_decoding = elem.attrs.iter().any(|a| a.name == "decoding");

                    if !has_loading {
                        attr_code.extend(quote! {
                            write!(f, " loading=\"lazy\"")?;
                        });
                    }
                    if !has_decoding {
                        attr_code.extend(quote! {
                            write!(f, " decoding=\"async\"")?;
                        });
                    }
                }

                // Generate element with potential scope attribute from context
                if let Some(ref scope_id) = ctx.scope_id {
                    quote! {
                        write!(f, "<{}", #name)?;
                        write!(f, " data-{}", #scope_id)?;
                        #attr_code
                        write!(f, ">")?;
                        #children_code
                        write!(f, "</{}>", #name)?;
                    }
                } else {
                    quote! {
                        write!(f, "<{}", #name)?;
                        #attr_code
                        write!(f, ">")?;
                        #children_code
                        write!(f, "</{}>", #name)?;
                    }
                }
            }
            token_parser::Node::Text(text) => {
                let content = &text.content;
                if content.is_empty() {
                    quote! {}
                } else {
                    // Strip outer quotes from string literals for cleaner rendering
                    let stripped = strip_outer_quotes(content);
                    quote! { write!(f, "{}", #stripped)?; }
                }
            }
            token_parser::Node::Expression(expr) => {
                let content = &expr.content;
                match ctx.mode {
                    Context::Script => {
                        // In JSON script tags, output raw (user provides JSON string via serde_json::to_string etc.)
                        quote! { write!(f, "{}", #content)?; }
                    }
                    Context::Normal => {
                        // In normal HTML, use Smart Interpolation
                        // RenderWrapper::render_azumi will pick:
                        // 1. Component::render (if it's a Component)
                        // 2. Escaped Display (if it's just Display)
                        quote! {
                            {
                                use azumi::FallbackRender; // Import trait for fallback
                                azumi::RenderWrapper(&(#content)).render_azumi(f)?;
                            }
                        }
                    }
                }
            }
            token_parser::Node::Comment(_) => {
                // Ignore comments in output
                quote! {}
            }
            token_parser::Node::Doctype(doctype) => {
                let content = &doctype.content;
                quote! { write!(f, "<!DOCTYPE {}>", #content)?; }
            }
            token_parser::Node::Fragment(frag) => generate_body_with_context(&frag.children, ctx),
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    let condition = &if_block.condition;
                    let then_code = generate_body_with_context(&if_block.then_branch, ctx);
                    let else_code = if let Some(else_branch) = &if_block.else_branch {
                        let else_body = generate_body_with_context(else_branch, ctx);
                        quote! { else { #else_body } }
                    } else {
                        quote! {}
                    };
                    quote! {
                        if #condition {
                            #then_code
                        } #else_code
                    }
                }
                token_parser::Block::For(for_block) => {
                    let pattern = &for_block.pattern;
                    let iterator = &for_block.iterator;
                    let body_code = generate_body_with_context(&for_block.body, ctx);
                    quote! {
                        for #pattern in #iterator {
                            #body_code
                        }
                    }
                }
                token_parser::Block::Match(match_block) => {
                    let expr = &match_block.expr;
                    let arms = match_block.arms.iter().map(|arm| {
                        let pattern = &arm.pattern;
                        let body = generate_body_with_context(&arm.body, ctx);
                        quote! {
                            #pattern => { #body }
                        }
                    });
                    quote! {
                        match #expr {
                            #(#arms),*
                        }
                    }
                }
                token_parser::Block::Call(call_block) => {
                    let name = &call_block.name;
                    let args = &call_block.args;
                    let has_children = !call_block.children.is_empty();
                    let children_code = if has_children {
                        let children_body = generate_body_with_context(&call_block.children, ctx);
                        quote! {
                            azumi::from_fn(|f| {
                                #children_body
                                Ok(())
                            })
                        }
                    } else {
                        quote! {}
                    };

                    // AZUMI 2.0: ENFORCE NAMED ARGUMENTS
                    // All component calls must use named arguments for clarity and maintainability
                    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::token::Comma>::parse_terminated;
                    let named_args = if let Ok(exprs) =
                        syn::parse::Parser::parse2(parser, args.clone())
                    {
                        // Check if we have positional arguments (not all are Assign expressions)
                        let has_positional = !exprs.is_empty()
                            && !exprs.iter().all(|e| matches!(e, syn::Expr::Assign(_)));

                        if has_positional {
                            // Positional arguments detected - generate clear compile error
                            // We enforce named arguments for ALL components to ensure #[component] usage
                            let name_str = quote!(#name).to_string();

                            // First, try to find the first positional argument to highlight
                            // This helps users see exactly which argument is the problem
                            let error_span = if let Some(first_positional) =
                                exprs.iter().find(|e| !matches!(e, syn::Expr::Assign(_)))
                            {
                                // Use the span of the first positional argument for precision
                                first_positional.span()
                            } else {
                                // Fallback to the entire call span
                                call_block.span
                            };

                            // Show the actual problematic call for better context
                            let positional_values: Vec<String> = exprs
                                .iter()
                                .filter(|e| !matches!(e, syn::Expr::Assign(_)))
                                .map(|e| {
                                    let s = quote!(#e).to_string();
                                    // Truncate long values
                                    if s.len() > 20 {
                                        format!("{}...", &s[..17])
                                    } else {
                                        s
                                    }
                                })
                                .collect();

                            let error_msg = if !positional_values.is_empty() {
                                format!(
                                        "Component '{}' must be called with named arguments.\n\
                                         \n\
                                         ❌ You wrote:\n\
                                         @{}({})\n\
                                         \n\
                                         ✅ Use named arguments instead:\n\
                                         @{}(param1={}, param2={}, ...)\n\
                                         \n\
                                         💡 Tip: Check the component definition to find the exact parameter names.\n\
                                         Named arguments prevent bugs when signatures change and make code self-documenting.",
                                        name_str,
                                        name_str,
                                        positional_values.join(", "),
                                        name_str,
                                        positional_values.get(0).unwrap_or(&"...".to_string()),
                                        positional_values.get(1).unwrap_or(&"...".to_string())
                                    )
                            } else {
                                format!(
                                        "Component '{}' must be called with named arguments.\n\
                                         \n\
                                         ❌ Positional arguments are not allowed:\n\
                                         @{}(\"value1\", \"value2\")  // Error!\n\
                                         \n\
                                         ✅ Use named arguments instead:\n\
                                         @{}(arg1=\"value1\", arg2=\"value2\")\n\
                                         \n\
                                         Why? Named arguments prevent bugs when component signatures change\n\
                                         and make code self-documenting.",
                                        name_str, name_str, name_str
                                    )
                            };

                            return syn::Error::new(error_span, error_msg).to_compile_error();
                        } else if !exprs.is_empty() {
                            // All are named arguments - good!
                            Some(exprs)
                        } else {
                            // Empty args: treat as named args (Builder pattern) to support defaults
                            // This enforces #[component] even for empty arg calls
                            Some(exprs)
                        }
                    } else {
                        None
                    };

                    if let Some(exprs) = named_args {
                        let setters = exprs.iter().map(|e| {
                            if let syn::Expr::Assign(assign) = e {
                                let key = &assign.left;
                                let value = &assign.right;
                                quote! { .#key(#value) }
                            } else {
                                unreachable!()
                            }
                        });

                        let children_arg = if has_children {
                            quote! { , #children_code }
                        } else {
                            quote! {}
                        };

                        // Check if name is snake_case (starts with lowercase)
                        let name_str = quote!(#name).to_string();
                        let last_segment = name_str.split("::").last().unwrap_or("").trim();
                        let is_snake_case = last_segment
                            .chars()
                            .next()
                            .is_some_and(|c| c.is_lowercase());

                        let module_name = if is_snake_case {
                            let new_name = format!("{}_component", name_str);
                            syn::parse_str::<syn::Path>(&new_name).unwrap_or(name.clone())
                        } else {
                            name.clone()
                        };

                        quote! {
                            azumi::Component::render(&#module_name::render(#module_name::Props::builder()
                                #(#setters)*
                                .build().expect("Failed to build props")
                                #children_arg), f)?;
                        }
                    } else {
                        let args_separator = if !args.is_empty() && has_children {
                            quote! { , }
                        } else {
                            quote! {}
                        };

                        quote! {
                            azumi::Component::render(&#name(#args #args_separator #children_code), f)?;
                        }
                    }
                }
                token_parser::Block::Component(comp_block) => {
                    let name = &comp_block.name;
                    quote! {
                        azumi::Component::render(&#name, f)?;
                    }
                }
                token_parser::Block::Let(let_block) => {
                    let pattern = &let_block.pattern;
                    let value = &let_block.value;
                    quote! {
                        let #pattern = #value;
                    }
                }
                token_parser::Block::Style(_) => {
                    // Styles are hoisted and handled separately, so we ignore them here
                    quote! {}
                }
            },
        };
        stream.extend(chunk);
    }
    stream
}


#[cfg(test)]
mod token_parser_tests;
