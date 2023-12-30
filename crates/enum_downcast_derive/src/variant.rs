use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate_variant(item: &syn::ItemEnum, variant: &syn::Variant) -> TokenStream {
    if variant.fields.len() != 1 {
        return syn::Error::new_spanned(variant, "expected exactly one field. if intented, put `#[enum_downcast(skip)]` on this variant, and provide custom implementation if you need to downcast anyways").to_compile_error();
    }
    let field = variant.fields.iter().next().unwrap();
    let enum_name = &item.ident;
    let variant_name = &variant.ident;
    let variant_ty = &field.ty;
    if let syn::Type::Path(path) = variant_ty {
        if let Some(variant_ty_ident) = path.path.get_ident() {
            if item.generics.params.iter().any(|param| {
                if let syn::GenericParam::Type(type_param) = param {
                    type_param.ident == *variant_ty_ident
                } else {
                    false
                }
            }) {
                return quote! {
                    ::core::compile_error!("cannot downcast a type parameter. use a newtype like `NewType<T>` instead of `T`, or use `#[enum_downcast(skip)]` to skip this variant");
                };
            }
        }
    }
    let bound = &item.generics;
    let mut generics = item.generics.clone();
    generics.params.iter_mut().for_each(|param| {
        if let syn::GenericParam::Type(ref mut type_param) = *param {
            // make bounds empty
            type_param.bounds = Default::default();
        }
    });
    let pattern = if let Some(ident) = &field.ident {
        quote! {#enum_name::#variant_name { #ident }}
    } else {
        quote! {#enum_name::#variant_name(inner)}
    };
    let inner = if let Some(ident) = &field.ident {
        quote! {#ident}
    } else {
        quote! {inner}
    };
    quote! {
        impl #bound enum_downcast::IntoVariant<#variant_ty> for #enum_name #generics {
            fn into_variant(self) -> Result<#variant_ty, Self>
            where
                Self: Sized {
                match self {
                    #pattern => Ok(#inner),
                    _ => Err(self),
                }
            }
        }
        impl #bound enum_downcast::AsVariant<#variant_ty> for #enum_name #generics {
            fn as_variant(&self) -> Option<&#variant_ty> {
                match self {
                    #pattern => Some(#inner),
                    _ => None,
                }
            }
        }
        impl #bound enum_downcast::AsVariantMut<#variant_ty> for #enum_name #generics {
            fn as_variant_mut(&mut self) -> Option<&mut #variant_ty> {
                match self {
                    #pattern => Some(#inner),
                    _ => None,
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_not_singular() {
        let input = syn::parse_quote! {
            enum Enum {
                String(String),
                Number(u32, u32),
            }
        };
        let variant = syn::parse_quote! {
            Number(u32, u32)
        };
        let actual = generate_variant(&input, &variant);
        let expected = syn::Error::new_spanned(&variant, "expected exactly one field. if intented, put `#[enum_downcast(skip)]` on this variant, and provide custom implementation if you need to downcast anyways").to_compile_error();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_with_field_name() {
        let input = syn::parse_quote! {
            enum Enum {
                String { name: String },
                Number(u32),
            }
        };
        let variant = syn::parse_quote! {
            String { name: String }
        };
        let actual = generate_variant(&input, &variant);
        let expected = quote! {
            impl enum_downcast::IntoVariant<String> for Enum {
                fn into_variant(self) -> Result<String, Self>
                where
                    Self: Sized
                {
                    match self {
                        Enum::String { name } => Ok(name),
                        _ => Err(self),
                    }
                }
            }
            impl enum_downcast::AsVariant<String> for Enum {
                fn as_variant(&self) -> Option<&String> {
                    match self {
                        Enum::String { name } => Some(name),
                        _ => None,
                    }
                }
            }
            impl enum_downcast::AsVariantMut<String> for Enum {
                fn as_variant_mut(&mut self) -> Option<&mut String> {
                    match self {
                        Enum::String { name } => Some(name),
                        _ => None,
                    }
                }
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_generics() {
        let input = syn::parse_quote! {
            enum Enum<T> {
                String(String),
                Number(T),
            }
        };
        let variant = syn::parse_quote! {
            String(String)
        };
        let actual = generate_variant(&input, &variant);
        let expected = quote! {
            impl <T> enum_downcast::IntoVariant<String> for Enum<T> {
                fn into_variant(self) -> Result<String, Self>
                where
                    Self: Sized
                {
                    match self {
                        Enum::String(inner) => Ok(inner),
                        _ => Err(self),
                    }
                }
            }
            impl <T> enum_downcast::AsVariant<String> for Enum<T> {
                fn as_variant(&self) -> Option<&String> {
                    match self {
                        Enum::String(inner) => Some(inner),
                        _ => None,
                    }
                }
            }
            impl <T> enum_downcast::AsVariantMut<String> for Enum<T> {
                fn as_variant_mut(&mut self) -> Option<&mut String> {
                    match self {
                        Enum::String(inner) => Some(inner),
                        _ => None,
                    }
                }
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_generics_with_bound() {
        let input = syn::parse_quote! {
            enum Enum<T: Clone> {
                String(String),
                Number(T),
            }
        };
        let variant = syn::parse_quote! {
            String(String)
        };
        let actual = generate_variant(&input, &variant);
        let expected = quote! {
            impl <T: Clone> enum_downcast::IntoVariant<String> for Enum<T> {
                fn into_variant(self) -> Result<String, Self>
                where
                    Self: Sized
                {
                    match self {
                        Enum::String(inner) => Ok(inner),
                        _ => Err(self),
                    }
                }
            }
            impl <T: Clone> enum_downcast::AsVariant<String> for Enum<T> {
                fn as_variant(&self) -> Option<&String> {
                    match self {
                        Enum::String(inner) => Some(inner),
                        _ => None,
                    }
                }
            }
            impl <T: Clone> enum_downcast::AsVariantMut<String> for Enum<T> {
                fn as_variant_mut(&mut self) -> Option<&mut String> {
                    match self {
                        Enum::String(inner) => Some(inner),
                        _ => None,
                    }
                }
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_generics_variant() {
        let input = syn::parse_quote! {
            enum Enum<T: Clone> {
                String(String),
                Number(T),
            }
        };
        let variant = syn::parse_quote! {
            Number(T)
        };
        let actual = generate_variant(&input, &variant);
        let expected = quote! {
            ::core::compile_error!("cannot downcast a type parameter. use a newtype like `NewType<T>` instead of `T`, or use `#[enum_downcast(skip)]` to skip this variant");
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_variant_of_type_with_generics() {
        let input = syn::parse_quote! {
            enum Enum<T> {
                String(String),
                Number(MyNumber<T>),
            }
        };
        let variant = syn::parse_quote! {
            Number(MyNumber<T>)
        };
        let actual = generate_variant(&input, &variant);
        let expected = quote! {
            impl <T> enum_downcast::IntoVariant<MyNumber<T> > for Enum<T> {
                fn into_variant(self) -> Result<MyNumber<T>, Self>
                where
                    Self: Sized
                {
                    match self {
                        Enum::Number(inner) => Ok(inner),
                        _ => Err(self),
                    }
                }
            }
            impl <T> enum_downcast::AsVariant<MyNumber<T> > for Enum<T> {
                fn as_variant(&self) -> Option<&MyNumber<T> > {
                    match self {
                        Enum::Number(inner) => Some(inner),
                        _ => None,
                    }
                }
            }
            impl <T> enum_downcast::AsVariantMut<MyNumber<T> > for Enum<T> {
                fn as_variant_mut(&mut self) -> Option<&mut MyNumber<T> > {
                    match self {
                        Enum::Number(inner) => Some(inner),
                        _ => None,
                    }
                }
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }
}
