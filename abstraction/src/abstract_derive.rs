use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, Data, DataEnum, DeriveInput, Fields, Ident, Path, Token};

pub(super) fn impl_abstraction(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let trait_attribute = derive_input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("abstract_traits"))
        .unwrap();

    let trait_paths = trait_attribute
        .parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
        .unwrap()
        .into_iter()
        .map(|p| p)
        .collect::<Vec<_>>();

    match &derive_input.data {
        Data::Enum(enum_data) => impl_abstraction_enum(&derive_input.ident, enum_data, trait_paths),
        _ => todo!(),
    }
}

fn impl_abstraction_enum(enum_name: &Ident, enum_data: &DataEnum, trait_paths: Vec<Path>) -> TokenStream {
    
    let traits = trait_paths.into_iter().map(|trait_path| {

        let trait_name = trait_path.get_ident().unwrap();
        let fn_name = format_ident!("{}_instance", trait_name.to_string().to_lowercase());

        let variants = enum_data.variants.iter().map(|variant| {
            let name = &variant.ident;
            match &variant.fields {
                Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
                    quote! {
                        Self::#name(inner) => (inner as &dyn #trait_name).#fn_name(),
                    }
                }
                _ => {
                    quote! {
                        _ => panic!("Cannot derive {} for variant {}", stringify!(#trait_path), stringify!(#name)),
                    }
                }
            }            
        });


        quote! {

            impl #trait_name for #enum_name {
                fn #fn_name(&self) -> &dyn #trait_name {
                    match self {
                        #(#variants)*
                    }
                }
            }
        }

    });

    let expanded = quote! {

        #(#traits)*

    };

    eprintln!("{:#}", expanded.to_string());

    TokenStream::from(expanded)
}
