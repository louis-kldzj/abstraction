extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Path};

mod data;
#[cfg(test)]
mod test_model;

#[proc_macro_attribute]
pub fn abstraction(_attr: TokenStream, item: TokenStream) -> TokenStream {
    data::impl_abstraction(_attr, item)
}

#[proc_macro_derive(Concrete, attributes(concrete_trait, data_struct))]
pub fn derive_concrete(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the trait name from the helper attribute
    let trait_name = match get_trait_name(&input.attrs, "concrete_trait") {
        Ok(trait_name) => trait_name,
        Err(token_stream) => return token_stream,
    };

    let data_name = match get_trait_name(&input.attrs, "data_struct") {
        Ok(data_name) => data_name,
        Err(token_stream) => return token_stream,
    };

    // Generate the implementation
    match &input.data {
        Data::Struct(data_struct) => {
            impl_concrete_trait(&input.ident, data_struct, &trait_name, &data_name)
        }
        _ => TokenStream::from(quote! {
            compile_error!("Concrete can only be derived for structs");
        }),
    }
}

fn impl_concrete_trait(
    struct_name: &syn::Ident,
    data_struct: &syn::DataStruct,
    trait_name: &Path,
    data_name: &Path,
) -> TokenStream {
    // Ensure the struct has named fields
    let fields = if let Fields::Named(fields_named) = &data_struct.fields {
        &fields_named.named
    } else {
        return TokenStream::from(quote! {
            compile_error!("Concrete can only be derived for structs with named fields");
        });
    };

    // Find the field of type DisplayData
    let display_data_field = fields.iter().find(|f| {
        if let syn::Type::Path(type_path) = &f.ty {
            type_path.path.segments.last().unwrap().ident
                == data_name.segments.last().unwrap().ident
        } else {
            false
        }
    });

    let field_name = match display_data_field {
        Some(field) => &field.ident,
        None => {
            return TokenStream::from(quote! {
                compile_error!("No field of type DisplayData found in struct");
            });
        }
    };

    let expanded = quote! {
        impl #trait_name for #struct_name {
            fn data(&self) -> &#data_name {
                &self.#field_name
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Abstract, attributes(abstract_trait, data_struct))]
pub fn derive_abstract(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the trait name from the helper attribute
    let trait_name = match get_trait_name(&input.attrs, "abstract_trait") {
        Ok(trait_name) => trait_name,
        Err(token_stream) => return token_stream,
    };
    let data_name = match get_trait_name(&input.attrs, "data_struct") {
        Ok(data_name) => data_name,
        Err(token_stream) => return token_stream,
    };
    // Generate the implementation
    match &input.data {
        Data::Enum(data_enum) => {
            impl_abstract_trait(&input.ident, data_enum, &trait_name, &data_name)
        }
        _ => TokenStream::from(quote! {
            compile_error!("Abstract can only be derived for enums");
        }),
    }
}

fn impl_abstract_trait(
    enum_name: &syn::Ident,
    data_enum: &syn::DataEnum,
    trait_name: &Path,
    data_name: &Path,
) -> TokenStream {
    let variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
                quote! {
                    Self::#variant_name(inner) => inner.data(),
                }
            }
            _ => {
                quote! {
                    _ => panic!("Cannot derive {} for variant {}", stringify!(#trait_name), stringify!(#variant_name)),
                }
            }
        }
    });

    let expanded = quote! {
        impl #trait_name for #enum_name {
            fn data(&self) -> &#data_name {
                match self {
                    #(#variants)*
                }
            }
        }
    };

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
