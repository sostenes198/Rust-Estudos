use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::Expr;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let args = parser.parse(attr.into()).expect("Erro ao fazer parse dos argumentos");
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_sig = &input_fn.sig;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;

    let method = if let Some(expr) = args.first() {
        if let Expr::Path(path_expr) = expr {
            path_expr.path.segments.first().unwrap().ident.to_string()
        } else if let Expr::Lit(expr_lit) = expr {
            if let syn::Lit::Str(s) = &expr_lit.lit {
                s.value()
            } else {
                "UNKNOWN".to_string()
            }
        } else {
            "UNKNOWN".to_string()
        }
    } else {
        "UNKNOWN".to_string()
    };

    let expanded = quote! {
        #fn_vis #fn_sig {
            println!("[{}] Rota chamada: {}", #method, stringify!(#fn_name));
            #fn_block
        }
    };

    TokenStream::from(expanded)
}