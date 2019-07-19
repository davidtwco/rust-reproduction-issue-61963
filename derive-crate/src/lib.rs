extern crate proc_macro;
#[macro_use]
extern crate quote;

use quote::TokenStreamExt;

#[proc_macro_derive(DomObject)]
pub fn expand_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let fields = if let syn::Data::Struct(syn::DataStruct { ref fields, .. }) = input.data {
        fields.iter().collect::<Vec<&syn::Field>>()
    } else {
        panic!("#[derive(DomObject)] should only be applied on proper structs")
    };

    let mut field_types = vec![];
    for field in fields {
        field_types.push(&field.ty);
    }

    let mut items = quote! {};
    items.append_all(field_types.iter().map(|ty| {
        quote! {
            impl Bar for ((), #ty) {}
        }
    }));

    let x = quote! { #items };
    x.into()
}
