use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, ItemFn, Lit};

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
        quote! { Some(#description) }
    };

    // 3. Generate Wrapper
    // We name the inner function `_inner_{name}`
    let inner_name = format_ident!("_inner_{}", fn_name);

    // We need to strip the #[azumi::page] attribute from the inner function to avoid recursion
    // The macro input doesn't have the attribute itself usually, but we should be clean.
    // Also strip other attributes that might be problematic if duplicated? No, keep them on inner.
    // Actually, `azumi::component` handles logic. If the user put `#[azumi::page]` INSTEAD OF `#[azumi::component]`,
    // we need to make sure `#[azumi::component]` is applied to the inner function!
    // BUT, usually the user might write:
    // #[azumi::page]
    // pub fn foo() ...
    // So we should generate:
    // #[azumi::component]
    // fn _inner_foo() ...
    // #[azumi::component]
    // pub fn foo() ...

    let expanded = quote! {
        // Inner implementation (The actual UI)
        #[azumi::component]
        fn #inner_name #fn_sig {
            #fn_block
        }

        // Public Wrapper (The SEO injector)
        #[azumi::component]
        #fn_vis fn #fn_name() -> impl azumi::Component {
            // Check if feature enabled, otherwise this is a no-op wrapper
            #[cfg(feature = "seo")]
            {
                let _seo_head = azumi::seo::generate_head(
                    #title,
                    #desc_tokens,
                    None // Image TODO: Extract from some other attribute?
                );

                azumi::html! {
                    // Inject the generated head first
                    {_seo_head}
                    // Render the inner component
                    @#inner_name()
                }
            }

            #[cfg(not(feature = "seo"))]
            {
                azumi::html! {
                    @#inner_name()
                }
            }
        }
    };

    TokenStream::from(expanded)
}
