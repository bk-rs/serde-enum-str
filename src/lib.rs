extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Serialize_enum_str, attributes(serde))]
pub fn derive_serialize(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "snake_case")]
        enum __FooSer {
            A,
            B,
        }

        impl serde::Serialize for Foo {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let value = match *self {
                    Self::A => __FooSer::A,
                    Self::B => __FooSer::B,
                    Self::Other(ref s) => return serde::Serialize::serialize(s, serializer),
                };

                serde::Serialize::serialize(&value, serializer)
            }
        }
    })
}

#[proc_macro_derive(Deserialize_enum_str, attributes(serde))]
pub fn derive_deserialize(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum __FooDe {
            A,
            B,
        }

        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum __FooDeUntagged {
            Enum(__FooDe),
            Other(String),
        }

        impl<'de> serde::Deserialize<'de> for Foo {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where D: serde::Deserializer<'de>
            {
                let value = match <__FooDeUntagged as serde::Deserialize>::deserialize(deserializer)? {
                    __FooDeUntagged::Enum(e) => match e {
                        __FooDe::A => Foo::A,
                        __FooDe::B => Foo::B,
                    },
                    __FooDeUntagged::Other(s) => Foo::Other(s)
                };

                Ok(value)
            }
        }
    })
}
