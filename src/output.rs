use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};
use syn::Ident;

use super::input::Input;

pub struct SerdeEnum<'a> {
    input: &'a Input,
    category: SerdeEnumCategory,
}
pub enum SerdeEnumCategory {
    Ser,
    De,
}
impl<'a> SerdeEnum<'a> {
    pub fn new(input: &'a Input, category: SerdeEnumCategory) -> Self {
        Self { input, category }
    }
    pub fn ident(&self) -> Ident {
        format_ident!("__{}{}", self.input.ident, self.suffix())
    }
    fn suffix(&self) -> &'static str {
        match self.category {
            SerdeEnumCategory::Ser => "Ser",
            SerdeEnumCategory::De => "De",
        }
    }
}
impl<'a> ToTokens for SerdeEnum<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let input = self.input;

        let derive_serde = match self.category {
            SerdeEnumCategory::Ser => quote! {
                #[derive(serde::Serialize)]
            },
            SerdeEnumCategory::De => quote! {
                #[derive(serde::Deserialize)]
            },
        };
        let serde_rename_all = if let Some(rename_all) = &input.rename_all {
            quote!(#[serde(rename_all = #rename_all)])
        } else {
            quote!()
        };
        let ident = self.ident();
        let variants = &input
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
            #derive_serde
            #serde_rename_all
            enum #ident {
                #(#variants)*
            }
        };
        tokens.append_all(token);
    }
}