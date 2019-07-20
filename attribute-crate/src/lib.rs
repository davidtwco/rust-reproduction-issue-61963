extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn dom_struct(_: TokenStream, _: TokenStream) -> TokenStream {
    // This is hardcoded for the test case, the space before the '>' is significant.
    "#[derive(DomObject)] struct Foo {
        qux: Qux<Qux<Baz> >,
        bar: Box<Bar>,
    }".parse().unwrap()
}
