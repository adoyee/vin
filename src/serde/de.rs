use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};
use encoding::all::GBK;
use encoding::{DecoderTrap, Encoding};
use serde::{de, Deserialize};

use crate::serde::error::{Error, Result};
use crate::serde::gbk;

pub fn from_str<'de, T: de::Deserialize<'de>>(s: &str) -> Result<T> {
    let buff = hex::decode(s).map_err(Error::from)?;
    from_bytes(buff.as_slice())
}

pub fn from_bytes<'de, T: Deserialize<'de>>(buff: &[u8]) -> Result<T> {
    let mut deserializer = Deserializer { reader: buff };
    T::deserialize(&mut deserializer)
}

pub struct Deserializer<R: Read> {
    reader: R,
}

impl<R: Read> Deserializer<R> {
    pub fn new(read: R) -> Self {
        Self { reader: read }
    }
    pub fn deserialize_u8(&mut self) -> Result<u8> {
        self.reader.read_u8().map_err(Error::from)
    }
    pub fn deserialize_u16(&mut self) -> Result<u16> {
        self.reader.read_u16::<BigEndian>().map_err(Error::from)
    }
    pub fn deserialize_u32(&mut self) -> Result<u32> {
        self.reader.read_u32::<BigEndian>().map_err(Error::from)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buff: Vec<u8> = Vec::with_capacity(len);
        buff.resize(len, 0);
        self.reader.read_exact(buff.as_mut_slice())?;
        Ok(buff)
    }

    fn read_until_string_end(&mut self) -> Result<Vec<u8>> {
        let mut ret = Vec::new();
        loop {
            let d = self.reader.read_u8().map_err(Error::from)?;
            if d == 0x00 {
                break;
            } else {
                ret.push(d)
            }
        }
        Ok(ret)
    }
}

macro_rules! deserialize_type {
    ($deserialize:ident => $visitor:ident) => {
        fn $deserialize<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
            let value = self.$deserialize()?;
            visitor.$visitor(value)
        }
    };
}

macro_rules! deserialize_unsupported {
    ($deserialize:ident ) => {
        fn $deserialize<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
            Err(Error::Unsupported)
        }
    };
}

macro_rules! deserialize_unsupported_3 {
    ($deserialize:ident ) => {
        fn $deserialize<V: de::Visitor<'de>>(
            self,
            _name: &'static str,
            _visitor: V,
        ) -> Result<V::Value> {
            Err(Error::Unsupported)
        }
    };
}

macro_rules! deserialize_unsupported_4 {
    ($deserialize:ident ) => {
        fn $deserialize<V: de::Visitor<'de>>(
            self,
            _name: &'static str,
            _len: usize,
            _visitor: V,
        ) -> Result<V::Value> {
            Err(Error::Unsupported)
        }
    };
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let buff = self.read_until_string_end()?;
        let gbk = GBK
            .decode(buff.as_slice(), DecoderTrap::Strict)
            .map_err(|_| Error::GBK)?;
        visitor.visit_string(gbk)
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let value: u8 = serde::de::Deserialize::deserialize(&mut *self)?;
        match value {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(&mut *self),
            _ => Err(Error::Unsupported),
        }
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let seq_access = SeqAccess::new(&mut *self);
        visitor.visit_seq(seq_access)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        struct Access<'a, R: 'a + Read> {
            deserializer: &'a mut Deserializer<R>,
            len: usize,
        }

        impl<'de, 'a, R: Read + 'a> de::SeqAccess<'de> for Access<'a, R> {
            type Error = Error;
            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
            where
                T: de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value = de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len,
        })
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _enum: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unsupported)
    }

    deserialize_type!(deserialize_u8 => visit_u8);
    deserialize_type!(deserialize_u16 => visit_u16);
    deserialize_type!(deserialize_u32 => visit_u32);

    deserialize_unsupported!(deserialize_any);
    deserialize_unsupported!(deserialize_i8);
    deserialize_unsupported!(deserialize_i16);
    deserialize_unsupported!(deserialize_i32);
    deserialize_unsupported!(deserialize_i64);
    deserialize_unsupported!(deserialize_u64);
    deserialize_unsupported!(deserialize_f32);
    deserialize_unsupported!(deserialize_f64);
    deserialize_unsupported!(deserialize_bool);
    deserialize_unsupported!(deserialize_char);
    deserialize_unsupported!(deserialize_unit);
    deserialize_unsupported!(deserialize_map);
    deserialize_unsupported!(deserialize_ignored_any);
    deserialize_unsupported!(deserialize_identifier);

    deserialize_unsupported_3!(deserialize_unit_struct);
    deserialize_unsupported_4!(deserialize_tuple_struct);
}

impl<'de, 'a, R> serde::de::VariantAccess<'de> for &'a mut Deserializer<R>
where
    R: Read,
{
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        de::DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}

struct SeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: 'a + Read> SeqAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Self {
        Self { de }
    }
}

impl<'a, 'de, R: 'a + Read> serde::de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;
    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de).map(Some)
    }
}
