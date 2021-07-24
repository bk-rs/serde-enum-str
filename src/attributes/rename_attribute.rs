use darling::{Error as DarlingError, FromMeta};
use syn::{Lit, Meta, NestedMeta};

#[derive(Debug, Clone)]
pub struct RenameAttribute {
    pub serialize: Option<String>,
    pub deserialize: Option<String>,
}
impl FromMeta for RenameAttribute {
    fn from_string(s: &str) -> Result<Self, DarlingError> {
        Ok(Self {
            serialize: Some(s.to_owned()),
            deserialize: Some(s.to_owned()),
        })
    }

    fn from_list(items: &[NestedMeta]) -> Result<Self, DarlingError> {
        let mut serialize = None;
        let mut deserialize = None;
        for item in items {
            match item {
                NestedMeta::Meta(Meta::NameValue(value)) if value.path.is_ident("serialize") => {
                    if let Lit::Str(s) = &value.lit {
                        serialize = Some(s.value());
                    }
                }
                NestedMeta::Meta(Meta::NameValue(value)) if value.path.is_ident("deserialize") => {
                    if let Lit::Str(s) = &value.lit {
                        deserialize = Some(s.value());
                    }
                }
                _ => {}
            }
        }

        if serialize.is_none() && deserialize.is_none() {
            return Err(DarlingError::custom(
                "must be at least one the serialize and deserialize",
            ));
        }

        Ok(Self {
            serialize,
            deserialize,
        })
    }
}
