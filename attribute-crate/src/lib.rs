extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro_attribute]
pub fn dom_struct(_: TokenStream, input: TokenStream) -> TokenStream {
    let attributes: TokenStream =
        "#[derive(DomObject)]".to_string().parse().unwrap();
    let output: TokenStream = attributes.into_iter()
        .chain(input.into_iter()).collect();

    let item: Item = syn::parse(output).unwrap();
    quote!(#item).into()
}
