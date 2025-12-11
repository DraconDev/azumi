use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn};

pub fn expand_action(item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_async = &input_fn.sig.asyncness;
    let fn_args = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;

    // We need to separate arguments into:
    // 1. Axum extractors (State, Extension, etc.)
    // 2. The payload argument (the last one, usually)

    // For MVP, we'll assume the LAST argument is the payload if there are arguments.
    // If there are no arguments, there is no payload.

    let mut extractors = Vec::new();
    let mut payload_arg = None;

    let args_len = fn_args.len();

    for (i, arg) in fn_args.iter().enumerate() {
        if let FnArg::Typed(pat_type) = arg {
            if i == args_len - 1 && args_len > 0 {
                // Check if it looks like an extractor (simple heuristic for now)
                // If it's `State(...)` or `Extension(...)`, it's an extractor.
                // But for now, let's assume the user follows the convention:
                // fn action(extractors..., payload: Payload)

                // Actually, a better heuristic might be: if it's a primitive or a struct that implements Deserialize, it's payload.
                // But we can't check traits in proc macro.

                // Let's assume the last argument is payload.
                payload_arg = Some(pat_type);
            } else {
                extractors.push(pat_type);
            }
        }
    }

    // If no args, payload is empty tuple

    let wrapper_name = quote::format_ident!("{}_handler", fn_name);

    let (payload_type, _payload_pat) = if let Some(arg) = payload_arg {
        (&arg.ty, &arg.pat)
    } else {
        // No payload
        return quote! {
            #(#fn_attrs)*
            #fn_vis #fn_async fn #fn_name(#fn_args) #fn_output {
                #fn_block
            }
        }
        .into();
    };

    // Reconstruct args for the wrapper
    // Extractors are passed through
    // Payload is wrapped in Json<...>

    let extractor_args = extractors.iter().map(|arg| {
        quote! { #arg }
    });

    let call_args = extractors.iter().map(|arg| {
        let pat = &arg.pat;
        quote! { #pat }
    });

    // Helper to return MethodRouter
    // We need a unique name for this too
    let router_helper_name = quote::format_ident!("{}_router", fn_name);

    let expanded = quote! {
        // Original function (modified to be called by wrapper if needed, or just keep it)
        // Actually, we want to replace it or generate a wrapper alongside it.
        // The user calls `action!(fn_name)` in HTML, which resolves to the route string.
        // But the router needs the handler.

        // Let's keep the original function as is, but maybe make it pub so the wrapper can call it?
        // Or generate the wrapper inside the same module.

        #(#fn_attrs)*
        #fn_vis #fn_async fn #fn_name(#fn_args) #fn_output {
            #fn_block
        }

        // Generated Axum handler
        // We use a different name so we don't conflict
        pub async fn #wrapper_name(
            #(#extractor_args,)*
            axum::extract::Json(payload): axum::extract::Json<#payload_type>
        ) -> impl axum::response::IntoResponse {
            // Call the original function
            // We need to match the arguments.
            // The payload pattern `payload` matches the Json body.
            // But the original function expects `payload_pat` (e.g. `id`).
            // So we need to destructure or pass it.

            // Wait, if the argument is `id: i32`, `payload` will be `i32`.
            // If the argument is `data: MyStruct`, `payload` will be `MyStruct`.
            // So we can just pass `payload` as the last argument.

            let result = #fn_name(#(#call_args,)* payload).await;
            azumi::render_to_string(&result)
        }

        // Helper to return MethodRouter
        #[allow(non_snake_case)]
        pub fn #router_helper_name() -> axum::routing::MethodRouter<()> {
            axum::routing::post(#wrapper_name)
        }

        // Auto-registration using inventory
        // We need to ensure this runs.
        azumi::inventory::submit! {
            azumi::action::ActionEntry {
                path: concat!("/_azumi/action/", stringify!(#fn_name)),
                handler: #router_helper_name,
            }
        }
    };

    TokenStream::from(expanded)
}
