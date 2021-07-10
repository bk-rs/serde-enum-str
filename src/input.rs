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

pub struct Input {
    pub ident: Ident,
    pub rename_all: Option<String>,
    pub variants: Vec<Variant>,
    pub default_variant: Option<DefaultVariant>,
}

pub struct Variant {
    pub ident: Ident,
    pub rename: Option<String>,
}

#[derive(Clone)]
pub struct DefaultVariant {
    pub ident: Ident,
    pub r#type: Type,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let call_site = Span::call_site();

        let derive_input = DeriveInput::parse(input)?;
        let enum_derive_input = EnumDeriveInput::from_derive_input(&derive_input)
            .map_err(|err| SynError::new(call_site, err.write_errors()))?;
        let enum_variants = match &enum_derive_input.data {
            Data::Enum(enum_variants) => enum_variants,
            _ => return Err(SynError::new(call_site, "input must be an enum")),
        };

        let ident = enum_derive_input.ident;
        let rename_all = enum_derive_input.rename_all;

        let mut variants = vec![];
        let mut default_variants = vec![];
        for enum_variant in enum_variants {
            if enum_variant.is_other {
                if !enum_variant.fields.is_tuple() {
                    return Err(SynError::new(
                        enum_variant.ident.span(),
                        "must be a tuple variant",
                    ));
                }
                let mut types_iter = enum_variant.fields.to_owned().into_iter();
                let r#type = types_iter.next().ok_or_else(|| {
                    SynError::new(enum_variant.ident.span(), "must be at least one type")
                })?;
                if types_iter.next().is_some() {
                    return Err(SynError::new(
                        enum_variant.ident.span(),
                        "must be at least one type",
                    ));
                }

                default_variants.push(DefaultVariant {
                    ident: enum_variant.ident.to_owned(),
                    r#type,
                });
            } else {
                if !enum_variant.fields.is_unit() {
                    return Err(SynError::new(
                        enum_variant.ident.span(),
                        "must be a unit variant",
                    ));
                }

                variants.push(Variant {
                    ident: enum_variant.ident.to_owned(),
                    rename: enum_variant.rename.to_owned(),
                });
            }
        }
        if variants.is_empty() && default_variants.is_empty() {
            return Err(SynError::new(
                call_site,
                "there must be at least one variant",
            ));
        }
        if default_variants.len() > 1 {
            return Err(SynError::new(
                call_site,
                "only one variant can be #[serde(other)]",
            ));
        }

        let generics = enum_derive_input.generics;
        if !generics.params.is_empty() || generics.where_clause.is_some() {
            return Err(SynError::new(call_site, "generic enum is not supported"));
        }

        let default_variant = default_variants.first().cloned();
        Ok(Self {
            ident,
            rename_all,
            variants,
            default_variant,
        })
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
    rename_all: Option<String>,
}

#[derive(FromVariant, Debug)]
#[darling(attributes(serde), forward_attrs(doc))]
struct EnumVariant {
    attrs: Vec<Attribute>,
    ident: Ident,
    fields: Fields<Type>,
    discriminant: Option<Expr>,

    #[darling(default)]
    rename: Option<String>,
    #[darling(default, rename = "other", map = "Self::make_is_other")]
    is_other: bool,
}
impl EnumVariant {
    fn make_is_other(v: Option<()>) -> bool {
        v.is_some()
    }
}
