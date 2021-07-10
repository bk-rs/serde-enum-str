use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

mod simple {
    use super::*;

    #[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Debug)]
    #[serde(rename_all = "snake_case")]
    enum Foo {
        A,
        #[serde(rename = "B")]
        B,
        #[serde(other)]
        Other(String),
    }

    #[test]
    fn test_ser() {
        assert_eq!(serde_json::to_string(&Foo::A).unwrap(), r#""a""#);
        assert_eq!(serde_json::to_string(&Foo::B).unwrap(), r#""B""#);
        assert_eq!(
            serde_json::to_string(&Foo::Other("c".to_owned())).unwrap(),
            r#""c""#
        );
    }

    #[test]
    fn test_de() {
        assert_eq!(serde_json::from_str::<Foo>(r#""a""#).unwrap(), Foo::A);
        assert_eq!(serde_json::from_str::<Foo>(r#""B""#).unwrap(), Foo::B);
        assert_eq!(
            serde_json::from_str::<Foo>(r#""c""#).unwrap(),
            Foo::Other("c".to_owned())
        );
    }
}

mod without_rename {
    use super::*;

    #[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Debug)]
    enum Foo {
        A,
        B,
        #[serde(other)]
        Other(String),
    }

    #[test]
    fn test_ser() {
        assert_eq!(serde_json::to_string(&Foo::A).unwrap(), r#""A""#);
        assert_eq!(serde_json::to_string(&Foo::B).unwrap(), r#""B""#);
        assert_eq!(
            serde_json::to_string(&Foo::Other("c".to_owned())).unwrap(),
            r#""c""#
        );
    }

    #[test]
    fn test_de() {
        assert_eq!(serde_json::from_str::<Foo>(r#""A""#).unwrap(), Foo::A);
        assert_eq!(serde_json::from_str::<Foo>(r#""B""#).unwrap(), Foo::B);
        assert_eq!(
            serde_json::from_str::<Foo>(r#""c""#).unwrap(),
            Foo::Other("c".to_owned())
        );
    }
}

mod without_other {
    use super::*;

    #[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Debug)]
    #[serde(rename_all = "snake_case")]
    enum Foo {
        A,
        #[serde(rename = "B")]
        B,
    }

    #[test]
    fn test_ser() {
        assert_eq!(serde_json::to_string(&Foo::A).unwrap(), r#""a""#);
        assert_eq!(serde_json::to_string(&Foo::B).unwrap(), r#""B""#);
    }

    #[test]
    fn test_de() {
        assert_eq!(serde_json::from_str::<Foo>(r#""a""#).unwrap(), Foo::A);
        assert_eq!(serde_json::from_str::<Foo>(r#""B""#).unwrap(), Foo::B);
    }
}
