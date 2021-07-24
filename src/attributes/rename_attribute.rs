use darling::{Error as DarlingError, FromMeta};
use syn::NestedMeta;

#[derive(Debug, Clone)]
pub struct RenameAttribute {
    pub serialize: String,
    pub deserialize: String,
}
impl FromMeta for RenameAttribute {
    fn from_string(s: &str) -> Result<Self, DarlingError> {
        Ok(Self {
            serialize: s.to_owned(),
            deserialize: s.to_owned(),
        })
    }

    fn from_list(items: &[NestedMeta]) -> Result<Self, DarlingError> {
        println!("TODO {:?}", items);
        Ok(Self {
            serialize: "aa".to_owned(),
            deserialize: "AA".to_owned(),
        })
    }
}
