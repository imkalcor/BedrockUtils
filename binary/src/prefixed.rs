use crate::datatypes::{VarI32, VarU32, I16, I32, U16, U32};
use crate::Binary;
use byteorder::ByteOrder;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Result, Write};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Prefix trait is implemented for the numeric data types that can be used
/// to encode the length of prefixed objects like Arrays, Vectors, Strings, etc.
pub trait Prefix {
    fn encode(size: usize, buf: &mut BytesMut);
    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize>;
}

impl<E: ByteOrder> Prefix for U16<E> {
    fn encode(size: usize, buf: &mut BytesMut) {
        let val = size as u16;
        U16::<E>::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = U16::<E>::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl<E: ByteOrder> Prefix for I16<E> {
    fn encode(size: usize, buf: &mut BytesMut) {
        let val = size as i16;
        I16::<E>::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = I16::<E>::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl<E: ByteOrder> Prefix for I32<E> {
    fn encode(size: usize, buf: &mut BytesMut) {
        let val = size as i32;
        I32::<E>::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = I32::<E>::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl<E: ByteOrder> Prefix for U32<E> {
    fn encode(size: usize, buf: &mut BytesMut) {
        let val = size as u32;
        U32::<E>::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = U32::<E>::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl Prefix for VarI32 {
    fn encode(size: usize, buf: &mut BytesMut) {
        let val = size as i32;
        VarI32::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = VarI32::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl Prefix for VarU32 {
    fn encode(size: usize, buf: &mut BytesMut) {
        let val = size as u32;
        VarU32::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = VarU32::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

/// Custom String Type with a generic for the Prefix type.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Str<'a, P: Prefix>(&'a str, PhantomData<P>);

impl<'a, P: Prefix> Str<'a, P> {
    pub fn new(val: &'a str) -> Self {
        Self(val, PhantomData)
    }
}

impl<'a, P: Prefix> Binary<'a> for Str<'a, P> {
    fn serialize(&self, buf: &mut BytesMut) {
        let len = self.0.len();
        P::encode(len, buf);

        buf.write_all(self.0.as_bytes()).unwrap()
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let len = P::decode(buf)?;
        let start = buf.position() as usize;
        let end = start + len;

        buf.advance(len);

        let val = std::str::from_utf8(&buf.get_ref()[start..end]).unwrap();
        Ok(Self::new(val))
    }
}

impl<'a, P: Prefix> Deref for Str<'a, P> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Custom Array Type with a generic for the Type T that implements the Binary trait (serializable)
/// and P for the type of prefix for serialization of length.
#[derive(Debug)]
pub struct Array<'a, B: Binary<'a>, P: Prefix> {
    array: Vec<B>,
    phantom: PhantomData<P>,
    _lifetime: &'a (),
}

impl<'a, B: Binary<'a>, P: Prefix> Array<'a, B, P> {
    /// Returns a new instance of the array.
    pub fn new(array: Vec<B>) -> Self {
        Self {
            array,
            phantom: PhantomData,
            _lifetime: &(),
        }
    }

    /// Returns the length of the array
    pub fn len(&self) -> usize {
        self.array.len()
    }
}

impl<'a, B: Binary<'a>, P: Prefix> Binary<'a> for Array<'a, B, P> {
    fn serialize(&self, buf: &mut BytesMut) {
        let len = self.array.len();
        P::encode(len, buf);

        for element in &self.array {
            element.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let len = P::decode(buf)?;
        let mut array = Vec::with_capacity(len);

        for i in 0..len {
            array.insert(i, B::deserialize(buf)?);
        }

        Ok(Self::new(array))
    }
}

impl<'a, B: Binary<'a>, P: Prefix> Deref for Array<'a, B, P> {
    type Target = Vec<B>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<'a, B: Binary<'a>, P: Prefix> DerefMut for Array<'a, B, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}
