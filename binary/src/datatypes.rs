use crate::{debug_impl, Binary};
use byteorder::ByteOrder;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Error, ErrorKind, Result, Write};
use std::marker::PhantomData;

#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Bool(pub bool);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct U8(pub u8);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct I8(pub i8);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct U16<E: ByteOrder>(pub u16, PhantomData<E>);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct I16<E: ByteOrder>(pub i16, PhantomData<E>);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct U24<E: ByteOrder>(pub u32, PhantomData<E>);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct I24<E: ByteOrder>(pub i32, PhantomData<E>);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct U32<E: ByteOrder>(pub u32, PhantomData<E>);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct I32<E: ByteOrder>(pub i32, PhantomData<E>);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct U64<E: ByteOrder>(pub u64, PhantomData<E>);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct I64<E: ByteOrder>(pub i64, PhantomData<E>);
#[derive(Clone, Default, PartialEq)]
pub struct F32<E: ByteOrder>(pub f32, PhantomData<E>);
#[derive(Clone, Default, PartialEq)]
pub struct F64<E: ByteOrder>(pub f64, PhantomData<E>);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VarI32(pub i32);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VarU32(pub u32);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VarI64(pub i64);
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VarU64(pub u64);

impl Bool {
    pub fn new(val: bool) -> Self {
        Self(val)
    }
}

impl U8 {
    pub fn new(val: u8) -> Self {
        Self(val)
    }
}

impl I8 {
    pub fn new(val: i8) -> Self {
        Self(val)
    }
}

impl<E: ByteOrder> U16<E> {
    pub fn new(val: u16) -> Self {
        Self(val, PhantomData)
    }
}

impl<E: ByteOrder> I16<E> {
    pub fn new(val: i16) -> Self {
        Self(val, PhantomData)
    }
}

impl<E: ByteOrder> U24<E> {
    pub fn new(val: u32) -> Self {
        Self(val, PhantomData)
    }
}

impl<E: ByteOrder> I24<E> {
    pub fn new(val: i32) -> Self {
        Self(val, PhantomData)
    }
}

impl<E: ByteOrder> U32<E> {
    pub fn new(val: u32) -> Self {
        Self(val, PhantomData)
    }
}

impl<E: ByteOrder> I32<E> {
    pub fn new(val: i32) -> Self {
        Self(val, PhantomData)
    }
}

impl<E: ByteOrder> U64<E> {
    pub fn new(val: u64) -> Self {
        Self(val, PhantomData)
    }
}

impl<E: ByteOrder> I64<E> {
    pub fn new(val: i64) -> Self {
        Self(val, PhantomData)
    }
}

impl<E: ByteOrder> F32<E> {
    pub fn new(val: f32) -> Self {
        Self(val, PhantomData)
    }
}

impl<E: ByteOrder> F64<E> {
    pub fn new(val: f64) -> Self {
        Self(val, PhantomData)
    }
}

impl VarI32 {
    pub fn new(val: i32) -> Self {
        Self(val)
    }
}

impl VarU32 {
    pub fn new(val: u32) -> Self {
        Self(val)
    }
}

impl VarI64 {
    pub fn new(val: i64) -> Self {
        Self(val)
    }
}

impl VarU64 {
    pub fn new(val: u64) -> Self {
        Self(val)
    }
}

debug_impl!(U8);
debug_impl!(I8);
debug_impl!(Bool);
debug_impl!(U16<E: ByteOrder>);
debug_impl!(I16<E: ByteOrder>);
debug_impl!(U24<E: ByteOrder>);
debug_impl!(I24<E: ByteOrder>);
debug_impl!(U32<E: ByteOrder>);
debug_impl!(I32<E: ByteOrder>);
debug_impl!(U64<E: ByteOrder>);
debug_impl!(I64<E: ByteOrder>);
debug_impl!(F32<E: ByteOrder>);
debug_impl!(F64<E: ByteOrder>);
debug_impl!(VarU32);
debug_impl!(VarI32);
debug_impl!(VarU64);
debug_impl!(VarI64);

impl<'a> Binary<'a> for Bool {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_u8(self.0 as u8).unwrap()
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_u8()?;
        Ok(Self::new(n == 1))
    }
}

impl<'a> Binary<'a> for U8 {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_u8(self.0).unwrap()
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_u8()?;
        Ok(Self::new(n))
    }
}

impl<'a> Binary<'a> for I8 {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_i8(self.0).unwrap()
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_i8()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for U16<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_u16::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_u16::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for I16<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_i16::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_i16::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for U24<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_u24::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_u24::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for I24<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_i24::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_i24::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for U32<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_u32::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_u32::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for I32<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_i32::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_i32::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for U64<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_u64::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_u64::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for I64<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_i64::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_i64::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for F32<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_f32::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_f32::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a, E: ByteOrder> Binary<'a> for F64<E> {
    fn serialize(&self, buf: &mut impl Write) {
        buf.write_f64::<E>(self.0).unwrap();
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let n = buf.read_f64::<E>()?;
        Ok(Self::new(n))
    }
}

impl<'a> Binary<'a> for VarI32 {
    fn serialize(&self, buf: &mut impl Write) {
        let u = self.0;
        let mut ux = (self.0 as u32) << 1;

        if u < 0 {
            ux = !ux;
        }

        while ux >= 0x80 {
            U8::new(ux as u8 | 0x80).serialize(buf);
            ux >>= 7;
        }

        U8::new(ux as u8).serialize(buf);
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let mut ux = 0;

        for i in (0..35).step_by(7) {
            let b = U8::deserialize(buf)?.0;
            ux |= ((b & 0x7f) as i32) << i;

            if b & 0x80 == 0 {
                let mut x = (ux as i32) >> 1;
                if ux & 1 != 0 {
                    x = !x;
                }

                return Ok(VarI32::new(x));
            }
        }

        Err(Error::new(
            ErrorKind::Other,
            "VarI32 size must not exceed 5 bytes",
        ))
    }
}

impl<'a> Binary<'a> for VarU32 {
    fn serialize(&self, buf: &mut impl Write) {
        let mut u = self.0;

        while u >= 0x80 {
            U8::new(u as u8 | 0x80).serialize(buf);
            u >>= 7;
        }

        U8::new(u as u8).serialize(buf);
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let mut v = 0;

        for i in (0..35).step_by(7) {
            let b = U8::deserialize(buf)?.0;
            v |= ((b & 0x7f) as u32) << (i as u32);

            if b & 0x80 == 0 {
                return Ok(VarU32::new(v));
            }
        }

        Err(Error::new(
            ErrorKind::Other,
            "VarU32 size must not exceed 5 bytes",
        ))
    }
}

impl<'a> Binary<'a> for VarI64 {
    fn serialize(&self, buf: &mut impl Write) {
        let u = self.0;
        let mut ux = (self.0 as u64) << 1;

        if u < 0 {
            ux = !ux;
        }

        while ux >= 0x80 {
            U8::new(ux as u8 | 0x80).serialize(buf);
            ux >>= 7;
        }

        U8::new(ux as u8).serialize(buf);
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let mut ux = 0;

        for i in (0..70).step_by(7) {
            let b = U8::deserialize(buf)?.0;
            ux |= ((b & 0x7f) as i64) << (i as i64);

            if b & 0x80 == 0 {
                let mut x = (ux >> 1) as i64;
                if ux & 1 != 0 {
                    x = !x;
                }

                return Ok(VarI64::new(x));
            }
        }

        Err(Error::new(
            ErrorKind::Other,
            "VarI64 size must not exceed 10 bytes",
        ))
    }
}

impl<'a> Binary<'a> for VarU64 {
    fn serialize(&self, buf: &mut impl Write) {
        let mut u = self.0;

        while u >= 0x80 {
            U8::new(u as u8 | 0x80).serialize(buf);
            u >>= 7;
        }

        U8::new(u as u8).serialize(buf);
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let mut v = 0;

        for i in (0..70).step_by(7) {
            let b = U8::deserialize(buf)?.0;
            v |= ((b & 0x7f) as u64) << (i as u64);

            if b & 0x80 == 0 {
                return Ok(VarU64::new(v));
            }
        }

        Err(Error::new(
            ErrorKind::Other,
            "VarU64 size must not exceed 10 bytes",
        ))
    }
}

impl<'a, B: Binary<'a>> Binary<'a> for Option<B> {
    fn serialize(&self, buf: &mut impl Write) {
        match self {
            Some(val) => {
                Bool::new(true).serialize(buf);
                val.serialize(buf);
            }
            None => Bool::new(false).serialize(buf),
        }
    }

    fn deserialize(buf: &mut Cursor<&'a [u8]>) -> Result<Self> {
        let bool = Bool::deserialize(buf)?.0;

        match bool {
            true => {
                let val = B::deserialize(buf)?;
                Ok(Some(val))
            }
            false => Ok(None),
        }
    }
}
