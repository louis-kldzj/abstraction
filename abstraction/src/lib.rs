extern crate proc_macro;

use proc_macro::TokenStream;

mod abstract_derive;
mod concrete_derive;

#[proc_macro_derive(Abstract, attributes(abstract_traits))]
pub fn abstract_derive(input: TokenStream) -> TokenStream {
    abstract_derive::impl_abstraction(input)
}

#[proc_macro_derive(Concrete, attributes(concrete_traits, data_field))]
pub fn concrete_derive(input: TokenStream) -> TokenStream {
    concrete_derive::impl_concrete_derive(input)
}
