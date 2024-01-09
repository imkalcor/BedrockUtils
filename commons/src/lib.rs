pub mod logger;
pub mod utils;

///
/// This macro returns true if an Enum Instance passed is of one of the variants passed.
///
#[macro_export]
macro_rules! is_variant {
    ($enum:ident, $enum_instance:expr, $($variant:ident),* $(,)?) => {
        {
            let mut is_match = false;
            $(if let $enum::$variant = $enum_instance {
                is_match = true;
            })*
            is_match
        }
    };
}
