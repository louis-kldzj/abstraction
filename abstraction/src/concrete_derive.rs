use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Path, Token};

pub fn impl_concrete_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let input_struct = match &derive_input.data {
        Data::Struct(st) => st,
        _ => return TokenStream::from(quote! { compile_error!("Concrete must be a struct")}),
    };
    let trait_attribute = derive_input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("concrete_traits"))
        .unwrap();

    let trait_paths = trait_attribute
        .parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
        .unwrap()
        .into_iter()
        .map(|p| p)
        .collect::<Vec<_>>();

    let struct_name = derive_input.ident;
    let traits = trait_paths.iter().map(|trait_path| {
        let trait_name = trait_path.get_ident().unwrap();
        let data_field = &input_struct
            .fields
            .iter()
            .find(|f| {
                f.attrs.iter().any(|a| {
                    a.path().is_ident("data_field")
                        && a.parse_nested_meta(|m| {
                            if m.path.is_ident(trait_name) {
                                Ok(())
                            } else {
                                Err(m.error("AHH"))
                            }
                        })
                        .is_ok()
                })
            })
            .unwrap()
            .ident
            .clone()
            .unwrap();

        quote! {
            impl #trait_name for #struct_name {
                fn instance(&self) -> &dyn #trait_name {
                    &self.#data_field
                }
            }
        }
    });

    TokenStream::from(quote! {

        #(#traits)*

    })
}
