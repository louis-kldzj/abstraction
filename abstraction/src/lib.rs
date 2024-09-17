extern crate proc_macro;

use proc_macro::TokenStream;

mod attribute;
mod derive;

#[proc_macro_attribute]
pub fn abstraction(_attr: TokenStream, item: TokenStream) -> TokenStream {
    attribute::impl_abstraction(_attr, item)
}

#[proc_macro_derive(Abstract, attributes(abstract_traits))]
pub fn abstract_derive(input: TokenStream) -> TokenStream {
    derive::impl_abstraction(input)
}
