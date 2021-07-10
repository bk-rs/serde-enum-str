extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod input;
mod output_de;
mod output_ser;

#[proc_macro_derive(Serialize_enum_str, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as self::input::Input);

    TokenStream::from(quote!(#input))
}

#[proc_macro_derive(Deserialize_enum_str, attributes(serde))]
pub fn derive_deserialize(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum __FooDe {
            A,
            #[serde(rename = "B")]
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
