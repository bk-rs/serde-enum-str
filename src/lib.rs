extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod input;
mod output;
mod output_de;
mod output_ser;

#[proc_macro_derive(Serialize_enum_str, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as self::input::Input);
    let input = self::output_ser::InputWrapper(input);

    TokenStream::from(quote!(#input))
}

#[proc_macro_derive(Deserialize_enum_str, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as self::input::Input);
    let input = self::output_de::InputWrapper(input);

    TokenStream::from(quote!(#input))
}
