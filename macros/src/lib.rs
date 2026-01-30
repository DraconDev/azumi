// Force rebuild 15
mod component;

mod accessibility_validator;
mod action;
mod asset_rewriter;
mod css;
mod css_validator;
mod head;
mod html_structure_validator;
mod live;
mod page;
#[cfg(feature = "schema")]
mod schema;
mod style;
mod test_spacing;
mod token_parser;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Parser};
use syn::parse_macro_input;
use syn::Token;
// use syn::spanned::Spanned;

#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
    head::expand_head(input)
}

#[proc_macro_attribute]
pub fn page(attr: TokenStream, item: TokenStream) -> TokenStream {
    page::expand_page(attr, item)
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

#[proc_macro_attribute]
pub fn live(attr: TokenStream, item: TokenStream) -> TokenStream {
    live::expand_live(attr, item)
}

#[proc_macro_attribute]
pub fn live_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    live::expand_live_impl(attr, item)
}

#[proc_macro_attribute]
pub fn predict(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[allow(dead_code)]
struct NodesWrapper(Vec<token_parser::Node>);

impl Parse for NodesWrapper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        token_parser::parse_nodes(input).map(NodesWrapper)
    }
}

// Helpers for parsing Component arguments
struct KeyValueArg {
    key: syn::Ident,
    value: syn::Expr,
}

impl Parse for KeyValueArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(KeyValueArg { key, value })
    }
}

fn parse_args(tokens: proc_macro2::TokenStream) -> syn::Result<Vec<KeyValueArg>> {
    let parser = syn::punctuated::Punctuated::<KeyValueArg, Token![,]>::parse_terminated;
    parser.parse2(tokens).map(|p| p.into_iter().collect())
}

// Helper to transform snake_case component paths to their module name (append _component)
fn transform_path_for_component(path: &syn::Path) -> syn::Path {
    let mut new_path = path.clone();
    if let Some(last) = new_path.segments.last_mut() {
        let s = last.ident.to_string();
        // If starts with lowercase, it's snake_case -> No longer appending _component suffix
        if s.chars().next().map(|c| c.is_lowercase()).unwrap_or(false) {
            // last.ident = syn::Ident::new(&format!("{}_component", s), last.ident.span());
        }
    }
    new_path
}

// Helper for parsing space-separated expressions (e.g. class={expr1 expr2})
fn parse_multi_exprs(input: ParseStream) -> syn::Result<Vec<syn::Expr>> {
    let mut exprs = Vec::new();
    while !input.is_empty() {
        exprs.push(input.parse()?);
    }
    Ok(exprs)
}

/// Validates that a style attribute only contains CSS custom properties (--variables).
#[allow(dead_code)]
fn validate_style_only_css_vars(style_value: &str) -> Result<(), String> {
    for part in style_value.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(colon_pos) = part.find(':') {
            let prop_name = part[..colon_pos].trim();
            if !prop_name.starts_with("--") {
                return Err(format!(
                    "Invalid style property '{}'. Only CSS variables are allowed in inline styles.",
                    prop_name
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

    asset_rewriter::rewrite_nodes(&mut nodes);

    let (style_bindings, _scoped_css, _global_css) = process_styles(&nodes);

    let css_deps: Vec<proc_macro2::TokenStream> = Vec::new();

    // HOT RELOAD INJECTION START
    let hot_reload_code = if let Some((_statics, dynamics)) = try_extract_template(&nodes) {
        let id_lit = if !nodes.is_empty() {
            let _span = nodes[0].span();
            quote! {
               concat!(file!(), ":", line!(), ":", column!())
            }
        } else {
            quote! { "unknown" }
        };

        let dynamic_blocks = dynamics.iter().enumerate().map(|(i, d)| {
            let w_name = quote::format_ident!("__w{}", i);
            let c_name = quote::format_ident!("__c{}", i);
            let h_name = quote::format_ident!("__h{}", i);
            quote! {
                let #w_name = azumi::RenderWrapper(&(#d));
                let #c_name = |f: &mut std::fmt::Formatter| #w_name.render_azumi(f);
                let #h_name = azumi::HotReloadClosure(&#c_name);
            }
        });

        let dynamic_refs = dynamics.iter().enumerate().map(|(i, _)| {
            let h_name = quote::format_ident!("__h{}", i);
            quote! { &#h_name }
        });

        quote! {
            #[cfg(debug_assertions)]
            {
                if let Some(tmpl) = azumi::hot_reload::get_template(#id_lit) {
                    #(#dynamic_blocks)*
                    let dyns: &[&dyn azumi::FallbackRender] = &[ #(#dynamic_refs),* ];
                    return tmpl.render(f, dyns);
                }
            }
        }
    } else {
        quote! {}
    };
    // HOT RELOAD INJECTION END

    let html_construction = generate_nodes(&nodes);

    let mut validation_checks = Vec::new();
    collect_bind_checks(&nodes, &mut validation_checks);

    let expanded = quote! {
        {
            #[allow(unused_imports)]
            use azumi::FallbackRender;

            #style_bindings
            #(#css_deps)*

            const _: () = {
                #(#validation_checks)*
            };

            azumi::from_fn(move |f| {
                #hot_reload_code
                if false {}
                #html_construction
            })
        }
    };

    TokenStream::from(expanded)
}

#[allow(dead_code)]
fn extract_styles(
    nodes: Vec<token_parser::Node>,
) -> (Vec<token_parser::StyleBlock>, Vec<token_parser::Node>) {
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
                    let output = style::process_global_style_macro(style_block.content.clone());
                    bindings.extend(output.bindings);
                    global_css.push_str(&output.css);
                } else {
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
                token_parser::Block::Call(call_block) => {
                    let (b, s, g) = process_styles(&call_block.children);
                    bindings.extend(b);
                    scoped_css.push_str(&s);
                    global_css.push_str(&g);
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
                collect_bind_checks(&elem.children, checks);
            }
            token_parser::Node::Fragment(frag) => {
                collect_bind_checks(&frag.children, checks);
            }
            token_parser::Node::Block(block) => match block {
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
            },
            _ => {}
        }
    }
}

#[allow(clippy::only_used_in_recursion)]
fn collect_input_names(
    nodes: &[token_parser::Node],
    bind_struct: &syn::Path,
    errors: &mut Vec<proc_macro2::TokenStream>,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "input" || elem.name == "textarea" || elem.name == "select" {
                    for attr in &elem.attrs {
                        if attr.name == "name" {
                            if let token_parser::AttributeValue::Static(name_str) = &attr.value {
                                let span = attr.value_span.unwrap_or(attr.span);
                                let parts: Vec<&str> = name_str.split('.').collect();
                                let mut all_valid = true;
                                for part in &parts {
                                    if !is_valid_identifier(part) {
                                        all_valid = false;
                                        let error_msg = format!("Invalid field name: {}", part);
                                        errors.push(
                                            quote_spanned! {span=> compile_error!(#error_msg); },
                                        );
                                        break;
                                    }
                                }

                                if !all_valid {
                                    continue;
                                }

                                let field_idents: Vec<proc_macro2::Ident> = parts
                                    .iter()
                                    .map(|s| proc_macro2::Ident::new(s, span))
                                    .collect();

                                errors.push(quote! {
                                    let _ = &data.#(#field_idents).*;
                                });
                            }
                        }
                    }
                }
                collect_input_names(&elem.children, bind_struct, errors);
            }
            token_parser::Node::Fragment(frag) => {
                collect_input_names(&frag.children, bind_struct, errors);
            }
            _ => {}
        }
    }
}

fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
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
    valid_classes: std::collections::HashSet<String>,
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

    fn with_scope(
        scope_id: String,
        valid_classes: std::collections::HashSet<String>,
        valid_ids: std::collections::HashSet<String>,
    ) -> Self {
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
                    if let Some(_src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                    } else {
                        for child in &elem.children {
                            if let token_parser::Node::Text(text) = child {
                                scoped_css.push_str(&text.content);
                                scoped_css.push('\n');
                            }
                        }
                    }
                } else {
                    collect_styles_recursive(&elem.children, global_css, scoped_css);
                }
            }
            token_parser::Node::Fragment(frag) => {
                collect_styles_recursive(&frag.children, global_css, scoped_css);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::Style(style_block) => {
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
    let css_validation_errors = css_validator::validate_component_css(nodes);
    if !css_validation_errors.is_empty() {
        return css_validation_errors;
    }

    let order_errors = html_structure_validator::validate_node_order(nodes);
    if !order_errors.is_empty() {
        let mut tokens = proc_macro2::TokenStream::new();
        for err in order_errors {
            tokens.extend(err);
        }
        return tokens;
    }

    let (global_css, scoped_css) = collect_all_styles(nodes);
    let (valid_classes, valid_ids) = crate::css::extract_selectors(&scoped_css);

    let style_validation_errors =
        validate_nodes(nodes, &valid_classes, &valid_ids, !scoped_css.is_empty());
    if !style_validation_errors.is_empty() {
        return style_validation_errors;
    }

    let has_global = !global_css.is_empty();
    let has_scoped = !scoped_css.is_empty();

    if has_global || has_scoped {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let (scoped_output, scope_id) = if has_scoped {
            let mut hasher = DefaultHasher::new();
            if cfg!(debug_assertions) {
                let span = nodes[0].span();
                span.start().line.hash(&mut hasher);
                span.start().column.hash(&mut hasher);
            } else {
                scoped_css.hash(&mut hasher);
            }

            let hash = hasher.finish();
            let scope_id = format!("s{:x}", hash);
            (
                crate::css::scope_css(&scoped_css, &scope_id),
                Some(scope_id),
            )
        } else {
            (String::new(), None)
        };

        let css_to_inject = if has_global {
            if let Some(sid) = &scope_id {
                format!(
                    "<style>{}</style><style data-azumi-internal=\"true\" data-azumi-scope=\"{}\">{}</style>",
                    global_css, sid, scoped_output
                )
            } else {
                format!("<style>{}</style>", global_css)
            }
        } else if let Some(sid) = &scope_id {
            format!(
                "<style data-azumi-internal=\"true\" data-azumi-scope=\"{}\">{}</style>",
                sid, scoped_output
            )
        } else {
            String::new()
        };

        let mut working_nodes = nodes.to_vec();
        let injected = inject_css_into_head(&mut working_nodes, &css_to_inject);

        let ctx = if let Some(sid) = scope_id {
            GenerationContext::with_scope(sid, valid_classes.clone(), valid_ids.clone())
        } else {
            GenerationContext::normal()
        };

        let body_content = generate_body_with_context(&working_nodes, &ctx);

        if injected {
            body_content
        } else {
            quote! {
                write!(f, "{}", #css_to_inject)?;
                #body_content
            }
        }
    } else {
        generate_body_with_context(nodes, &GenerationContext::normal())
    }
}

#[allow(clippy::ptr_arg)]
fn inject_css_into_head(nodes: &mut Vec<token_parser::Node>, css: &str) -> bool {
    for node in nodes.iter_mut() {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "head" {
                    let content = format!("{:?}", css);
                    let text_node = token_parser::Node::Text(token_parser::Text {
                        content,
                        span: elem.span,
                    });
                    elem.children.insert(0, text_node);
                    return true;
                }
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
                if inject_css_into_head(&mut if_block.then_branch, css) {
                    return true;
                }
                if let Some(else_branch) = &mut if_block.else_branch {
                    if inject_css_into_head(else_branch, css) {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

/// Collect all let binding variable names from the AST
fn collect_let_bindings(nodes: &[token_parser::Node]) -> std::collections::HashSet<String> {
    let mut bindings = std::collections::HashSet::new();
    
    fn collect_recursive(nodes: &[token_parser::Node], bindings: &mut std::collections::HashSet<String>) {
        for node in nodes {
            match node {
                token_parser::Node::Block(token_parser::Block::Let(let_block)) => {
                    // Extract the variable name from the pattern
                    // The pattern is a TokenStream, usually just an identifier
                    if let Ok(ident) = syn::parse2::<syn::Ident>(let_block.pattern.clone()) {
                        bindings.insert(ident.to_string());
                    }
                }
                token_parser::Node::Element(elem) => {
                    collect_recursive(&elem.children, bindings);
                }
                token_parser::Node::Fragment(frag) => {
                    collect_recursive(&frag.children, bindings);
                }
                token_parser::Node::Block(block) => match block {
                    token_parser::Block::If(if_block) => {
                        collect_recursive(&if_block.then_branch, bindings);
                        if let Some(else_branch) = &if_block.else_branch {
                            collect_recursive(else_branch, bindings);
                        }
                    }
                    token_parser::Block::For(for_block) => {
                        collect_recursive(&for_block.body, bindings);
                    }
                    token_parser::Block::Match(match_block) => {
                        for arm in &match_block.arms {
                            collect_recursive(&arm.body, bindings);
                        }
                    }
                    token_parser::Block::Call(call_block) => {
                        collect_recursive(&call_block.children, bindings);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    
    collect_recursive(nodes, &mut bindings);
    bindings
}

/// Check if a value looks like a CSS class name (contains dashes, spaces, or is snake_case)
fn looks_like_class_name(value: &str) -> bool {
    // Check if it's a string that looks like a CSS class
    // e.g., "my_class", "btn-primary", "btn primary"
    value.contains('_') || value.contains('-') || value.contains(' ')
}

/// Check if a let binding value is a string literal defining a class name
fn is_let_class_definition(value: &proc_macro2::TokenStream) -> bool {
    let value_str = value.to_string();
    // Check if it's a quoted string
    if (value_str.starts_with('"') && value_str.ends_with('"'))
        || (value_str.starts_with("\"") && value_str.ends_with("\""))
    {
        let inner = value_str.trim_matches('"');
        looks_like_class_name(inner)
    } else {
        false
    }
}

fn validate_nodes(
    nodes: &[token_parser::Node],
    valid_classes: &std::collections::HashSet<String>,
    valid_ids: &std::collections::HashSet<String>,
    has_scoped_css: bool,
) -> proc_macro2::TokenStream {
    use quote::quote_spanned;
    let mut errors = vec![];
    
    // Collect all let bindings to detect shadowing anti-pattern
    let let_bindings = collect_let_bindings(nodes);

    #[allow(clippy::too_many_arguments)]
    fn collect_errors_recursive(
        nodes: &[token_parser::Node],
        valid_classes: &std::collections::HashSet<String>,
        valid_ids: &std::collections::HashSet<String>,
        let_bindings: &std::collections::HashSet<String>,
        _has_scoped_css: bool,
        errors: &mut Vec<proc_macro2::TokenStream>,
        _is_inside_form: bool,
        _is_inside_button: bool,
        _is_inside_anchor: bool,
    ) {
        for node in nodes {
            match node {
                token_parser::Node::Block(token_parser::Block::Let(let_block)) => {
                    // Check for anti-pattern: @let my_class = "my_class" or @let btn = "btn-primary"
                    if is_let_class_definition(&let_block.value) {
                        if let Ok(ident) = syn::parse2::<syn::Ident>(let_block.pattern.clone()) {
                            let var_name = ident.to_string();
                            let value_str = let_block.value.to_string().trim_matches('"').to_string();
                            
                            let msg = format!(
                                "ANTI-PATTERN: @let {} = \"{}\"\n\n\
                                Using @let to define CSS class names is NOT allowed in Azumi.\n\
                                CSS classes must be defined in <style> blocks, not as variables.\n\n\
                                CORRECT - Define in <style> block:\n\
                                    <div class={{{{var}}}>...</div>\n\
                                    <style>\n\
                                        .{}{{ ... }}\n\
                                    </style>\n\n\
                                INCORRECT - Using @let for classes:\n\
                                    @let {var} = \"{}\";  // DON'T DO THIS!\n\
                                    <div class={{{{var}}}>...</div>\n\n\
                                The <style> block automatically creates the variable for you.\n\
                                See: AI_GUIDE_FOR_WRITING_AZUMI.md - Critical Rules section",
                                var_name, value_str, var_name, value_str
                            );
                            errors.push(quote_spanned! { let_block.span =>
                                compile_error!(#msg);
                            });
                        }
                    }
                }
                token_parser::Node::Element(elem) => {
                    for attr in &elem.attrs {
                        let name = &attr.name;

                        if name == "style" {
                            if let token_parser::AttributeValue::Static(_) = &attr.value {
                                let error_span = attr.value_span.unwrap_or(attr.span);
                                errors.push(quote_spanned! { error_span =>
                                    compile_error!("Static style attributes are banned (e.g. style=\"...\"). Use style={ --prop: value } instead.");
                                });
                            }
                        }

                        if name == "class" {
                            match &attr.value {
                                token_parser::AttributeValue::Static(_) => {
                                    let error_span = attr.value_span.unwrap_or(attr.span);
                                    errors.push(quote_spanned! { error_span =>
                                        compile_error!("Static class attributes are banned (e.g. class=\"...\"). Use class={variable_name} instead.");
                                    });
                                }
                                token_parser::AttributeValue::Dynamic(tokens) => {
                                    if let Ok(ident) = syn::parse2::<syn::Ident>(tokens.clone()) {
                                        let var_name = ident.to_string();
                                        
                                        // Check if this variable is a let binding (anti-pattern)
                                        if let_bindings.contains(&var_name) {
                                            let msg = format!(
                                                "ANTI-PATTERN: class={{{{}}}} uses a @let binding.\n\
                                                \n\
                                                Variable '{}' was defined with @let, but CSS classes\n\
                                                should be defined in <style> blocks, not as @let variables.\n\
                                                \n\
                                                ✅ CORRECT:\n\
                                                    <div class={{{{}}}}>...</div>\n\
                                                    <style>\n\
                                                        .{}{{ ... }}\n\
                                                    </style>\n\
                                                \n\
                                                ❌ INCORRECT:\n\
                                                    @let {} = \"...\";\n\
                                                    <div class={{{{}}}}>...</div>\n\
                                                \n\
                                                See: AI_GUIDE_FOR_WRITING_AZUMI.md - Critical Rules",
                                                var_name, var_name, var_name, var_name, var_name, var_name
                                            );
                                            errors.push(quote_spanned! { ident.span() =>
                                                compile_error!(#msg);
                                            });
                                        } else if valid_ids.contains(&var_name)
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
                                    errors.push(quote_spanned! { attr.span =>
                                        compile_error!("Style DSL syntax { --var: val } is only allowed in 'style' attribute.");
                                    });
                                }
                                _ => {}
                            }
                        }

                        if name == "id" {
                            match &attr.value {
                                token_parser::AttributeValue::Static(_) => {
                                    let error_span = attr.value_span.unwrap_or(attr.span);
                                    errors.push(quote_spanned! { error_span =>
                                        compile_error!("Static id attributes are banned (e.g. id=\"...\"). Use id={variable_name} instead.");
                                    });
                                }
                                token_parser::AttributeValue::Dynamic(tokens) => {
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
                                    errors.push(quote_spanned! { attr.span =>
                                        compile_error!("Style DSL syntax { --var: val } is only allowed in 'style' attribute.");
                                    });
                                }
                                _ => {}
                            }
                        }

                        if let Some(err) = html_structure_validator::validate_attribute_name(attr) {
                            errors.push(err);
                        }
                    }

                    collect_errors_recursive(
                        &elem.children,
                        valid_classes,
                        valid_ids,
                        let_bindings,
                        _has_scoped_css,
                        errors,
                        _is_inside_form,
                        _is_inside_button,
                        _is_inside_anchor,
                    );
                }
                token_parser::Node::Fragment(frag) => {
                    collect_errors_recursive(
                        &frag.children,
                        valid_classes,
                        valid_ids,
                        let_bindings,
                        _has_scoped_css,
                        errors,
                        _is_inside_form,
                        _is_inside_button,
                        _is_inside_anchor,
                    );
                }
                token_parser::Node::Block(block) => match block {
                    token_parser::Block::If(if_block) => {
                        collect_errors_recursive(
                            &if_block.then_branch,
                            valid_classes,
                            valid_ids,
                            let_bindings,
                            _has_scoped_css,
                            errors,
                            _is_inside_form,
                            _is_inside_button,
                            _is_inside_anchor,
                        );
                        if let Some(else_branch) = &if_block.else_branch {
                            collect_errors_recursive(
                                else_branch,
                                valid_classes,
                                valid_ids,
                                let_bindings,
                                _has_scoped_css,
                                errors,
                                _is_inside_form,
                                _is_inside_button,
                                _is_inside_anchor,
                            );
                        }
                    }
                    token_parser::Block::For(for_block) => {
                        collect_errors_recursive(
                            &for_block.body,
                            valid_classes,
                            valid_ids,
                            let_bindings,
                            _has_scoped_css,
                            errors,
                            _is_inside_form,
                            _is_inside_button,
                            _is_inside_anchor,
                        );
                    }
                    token_parser::Block::Match(match_block) => {
                        for arm in &match_block.arms {
                            collect_errors_recursive(
                                &arm.body,
                                valid_classes,
                                valid_ids,
                                let_bindings,
                                _has_scoped_css,
                                errors,
                                _is_inside_form,
                                _is_inside_button,
                                _is_inside_anchor,
                            );
                        }
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
        &let_bindings,
        has_scoped_css,
        &mut errors,
        false,
        false,
        false,
    );

    let mut tokens = proc_macro2::TokenStream::new();
    for err in errors {
        tokens.extend(err);
    }
    tokens
}

fn generate_body_with_context(
    nodes: &[token_parser::Node],
    ctx: &GenerationContext,
) -> proc_macro2::TokenStream {
    let mut instructions = Vec::new();

    for node in nodes {
        match node {
            token_parser::Node::Text(text) => {
                let content = &text.content;
                let clean_content = strip_outer_quotes(content);
                if !clean_content.is_empty() {
                    instructions.push(quote! {
                        write!(f, "{}", #clean_content)?;
                    });
                }
            }
            token_parser::Node::Element(elem) => {
                let name = &elem.name;

                instructions.push(quote! {
                   write!(f, "<{}", #name)?;
                });

                for attr in &elem.attrs {
                    let attr_name = &attr.name;

                    if attr_name.starts_with("az-") {
                        if attr_name == "az-scope" {
                            match &attr.value {
                                token_parser::AttributeValue::Dynamic(tokens) => {
                                    instructions.push(quote! {
                                        let __scope_val: String = #tokens;
                                        let __escaped = __scope_val.replace("\"", "&quot;");
                                        write!(f, " {}=\"{}\"", #attr_name, format!("{}|DEBUG", __escaped))?;
                                    });
                                }
                                token_parser::AttributeValue::Static(val) => {
                                    let clean = strip_outer_quotes(val);
                                    instructions.push(quote! {
                                        write!(f, " {}=\"{}\"", #attr_name, #clean)?;
                                    });
                                }
                                _ => {}
                            }
                            continue;
                        }

                        match &attr.value {
                            token_parser::AttributeValue::Dynamic(tokens) => {
                                let s = tokens.to_string();
                                instructions.push(quote! {
                                    write!(f, " {}=\"{}\"", #attr_name, #s)?;
                                });
                            }
                            token_parser::AttributeValue::Static(val) => {
                                let clean = strip_outer_quotes(val);
                                instructions.push(quote! {
                                    write!(f, " {}=\"{}\"", #attr_name, #clean)?;
                                });
                            }
                            _ => {}
                        }
                        continue;
                    }

                    if attr_name.starts_with("on:") {
                        match &attr.value {
                            token_parser::AttributeValue::Dynamic(tokens) => {
                                let (s, base) =
                                    if let Ok(expr) = syn::parse2::<syn::Expr>(tokens.clone()) {
                                        match expr {
                                            syn::Expr::Field(f) => {
                                                if let syn::Member::Named(ident) = f.member {
                                                    let base = &f.base;
                                                    (ident.to_string(), Some(quote! { #base }))
                                                } else {
                                                    (tokens.to_string().replace(" ", ""), None)
                                                }
                                            }
                                            syn::Expr::Path(p) => {
                                                if let Some(ident) = p.path.get_ident() {
                                                    (ident.to_string(), None)
                                                } else {
                                                    (tokens.to_string().replace(" ", ""), None)
                                                }
                                            }
                                            syn::Expr::MethodCall(m) => {
                                                let receiver = &m.receiver;
                                                (m.method.to_string(), Some(quote! { #receiver }))
                                            }
                                            _ => (tokens.to_string().replace(" ", ""), None),
                                        }
                                    } else {
                                        (tokens.to_string().replace(" ", ""), None)
                                    };

                                let event_name = attr_name.strip_prefix("on:").unwrap_or(attr_name);
                                let dsl = format!("{} call {}", event_name, s);
                                instructions.push(quote! {
                                    write!(f, " az-on=\"{}\"", #dsl)?;
                                });

                                if let Some(base_expr) = base {
                                    instructions.push(quote! {
                                        if let Some(pred) = azumi::get_prediction(&(#base_expr), #s) {
                                            write!(f, " data-predict=\"{}\"", pred)?;
                                        }
                                    });
                                }
                            }
                            token_parser::AttributeValue::Static(val) => {
                                let clean = strip_outer_quotes(val);
                                let event_name = attr_name.strip_prefix("on:").unwrap_or(attr_name);
                                let dsl = format!("{} call {}", event_name, clean);
                                instructions.push(quote! {
                                    write!(f, " az-on=\"{}\"", #dsl)?;
                                });
                            }
                            _ => {}
                        }
                        continue;
                    }

                    if attr_name == "class" {
                        match &attr.value {
                            token_parser::AttributeValue::Static(val) => {
                                let clean = strip_outer_quotes(val);
                                instructions.push(quote! {
                                    write!(f, " class=\"{}\"", #clean)?;
                                });
                            }
                            token_parser::AttributeValue::Dynamic(tokens) => {
                                let exprs_res =
                                    syn::parse::Parser::parse2(parse_multi_exprs, tokens.clone());
                                match exprs_res {
                                    Ok(exprs) if !exprs.is_empty() => {
                                        let fmt = vec!["{}"; exprs.len()].join(" ");
                                        let mut format_args = Vec::new();
                                        for e in exprs {
                                            format_args.push(quote! { #e });
                                        }
                                        instructions.push(quote! {
                                            write!(f, " class=\"{}\"", azumi::Escaped(&format!(#fmt, #(#format_args),*)))?;
                                        });
                                    }
                                    _ => {
                                        instructions.push(quote! {
                                            write!(f, " class=\"{}\"", azumi::Escaped(&#tokens))?;
                                        });
                                    }
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }

                    if attr_name == "style" {
                        match &attr.value {
                            token_parser::AttributeValue::StyleDsl(props) => {
                                instructions.push(quote! { write!(f, " style=\"")?; });
                                for (i, (key, val)) in props.iter().enumerate() {
                                    if i > 0 {
                                        instructions.push(quote! { write!(f, "; ")?; });
                                    }
                                    instructions.push(quote! {
                                        write!(f, "{}: {}", #key, azumi::Escaped(&#val))?;
                                    });
                                }
                                instructions.push(quote! { write!(f, "\"")?; });
                            }
                            _ => match &attr.value {
                                token_parser::AttributeValue::Static(val) => {
                                    let clean = strip_outer_quotes(val);
                                    instructions.push(quote! {
                                        write!(f, " {}=\"{}\"", #attr_name, #clean)?;
                                    });
                                }
                                token_parser::AttributeValue::Dynamic(expr) => {
                                    instructions.push(quote! {
                                              write!(f, " {}=\"{}\"", #attr_name, azumi::Escaped(&#expr))?;
                                          });
                                }
                                _ => {}
                            },
                        }
                    } else {
                        match &attr.value {
                            token_parser::AttributeValue::Static(val) => {
                                let clean = strip_outer_quotes(val);
                                instructions.push(quote! {
                                    write!(f, " {}=\"{}\"", #attr_name, #clean)?;
                                });
                            }
                            token_parser::AttributeValue::Dynamic(expr) => {
                                instructions.push(quote! {
                                    write!(f, " {}=\"{}\"", #attr_name, azumi::Escaped(&#expr))?;
                                });
                            }
                            token_parser::AttributeValue::None => {
                                instructions.push(quote! {
                                    write!(f, " {}", #attr_name)?;
                                });
                            }
                            _ => {}
                        }
                    }
                }

                if let Some(sid) = &ctx.scope_id {
                    instructions.push(quote! {
                        write!(f, " data-{}", #sid)?;
                    });
                }

                instructions.push(quote! {
                   write!(f, ">")?;
                });

                let child_ctx = ctx.with_mode(if name == "script" {
                    Context::Script
                } else {
                    ctx.mode.clone()
                });
                instructions.push(generate_body_with_context(&elem.children, &child_ctx));

                let void_elements = [
                    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
                    "param", "source", "track", "wbr",
                ];
                if !void_elements.contains(&name.as_str()) {
                    instructions.push(quote! {
                        write!(f, "</{}>", #name)?;
                    });
                }
            }
            token_parser::Node::Expression(expr) => {
                let tokens = &expr.content;
                instructions.push(quote! {
                    azumi::RenderWrapper(&(#tokens)).render_azumi(f)?;
                });
            }
            token_parser::Node::Fragment(frag) => {
                instructions.push(generate_body_with_context(&frag.children, ctx));
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    let cond = &if_block.condition;
                    let then_body = generate_body_with_context(&if_block.then_branch, ctx);
                    let else_part = if let Some(else_branch) = &if_block.else_branch {
                        let else_body = generate_body_with_context(else_branch, ctx);
                        quote! { else { #else_body } }
                    } else {
                        quote! {}
                    };

                    instructions.push(quote! {
                        if #cond {
                            #then_body
                        } #else_part
                    });
                }
                token_parser::Block::For(for_block) => {
                    let pat = &for_block.pattern;
                    let iter = &for_block.iterator;
                    let body = generate_body_with_context(&for_block.body, ctx);

                    instructions.push(quote! {
                        for #pat in #iter {
                            #body
                        }
                    });
                }
                token_parser::Block::Match(match_block) => {
                    let expr = &match_block.expr;
                    let mut arms = Vec::new();
                    for arm in &match_block.arms {
                        let pat = &arm.pattern;
                        let body = generate_body_with_context(&arm.body, ctx);
                        arms.push(quote! {
                            #pat => { #body }
                        });
                    }
                    instructions.push(quote! {
                        match #expr {
                            #(#arms),*
                        }
                    });
                }
                token_parser::Block::Component(comp_block) => {
                    let func_path = &comp_block.name;
                    let func_mod_path = transform_path_for_component(func_path);

                    instructions.push(quote! {
                        #func_mod_path::render(
                            #func_mod_path::Props::builder().build().expect("Failed to build props")
                        ).render(f)?;
                    });
                }
                token_parser::Block::Call(call_block) => {
                    let func_path = &call_block.name;
                    let func_mod_path = transform_path_for_component(func_path);

                    let args_list = match parse_args(call_block.args.clone()) {
                        Ok(a) => a,
                        Err(e) => {
                            instructions.push(e.to_compile_error());
                            Vec::new()
                        }
                    };

                    let setters = args_list.iter().map(|arg| {
                        let key = &arg.key;
                        let val = &arg.value;
                        quote! { .#key(#val) }
                    });

                    let builder_expr = quote! {
                        #func_mod_path::Props::builder()
                        #(#setters)*
                        .build()
                        .expect("Failed to build props")
                    };

                    if call_block.children.is_empty() {
                        instructions.push(quote! {
                            #func_mod_path::render(#builder_expr).render(f)?;
                        });
                    } else {
                        let children_body = generate_body_with_context(&call_block.children, ctx);
                        let children_arg = quote! {
                            azumi::from_fn(|f| {
                                #children_body
                                Ok(())
                            })
                        };

                        instructions.push(quote! {
                            #func_mod_path::render(#builder_expr, #children_arg).render(f)?;
                        });
                    }
                }
                token_parser::Block::Let(let_block) => {
                    let pat = &let_block.pattern;
                    let val = &let_block.value;
                    instructions.push(quote! {
                        let #pat = #val;
                    });
                }
                token_parser::Block::Style(_) => {}
            },
            _ => {}
        }
    }

    quote! {
        #(#instructions)*
    }
}

// Helper: try_extract_template implementation
fn try_extract_template(
    nodes: &[token_parser::Node],
) -> Option<(Vec<String>, Vec<proc_macro2::TokenStream>)> {
    let mut statics = Vec::new();
    let mut dynamics = Vec::new();
    let mut current_static = String::new();

    if !extract_recursive(nodes, &mut current_static, &mut statics, &mut dynamics) {
        return None;
    }
    statics.push(current_static);
    Some((statics, dynamics))
}

fn extract_recursive(
    nodes: &[token_parser::Node],
    current_static: &mut String,
    statics: &mut Vec<String>,
    dynamics: &mut Vec<proc_macro2::TokenStream>,
) -> bool {
    for node in nodes {
        match node {
            token_parser::Node::Text(text) => {
                current_static.push_str(&strip_outer_quotes(&text.content));
            }
            token_parser::Node::Element(elem) => {
                current_static.push('<');
                current_static.push_str(&elem.name);

                for attr in &elem.attrs {
                    match &attr.value {
                        token_parser::AttributeValue::Static(val) => {
                            current_static.push(' ');
                            current_static.push_str(&attr.name);
                            current_static.push_str("=\"");
                            current_static.push_str(&strip_outer_quotes(val));
                            current_static.push('"');
                        }
                        _ => return false,
                    }
                }
                current_static.push('>');
                if !extract_recursive(&elem.children, current_static, statics, dynamics) {
                    return false;
                }
                current_static.push_str("</");
                current_static.push_str(&elem.name);
                current_static.push('>');
            }
            token_parser::Node::Expression(expr) => {
                statics.push(current_static.clone());
                current_static.clear();
                dynamics.push(expr.content.clone());
            }
            token_parser::Node::Fragment(frag) => {
                if !extract_recursive(&frag.children, current_static, statics, dynamics) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}
