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

        //
        let impl_ident = &input.ident;

        //
        let impl_serialize_variants = &input
            .variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                quote! {
                    Self::#ident => #ser_enum_ident::#ident,
                }
            })
            .collect::<Vec<_>>();
        let impl_serialize_default_variant = if let Some(default_variant) = &input.default_variant {
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
                        #(#impl_serialize_variants)*
                        #impl_serialize_default_variant
                    };
                    serde::Serialize::serialize(&value, serializer)
                }
            }
        };
        tokens.append_all(token);

        //
        let impl_display_variants = &input
            .variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                if variant.skip_serializing == Some(true) {
                    // TODO, match rename_all
                    let name = ident.to_string();
                    quote! {
                        Self::#ident => write!(f, "{}", #name),
                    }
                } else {
                    quote! {
                        Self::#ident => self.serialize(f),
                    }
                }
            })
            .collect::<Vec<_>>();
        let impl_display_default_variant = if let Some(default_variant) = &input.default_variant {
            let ident = &default_variant.ident;
            quote! {
                Self::#ident(ref s) => write!(f, "{}", s),
            }
        } else {
            quote!()
        };

        let token = quote! {
            impl ::core::fmt::Display for #impl_ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    use serde::Serialize as _;

                    match *self {
                        #(#impl_display_variants)*
                        #impl_display_default_variant
                    }
                }
            }
        };
        tokens.append_all(token);
    }
}
