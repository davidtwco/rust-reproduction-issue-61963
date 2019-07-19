#![deny(bare_trait_objects)]

#[macro_use]
extern crate attribute_crate;
#[macro_use]
extern crate derive_crate;

pub struct Baz;
pub trait Bar { }
pub struct Qux<T>(T);

#[dom_struct]
pub struct Foo {
    qux: Qux<Qux<Baz>>,
    bar: Box<Bar>,
}

fn main() {}
