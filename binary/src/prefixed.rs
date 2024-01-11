use crate::datatypes::{VarI32, VarU32, I16, I32, U16, U32};
use crate::{debug_impl_tt, Binary};
use byteorder::ByteOrder;
use bytes::Buf;
use std::io::{Cursor, Result, Write};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Prefix trait is implemented for the numeric data types that can be used
/// to encode the length of prefixed objects like Arrays, Vectors, Strings, etc.
pub trait Prefix {
    fn encode(size: usize, buf: &mut impl Write);
    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize>;
}

impl<E: ByteOrder> Prefix for U16<E> {
    fn encode(size: usize, buf: &mut impl Write) {
        let val = size as u16;
        U16::<E>::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = U16::<E>::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl<E: ByteOrder> Prefix for I16<E> {
    fn encode(size: usize, buf: &mut impl Write) {
        let val = size as i16;
        I16::<E>::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = I16::<E>::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl<E: ByteOrder> Prefix for I32<E> {
    fn encode(size: usize, buf: &mut impl Write) {
        let val = size as i32;
        I32::<E>::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = I32::<E>::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl<E: ByteOrder> Prefix for U32<E> {
    fn encode(size: usize, buf: &mut impl Write) {
        let val = size as u32;
        U32::<E>::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = U32::<E>::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl Prefix for VarI32 {
    fn encode(size: usize, buf: &mut impl Write) {
        let val = size as i32;
        VarI32::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = VarI32::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

impl Prefix for VarU32 {
    fn encode(size: usize, buf: &mut impl Write) {
        let val = size as u32;
        VarU32::new(val).serialize(buf);
    }

    fn decode<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<usize> {
        let val = VarU32::deserialize(buf)?.0;
        Ok(val as usize)
    }
}

/// Custom String Type with a generic for the Prefix type.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Str<'a, P: Prefix>(&'a str, PhantomData<P>);

impl<'a, P: Prefix> Str<'a, P> {
    pub fn new(val: &'a str) -> Self {
        Self(val, PhantomData)
    }
}

impl<'a, P: Prefix> Binary<'a> for Str<'a, P> {
    fn serialize(&self, buf: &mut impl Write) {
        let len = self.0.len();
        P::encode(len, buf);

        buf.write_all(self.0.as_bytes()).unwrap();
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
#[derive(Clone, PartialEq, Eq)]
pub struct Array<'a, B: Binary<'a>, P: Prefix>(Vec<B>, PhantomData<P>, &'a ());

impl<'a, B: Binary<'a>, P: Prefix> Array<'a, B, P> {
    pub fn new(array: Vec<B>) -> Self {
        Self(array, PhantomData, &())
    }
}

impl<'a, B: Binary<'a>, P: Prefix> Binary<'a> for Array<'a, B, P> {
    fn serialize(&self, buf: &mut impl Write) {
        let len = self.0.len();
        P::encode(len, buf);

        for element in &self.0 {
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
        &self.0
    }
}

impl<'a, B: Binary<'a>, P: Prefix> DerefMut for Array<'a, B, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// UnsizedBytes should be used when you want to encode or decode a slice of bytes without any prefix.
/// It reads the complete remaining portion of the buffer as a slice, so use this only when you want
/// to read a slice from the end of the buffer that does not contain anything else after the slice.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct UnsizedBytes<'a>(&'a [u8]);

impl<'a> UnsizedBytes<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self(data)
    }
}

impl<'a> Binary<'a> for UnsizedBytes<'a> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_all(&self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let start = buf.position() as usize;
        let end = buf.remaining() + start;

        Ok(Self::new(&buf.get_ref()[start..end]))
    }
}

impl<'a> Deref for UnsizedBytes<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for UnsizedBytes<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

debug_impl_tt!(Str<P: Prefix>);
debug_impl_tt!(Array<B: Binary<'a>, P: Prefix>);
debug_impl_tt!(UnsizedBytes);
