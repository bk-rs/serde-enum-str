use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

mod simple {
    use super::*;

    #[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Debug)]
    #[serde(rename_all = "snake_case")]
    enum Foo {
        A,
        B,
        #[serde(other)]
        Other(String),
    }

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&Foo::A).unwrap(), r#""a""#);
        assert_eq!(serde_json::to_string(&Foo::B).unwrap(), r#""b""#);
        assert_eq!(
            serde_json::to_string(&Foo::Other("c".to_owned())).unwrap(),
            r#""c""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(serde_json::from_str::<Foo>(r#""a""#).unwrap(), Foo::A);
        assert_eq!(
            serde_json::from_str::<Foo>(r#""c""#).unwrap(),
            Foo::Other("c".to_owned())
        );
    }
}
