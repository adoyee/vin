use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};
use encoding::all::GBK;
use encoding::{EncoderTrap, Encoding};
use serde::ser::{self, Impossible, Serialize};

use crate::serde::gbk;

use super::error::{Error, Result};

pub fn to_string<T: Serialize>(input: &T) -> Result<String> {
    let buff = to_bytes(input)?;
    Ok(hex::encode_upper(buff.as_slice()))
}

pub fn to_bytes<T: Serialize>(input: &T) -> Result<Vec<u8>> {
    let mut buff = Vec::new();
    input.serialize(&mut Serializer { writer: &mut buff })?;
    Ok(buff)
}

pub struct Serializer<W: Write> {
    writer: W,
}

impl<W: Write> Serializer<W> {
    pub fn serialize_gbk_string<O: gbk::Options>(
        &mut self,
        message: &gbk::GBKString<O>,
    ) -> Result<()> {
        let mut buff = GBK
            .encode(message.message.as_str(), EncoderTrap::Strict)
            .map_err(|_| Error::GBK)?;

        if buff.len() > O::LENGTH {
            return Err(Error::GbkTooLarge);
        }

        buff.resize(O::LENGTH, 0);
        self.writer.write_all(buff.as_slice()).map_err(Error::from)
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_i8(self, _: i8) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_i16(self, _: i16) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_i32(self, _: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_i64(self, _: i64) -> Result<()> {
        Err(Error::Unsupported)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.writer.write_u8(v).map_err(Error::from)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.writer.write_u16::<BigEndian>(v).map_err(Error::from)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.writer.write_u32::<BigEndian>(v).map_err(Error::from)
    }

    fn serialize_u64(self, _: u64) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_f32(self, _: f32) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_f64(self, _: f64) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_char(self, _: char) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let data = GBK
            .encode(&v, EncoderTrap::Strict)
            .map_err(|_| Error::GBK)?;
        self.writer.write(&data)?;
        self.serialize_u8(0)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.writer.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::Unsupported)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, _: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::Unsupported)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::Unsupported)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Unsupported)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::Unsupported)
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}
