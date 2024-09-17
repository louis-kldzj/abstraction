use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, Fields, ItemEnum, Path};

pub fn impl_abstraction(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_path = parse_macro_input!(_attr as Path);

    let input = parse_macro_input!(item as ItemEnum);

    eprintln!(
        "Input: {:#?} - {:#?}",
        trait_path.to_token_stream(),
        input.variants.to_token_stream()
    );

    let variants = input.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
                quote! {
                    Self::#variant_name(inner) => inner.instance(),
                }
            }
            _ => {
                quote! {
                    _ => panic!("Cannot derive {} for variant {}", stringify!(#trait_path), stringify!(#variant_name)),
                }
            }
        }
    });

    let enum_name = input.ident;
    let trait_name = trait_path.get_ident().unwrap();

    eprintln!(
        "Variants: {:#?}",
        variants
            .clone()
            .map(|v| v.to_token_stream().to_string())
            .collect::<Vec<_>>()
    );

    let expanded = quote! {

        impl #trait_name for #enum_name {
            fn instance(&self) -> &dyn #trait_name {
                match self {
                    #(#variants)*
                }
            }
        }
    };

    eprintln!("Result: {:#?}", &expanded.to_string());

    TokenStream::from(expanded)
}

fn get_trait_name(attrs: &[Attribute], attr_name: &str) -> Result<Path, TokenStream> {
    let mut path = Err(TokenStream::from(quote! {
        compile_error!("Attribute {} not found", stringify!(#attr_name));
    }));
    for attr in attrs {
        if attr.path().is_ident(attr_name) {
            match attr.parse_nested_meta(|m| {
                path = Ok(m.path.clone());
                return Ok(());
            }) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    path
}
