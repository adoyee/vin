use std::fmt;
use std::marker::PhantomData;

use encoding::all::GB18030;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use serde::de::{SeqAccess, Visitor};
use serde::{Deserializer, Serializer};

pub trait Options {
    const LENGTH: usize;
}

#[derive(Debug)]
pub struct GBKString<O> {
    pub message: String,
    _marker: PhantomData<O>,
}

impl<O: Options> GBKString<O> {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            _marker: PhantomData,
        }
    }
    pub fn from(src: String) -> Self {
        Self {
            message: src,
            _marker: PhantomData,
        }
    }
}

impl<O> serde::Serialize for GBKString<O>
where
    O: Options,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buff = GB18030
            .encode(self.message.as_str(), EncoderTrap::Strict)
            .unwrap();
        buff.resize(O::LENGTH, 0);
        serializer.serialize_bytes(buff.as_slice())
    }
}

impl<'de, O> serde::Deserialize<'de> for GBKString<O>
where
    O: Options,
{
    fn deserialize<D>(deserializer: D) -> Result<GBKString<O>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = StringVisitor::new();
        deserializer.deserialize_seq(visitor)
    }
}

struct StringVisitor<O> {
    _marker: PhantomData<O>,
}

impl<O: Options> StringVisitor<O> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'de, O> Visitor<'de> for StringVisitor<O>
where
    O: Options,
{
    type Value = GBKString<O>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("gbk string visitor")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut buff: Vec<u8> = Vec::with_capacity(O::LENGTH);
        buff.resize(O::LENGTH, 0);
        for i in 0..O::LENGTH {
            buff[i] = seq
                .next_element()?
                .ok_or_else(|| ::serde::de::Error::custom("hello"))?;
        }

        let p = buff.iter().position(|&x| x == 0x00);
        if p.is_some() {
            let p = p.unwrap();
            buff.resize(p, 0)
        }

        let message = GB18030
            .decode(buff.as_slice(), DecoderTrap::Strict)
            .unwrap();
        Ok(GBKString::from(message))
    }
}
