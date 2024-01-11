pub mod datatypes;
pub mod prefixed;

use std::{
    fmt::Debug,
    io::{Cursor, Write},
};

///
/// Binary trait is implemented for all the data types, structs or enums that can be
/// serialized or deserialized over the network stream.
///
/// We do not need to return a `Result` whilst serializing because it's rare to crash on `panic!` while
/// unwrapping.
///
/// We can guarantee the serialization of data that we do on our end but we cannot guarantee the data
/// coming from the other end is in the format we expect.
///
pub trait Binary<'a>: Sized + Debug {
    fn serialize(&self, buf: &mut impl Write);
    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> std::io::Result<Self>;
}

///
/// This macro implements automatic formatting for various datatype and structs that we derive.
///
#[macro_export]
macro_rules! debug_impl {
    ($struct_name:ident $(<$($generic_param:ident $(: $trait_bound:path)*),*>)?) => {
        impl<$($($generic_param $(: $trait_bound)*),*)?> std::fmt::Debug for $struct_name<$($($generic_param),*)?> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}

///
/// Does the same job as the above provided debug implementation, but it is to be used only for lifetime based
/// datatypes.
///
#[macro_export]
macro_rules! debug_impl_tt {
    ($struct_name:ident $(<$($generic_param:ident $(: $trait_bound:path)*),*>)?) => {
        impl<'a, $($($generic_param $(: $trait_bound)*),*)?> std::fmt::Debug for $struct_name<'a, $($($generic_param),*)?>
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}
