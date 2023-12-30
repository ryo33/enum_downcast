mod variant;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, parse_quote};

use crate::variant::generate_variant;

#[proc_macro_derive(EnumDowncast, attributes(enum_downcast))]
pub fn enum_downcast_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_enum_downcast(parse_macro_input!(input)).into()
}

fn impl_enum_downcast(item: syn::ItemEnum) -> TokenStream {
    let mut tokens = TokenStream::new();
    for variant in &item.variants {
        if variant
            .attrs
            .contains(&parse_quote! {#[enum_downcast(skip)]})
        {
            continue;
        }
        tokens.extend(generate_variant(&item, variant));
    }
    tokens
}
