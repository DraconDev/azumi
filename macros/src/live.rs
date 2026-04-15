//! Azumi Live - Compiler-Driven Optimistic UI
//!
//! This module implements the `#[azumi::live]` macro that:
//! 1. Parses struct fields to understand state shape
//! 2. Analyzes impl block methods for predictable mutations
//! 3. Generates prediction metadata for client-side optimistic updates
//! 4. Auto-registers server action handlers

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, BinOp, Expr, ExprAssign, ExprBinary, ExprField, ExprMethodCall, ExprPath,
    ExprUnary, Fields, ImplItem, ImplItemFn, ItemImpl, ItemStruct, Member, Stmt, UnOp,
};

/// Represents a predictable mutation that can be executed optimistically
#[derive(Debug, Clone)]
pub enum Prediction {
    /// self.field = literal (e.g., self.open = true)
    SetLiteral { field: String, value: String },
    /// self.field = !self.field (toggle)
    Toggle { field: String },
    /// self.field += value (increment)
    Add { field: String, value: String },
    /// self.field -= value (decrement)
    Sub { field: String, value: String },
    /// Manual prediction string from #[azumi::predict]
    Manual(String),
}

impl Prediction {
    /// Convert to DSL string for data-predict attribute
    pub fn to_dsl(&self) -> String {
        match self {
            Prediction::SetLiteral { field, value } => {
                format!("{} = {}", field, value)
            }
            Prediction::Toggle { field } => {
                format!("{} = !{}", field, field)
            }
            Prediction::Add { field, value } => {
                format!("{} = {} + {}", field, field, value)
            }
            Prediction::Sub { field, value } => {
                format!("{} = {} - {}", field, field, value)
            }
            Prediction::Manual(s) => s.clone(),
        }
    }
}

/// Metadata about an analyzed method
#[derive(Debug)]
pub struct MethodAnalysis {
    #[allow(dead_code)]
    pub name: String,
    pub predictions: Vec<Prediction>,
    #[allow(dead_code)]
    pub has_unpredictable: bool,
}

/// Extract field name from `self.field` expression
fn extract_self_field(expr: &Expr) -> Option<String> {
    if let Expr::Field(ExprField { base, member, .. }) = expr {
        // Check if base is `self`
        if let Expr::Path(ExprPath { path, .. }) = &**base {
            if path.is_ident("self") {
                if let Member::Named(ident) = member {
                    return Some(ident.to_string());
                }
            }
        }
    }
    None
}

/// Check if expression is a simple literal we can predict
fn expr_to_literal_string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(lit) => Some(quote!(#lit).to_string()),
        Expr::Path(path) => {
            // Handle true/false as paths
            if path.path.is_ident("true") || path.path.is_ident("false") {
                Some(path.path.get_ident()?.to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Check if expression is `!self.field`
fn is_toggle_expr(expr: &Expr, expected_field: &str) -> bool {
    if let Expr::Unary(ExprUnary {
        op: UnOp::Not(_),
        expr,
        ..
    }) = expr
    {
        if let Some(field) = extract_self_field(expr) {
            return field == expected_field;
        }
    }
    false
}

/// Analyze a single statement for predictable mutations
fn analyze_statement(stmt: &Stmt) -> Option<Prediction> {
    match stmt {
        Stmt::Expr(expr, _semicolon) => analyze_expr(expr),
        _ => None,
    }
}

/// Analyze an expression for predictable mutations
fn analyze_expr(expr: &Expr) -> Option<Prediction> {
    match expr {
        // self.field = value
        Expr::Assign(ExprAssign { left, right, .. }) => {
            let field = extract_self_field(left)?;

            // Check for toggle: self.field = !self.field
            if is_toggle_expr(right, &field) {
                return Some(Prediction::Toggle { field });
            }

            // Check for literal assignment
            if let Some(value) = expr_to_literal_string(right) {
                return Some(Prediction::SetLiteral { field, value });
            }

            None
        }

        // self.field += value or self.field -= value
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => {
            let field = extract_self_field(left)?;
            let value = expr_to_literal_string(right)?;

            match op {
                BinOp::AddAssign(_) => Some(Prediction::Add { field, value }),
                BinOp::SubAssign(_) => Some(Prediction::Sub { field, value }),
                _ => None,
            }
        }

        _ => None,
    }
}

/// Analyze a method body for all predictable mutations
pub fn analyze_method(method: &ImplItemFn) -> MethodAnalysis {
    let name = method.sig.ident.to_string();
    let mut predictions = Vec::new();
    let mut has_unpredictable = false;

    // Check for #[azumi::predict("...")] attribute
    for attr in &method.attrs {
        if attr.path().is_ident("predict") {
            if let Ok(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit),
                ..
            })) = attr.parse_args()
            {
                predictions.push(Prediction::Manual(lit.value()));
            }
        }
    }

    for stmt in &method.block.stmts {
        if let Some(prediction) = analyze_statement(stmt) {
            predictions.push(prediction);
        } else {
            // Check if this is a statement that could have side effects
            match stmt {
                Stmt::Expr(expr, _semicolon) => {
                    if is_side_effect(expr) {
                        has_unpredictable = true;
                    }
                }
                Stmt::Local(_) => {
                    // Local variable bindings are fine
                }
                _ => {
                    has_unpredictable = true;
                }
            }
        }
    }

    MethodAnalysis {
        name,
        predictions,
        has_unpredictable,
    }
}

/// Check if an expression likely has side effects (async, await, method calls, etc.)
fn is_side_effect(expr: &Expr) -> bool {
    match expr {
        Expr::Await(_) => true,
        Expr::Call(_) => true, // Function calls might have side effects
        Expr::MethodCall(mc) => {
            // self.field mutations are handled separately
            // External method calls are side effects
            !is_self_field_mutation(mc)
        }
        Expr::Assign(_) => false, // Assignments to self are fine
        Expr::Macro(_) => true,   // Macros are unpredictable
        _ => false,
    }
}

fn is_self_field_mutation(_mc: &ExprMethodCall) -> bool {
    // Check if this is something like self.field.push() which we can't predict
    false
}

/// Main macro expansion for #[azumi::live]
pub fn expand_live(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let struct_vis = &input.vis;
    let struct_generics = &input.generics;
    let struct_fields = &input.fields;

    // Extract field information
    let _field_info: Vec<_> = match struct_fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let name = f.ident.as_ref().unwrap();
                let ty = &f.ty;
                (name.clone(), ty.clone())
            })
            .collect(),
        _ => {
            return syn::Error::new_spanned(
                &input,
                "#[azumi::live] only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    let struct_attrs = &input.attrs;

    // Generate the struct with derives
    let expanded = quote! {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        #(#struct_attrs)*
        #struct_vis struct #struct_name #struct_generics #struct_fields

        impl #struct_generics #struct_name #struct_generics {
            /// Serialize state for az-scope attribute
            pub fn to_scope(&self) -> String {
                let json = serde_json::to_string(self).unwrap_or_default();
                azumi::security::sign_state(&json)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Attribute macro for impl blocks: #[azumi::live_impl]
/// This analyzes methods and generates action handlers with predictions
pub fn expand_live_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = &input.self_ty;
    // Get struct name as string for namespacing
    let struct_name_str = quote!(#struct_name).to_string();
    // Clean up struct name (remove spaces)
    let struct_name_str = struct_name_str.replace(" ", "");

    // Parse attributes to find component="name"
    let args = parse_macro_input!(attr with syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated);
    let mut component_name = None;

    for arg in args {
        if let syn::Meta::NameValue(nv) = arg {
            if nv.path.is_ident("component") {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = nv.value
                {
                    component_name = Some(lit.value());
                }
            }
        }
    }

    let mut method_handlers = Vec::new();
    let mut original_methods = Vec::new();

    let mut predictions_entries = Vec::new();

    for item in &input.items {
        if let ImplItem::Fn(method) = item {
            let analysis = analyze_method(method);

            // Generate prediction string
            let prediction_dsl: String = analysis
                .predictions
                .iter()
                .map(|p| p.to_dsl())
                .collect::<Vec<_>>()
                .join("; ");

            let method_name = &method.sig.ident;
            let method_name_str = method_name.to_string();

            if !prediction_dsl.is_empty() {
                predictions_entries.push(quote! {
                    (#method_name_str, #prediction_dsl)
                });
            }

            let handler_name = format_ident!("{}_handler", method_name);
            let router_name = format_ident!("{}_router", method_name);

            // Keep original method
            original_methods.push(quote! { #method });

            let is_async = method.sig.asyncness.is_some();
            let method_call = if is_async {
                quote! { state.#method_name().await; }
            } else {
                quote! { state.#method_name(); }
            };

            // Generate Axum handler
            let handler = if let Some(comp_name) = &component_name {
                let comp_mod = format_ident!("{}_component", comp_name);
                quote! {
                    pub async fn #handler_name(
                        body: String
                    ) -> axum::response::Response {
                        let json = match azumi::security::verify_state(&body) {
                            Ok(j) => j,
                            Err(e) => return axum::response::IntoResponse::into_response((axum::http::StatusCode::BAD_REQUEST, format!("Security Error: {}", e))),
                        };
                        let mut state: #struct_name = serde_json::from_str(&json).unwrap();
                        #method_call

                        // Re-render the component with new state
                        let html = azumi::render_to_string(&#comp_mod::render(
                            #comp_mod::Props::builder()
                                .state(&state)
                                .build()
                                .expect("Live component re-render failed")
                        ));

                        axum::response::IntoResponse::into_response(axum::response::Html(html))
                    }

                    #[allow(non_snake_case)]
                    pub fn #router_name() -> axum::routing::MethodRouter<()> {
                        axum::routing::post(#handler_name)
                    }
                }
            } else {
                quote! {
                    pub async fn #handler_name(
                        body: String
                    ) -> axum::response::Response {
                        let json = match azumi::security::verify_state(&body) {
                            Ok(j) => j,
                            Err(e) => return axum::response::IntoResponse::into_response((axum::http::StatusCode::BAD_REQUEST, format!("Security Error: {}", e))),
                        };
                        let mut state: #struct_name = serde_json::from_str(&json).unwrap();
                        #method_call
                        axum::response::IntoResponse::into_response(axum::response::Json(state))
                    }

                    #[allow(non_snake_case)]
                    pub fn #router_name() -> axum::routing::MethodRouter<()> {
                        axum::routing::post(#handler_name)
                    }
                }
            };

            method_handlers.push(handler);

            // Generate inventory registration with NAMESPACED path
            // /_azumi/action/{StructName}/{MethodName}
            let action_path = format!("/_azumi/action/{}/{}", struct_name_str, method_name);
            let registration = quote! {
                azumi::inventory::submit! {
                    azumi::action::ActionEntry {
                        path: #action_path,
                        handler: #router_name,
                    }
                }
            };
            method_handlers.push(registration);
        }
    }

    let expanded = quote! {
        impl #struct_name {
            #(#original_methods)*
        }

        impl azumi::LiveState for #struct_name {
            fn to_scope(&self) -> String {
                self.to_scope()
            }
            fn predictions() -> &'static [(&'static str, &'static str)] {
                &[
                    #(#predictions_entries),*
                ]
            }
            fn struct_name() -> &'static str {
                #struct_name_str
            }
        }

        // Generated handlers module
        #[allow(non_snake_case)]
        mod __azumi_live_handlers {
            use super::*;
            #(#method_handlers)*
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use crate::live::Prediction;

    #[test]
    fn test_prediction_to_dsl() {
        let toggle = Prediction::Toggle {
            field: "open".to_string(),
        };
        assert_eq!(toggle.to_dsl(), "open = !open");

        let add = Prediction::Add {
            field: "count".to_string(),
            value: "1".to_string(),
        };
        assert_eq!(add.to_dsl(), "count = count + 1");

        let set = Prediction::SetLiteral {
            field: "name".to_string(),
            value: "\"hello\"".to_string(),
        };
        assert_eq!(set.to_dsl(), "name = \"hello\"");
    }
}
