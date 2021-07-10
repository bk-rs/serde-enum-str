use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};

use super::input::Input;

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ser_enum_ident = format_ident!("__{}Ser", &self.ident);
        let ser_enum_serde_rename_all = if let Some(rename_all) = &self.rename_all {
            quote!(#[serde(rename_all = #rename_all)])
        } else {
            quote!()
        };
        let ser_enum_variants = &self
            .variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                let serde_rename = if let Some(rename) = &variant.rename {
                    quote!(#[serde(rename = #rename)])
                } else {
                    quote!()
                };
                quote! {
                    #serde_rename
                    #ident,
                }
            })
            .collect::<Vec<_>>();

        let token = quote! {
            #[derive(serde::Serialize)]
            #ser_enum_serde_rename_all
            enum #ser_enum_ident {
                #(#ser_enum_variants)*
            }
        };
        tokens.append_all(token);

        let impl_ident = &self.ident;
        let impl_variants = &self
            .variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                quote! {
                    Self::#ident => #ser_enum_ident::#ident,
                }
            })
            .collect::<Vec<_>>();
        let impl_default_variant = if let Some(default_variant) = &self.default_variant {
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
