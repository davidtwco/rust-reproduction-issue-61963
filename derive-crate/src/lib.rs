extern crate proc_macro;

use proc_macro::{Group, TokenStream, TokenTree};

#[proc_macro_derive(DomObject)]
pub fn expand_token_stream(input: TokenStream) -> TokenStream {
    let dummy_span = input.into_iter().nth(0).unwrap().span();

    // set-up the expanded output, with the piece of code triggering the warning in the original crate
    let static_source: TokenStream = "impl Bar for ((), Box<Bar>) {}".parse().unwrap();
    let mut tokens: Vec<_> = static_source.into_iter().collect();

    // set-up token spans just like `quote` would in this example
    for token in tokens.iter_mut() {
        if let TokenTree::Group(group) = token {
            let mut tokens: Vec<_> = group.stream().into_iter().collect();
            for token in tokens.iter_mut().skip(2) {
                token.set_span(dummy_span);
            }

            let mut stream = TokenStream::new();
            stream.extend(tokens);

            *group = Group::new(group.delimiter(), stream);
        }
    }
    
    let mut stream = TokenStream::new();
    stream.extend(tokens);
    stream
}
