use proc_macro::TokenStream;
use quote::{format_ident, quote};
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
        let fn_name = format_ident!("{}_instance", trait_name.to_string().to_lowercase());
        let fn_mut_name = format_ident!("{}_instance_mut", trait_name.to_string().to_lowercase());
        let (data_field, data_field_mut) = match &input_struct.fields.iter().find(|f| {
            f.attrs.iter().any(|a| {
                a.path().is_ident("data_field")
                    && a.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
                        .unwrap()
                        .iter()
                        .find(|m| if m.is_ident(trait_name) { true } else { false })
                        .is_some()
            })
        }) {
            Some(f) => {
                let field_name = f.ident.clone().unwrap();
                (
                    quote! { &self.#field_name },
                    quote! { &mut self.#field_name },
                )
            }
            None => (quote! { self }, quote! {self }),
        };

        quote! {
            impl #trait_name for #struct_name {
                fn #fn_name(&self) -> &dyn #trait_name {
                    #data_field
                }

                fn #fn_mut_name(&mut self) -> &mut dyn #trait_name {
                    #data_field_mut
                }
            }
        }
    });

    TokenStream::from(quote! {

        #(#traits)*

    })
}
