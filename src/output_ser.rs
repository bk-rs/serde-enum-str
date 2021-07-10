use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt as _};

use super::{
    input::Input,
    output::{SerdeEnum, SerdeEnumCategory},
};

pub struct InputWrapper(pub Input);

impl ToTokens for InputWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let input = &self.0;

        let ser_enum = SerdeEnum::new(input, SerdeEnumCategory::Ser);
        let ser_enum_ident = ser_enum.ident();

        let token = quote! {
            #ser_enum
        };
        tokens.append_all(token);

        let impl_ident = &input.ident;
        let impl_variants = &input
            .variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                quote! {
                    Self::#ident => #ser_enum_ident::#ident,
                }
            })
            .collect::<Vec<_>>();
        let impl_default_variant = if let Some(default_variant) = &input.default_variant {
            let ident = &default_variant.ident;
            quote! {
                Self::#ident(ref s) => return serde::Serialize::serialize(s, serializer),
            }
        } else {
            quote!()
        };

        let token = quote! {
            impl serde::Serialize for #impl_ident {
                fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let value = match *self {
                        #(#impl_variants)*
                        #impl_default_variant
                    };
                    serde::Serialize::serialize(&value, serializer)
                }
            }
        };
        tokens.append_all(token);
    }
}
