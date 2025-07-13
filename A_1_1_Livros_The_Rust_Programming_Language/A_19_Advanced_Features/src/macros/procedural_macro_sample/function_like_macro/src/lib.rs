use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// A macro sql! que aceita uma string SQL como entrada.
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // Parse da entrada como uma string literal
    let query = parse_macro_input!(input as LitStr);

    // Aqui você pode manipular a string de várias formas, se necessário.
    // Para este exemplo, apenas vamos imprimir a consulta.

    let expanded = quote! {
        // O código gerado pela macro vai imprimir a string SQL.
        println!("Consultando SQL: {}", #query);
    };

    TokenStream::from(expanded)
}