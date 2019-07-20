extern crate proc_macro;

use proc_macro::{Group, TokenStream, TokenTree};

#[proc_macro_derive(DomObject)]
pub fn expand_token_stream(input: TokenStream) -> TokenStream {
    // this DUMMY_SP span, `#0 bytes(0..0)`, is present in the input because
    // of the specially crafted generated tokens in the `attribute-crate` proc-macro
    let dummy_span = input.into_iter().nth(0).unwrap().span();

    // set-up the expanded output, with the piece of code triggering the warning in the original crate
    let static_source: TokenStream = "impl Bar for ((), Box<Bar>) {}".parse().unwrap();
    let mut tokens: Vec<_> = static_source.into_iter().collect();

    // set-up token spans just like the original crate would using `quote` in this example: with some
    // of the generated tokens pointing to DUMMY_SP
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
