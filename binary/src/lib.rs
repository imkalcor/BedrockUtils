pub mod datatypes;
pub mod prefixed;

use std::io::{Cursor, Result, Write};

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
pub trait Binary<'a>: Sized {
    fn serialize(&self, buf: &mut impl Write);
    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self>;
}
