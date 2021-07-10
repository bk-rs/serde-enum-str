#![allow(clippy::nonstandard_macro_braces)]

use darling::{
    ast::{Data, Fields},
    util::Ignored,
    FromDeriveInput, FromVariant,
};
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, DeriveInput, Error as SynError, Expr, Generics, Ident, Type, Visibility,
};

pub struct Input {}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let call_site = Span::call_site();

        let derive_input = DeriveInput::parse(input)?;
        let enum_derive_input = match EnumDeriveInput::from_derive_input(&derive_input) {
            Ok(enum_derive_input) => enum_derive_input,
            Err(err) => return Err(SynError::new(call_site, err.write_errors())),
        };

        println!("{:?}", enum_derive_input);

        Ok(Self {})
    }
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(serde), forward_attrs(doc))]
struct EnumDeriveInput {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    generics: Generics,
    data: Data<EnumVariant, Ignored>,

    #[darling(default)]
    pub rename_all: Option<String>,
}

#[derive(FromVariant, Debug)]
#[darling(attributes(serde), forward_attrs(doc))]
struct EnumVariant {
    attrs: Vec<Attribute>,
    ident: Ident,
    fields: Fields<Type>,
    discriminant: Option<Expr>,

    #[darling(default)]
    pub rename: Option<String>,
    #[darling(default)]
    pub other: Option<()>,
}
