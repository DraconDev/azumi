use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, Lit};

pub fn expand_page(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_block = &input.block;
    let fn_sig = &input.sig;

    // 1. Infer Title from Function Name
    // lesson_9 -> "Lesson 9"
    let name_str = fn_name.to_string();
    let title = name_str
        .split('_')
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    // 2. Infer Description from Doc Comments
    // /// This is a description
    let mut description = String::new();
    for attr in &input.attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Str(s), ..
                }) = &meta.value
                {
                    let val = s.value();
                    let trimmed = val.trim();
                    if !description.is_empty() {
                        description.push(' ');
                    }
                    description.push_str(trimmed);
                }
            }
        }
    }

    let desc_tokens = if description.is_empty() {
        quote! { None }
    } else {
        quote! { Some(#description.to_string()) }
    };

    // 3. Generate Wrapper
    let inner_name = format_ident!("_inner_{}", fn_name);
    let mut inner_sig = fn_sig.clone();
    inner_sig.ident = inner_name.clone();

    let expanded = quote! {
        // Inner implementation
        #[azumi::component]
        #inner_sig {
            #fn_block
        }

        // Public Wrapper
        #[azumi::component]
        #fn_vis fn #fn_name() -> impl azumi::Component {
            // Set context for Layouts to find
            azumi::context::set_page_meta(
                Some(#title.to_string()),
                #desc_tokens,
                None
            );

            // Render inner (which calls Layout, which calls seo::render_automatic_seo)
            azumi::html! {
                @#inner_name()
            }
        }
    };

    TokenStream::from(expanded)
}
