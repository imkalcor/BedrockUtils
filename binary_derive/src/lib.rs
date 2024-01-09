extern crate proc_macro;
use binary::binary_derive;
use proc_macro::TokenStream as StdTokenStream;

mod binary;

///
/// Derives Binary trait for Structs and Enums
///
#[proc_macro_derive(Binary, attributes(data, variant, skip))]
pub fn derive_binary(item: StdTokenStream) -> StdTokenStream {
    match binary_derive(item.into()) {
        Ok(val) => val.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
