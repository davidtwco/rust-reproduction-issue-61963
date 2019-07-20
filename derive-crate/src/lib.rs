extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(DomObject)]
pub fn expand_token_stream(input: TokenStream) -> TokenStream {
    let dummy_span = input.into_iter().nth(0).unwrap().span();

    // set-up the expanded output, just like the original crate would in this example
    let static_source: TokenStream = "impl Bar for ((), Qux<Qux<Baz> >) {}
                                      impl Bar for ((), Box<Bar>) {}".parse().unwrap();

    let mut tokens: Vec<_> = static_source.into_iter().collect();

    // set-up tokens' spans just like `quote` would in this example
    for group in tokens.iter_mut() {
        if let TokenTree::Group(group) = group {
            let mut tokens: Vec<_> = group.stream().into_iter().collect();
            for token in tokens.iter_mut().skip(2) {
                token.set_span(dummy_span);
            }

            let mut stream = proc_macro::TokenStream::new();
            stream.extend(tokens);

            *group = proc_macro::Group::new(group.delimiter(), stream);
        }
    }
    
    let mut stream = proc_macro::TokenStream::new();
    stream.extend(tokens);
    stream
}
