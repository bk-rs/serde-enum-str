use darling::{Error as DarlingError, FromMeta};
use serde_rename_rule::RenameRule;
use syn::{Lit, Meta, NestedMeta};

#[derive(Debug, Clone)]
pub struct RenameAllAttribute {
    pub serialize: Option<RenameRule>,
    pub deserialize: Option<RenameRule>,
}
impl FromMeta for RenameAllAttribute {
    fn from_string(s: &str) -> Result<Self, DarlingError> {
        let rule = RenameRule::from_rename_all_str(s)
            .map_err(|err| DarlingError::custom(err.msg_for_rename_all()))?;

        Ok(Self {
            serialize: Some(rule.to_owned()),
            deserialize: Some(rule),
        })
    }

    fn from_list(items: &[NestedMeta]) -> Result<Self, DarlingError> {
        let mut serialize = None;
        let mut deserialize = None;
        for item in items {
            match item {
                NestedMeta::Meta(Meta::NameValue(value)) if value.path.is_ident("serialize") => {
                    if let Lit::Str(s) = &value.lit {
                        let rule = RenameRule::from_rename_all_str(&s.value())
                            .map_err(|err| DarlingError::custom(err.msg_for_rename_all()))?;
                        serialize = Some(rule);
                    }
                }
                NestedMeta::Meta(Meta::NameValue(value)) if value.path.is_ident("deserialize") => {
                    if let Lit::Str(s) = &value.lit {
                        let rule = RenameRule::from_rename_all_str(&s.value())
                            .map_err(|err| DarlingError::custom(err.msg_for_rename_all()))?;
                        deserialize = Some(rule);
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
