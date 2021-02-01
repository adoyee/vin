pub use de::{from_str, Deserializer};
use encoding::all::GB18030;
use encoding::{EncoderTrap, Encoding};
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};

mod de;
mod error;
mod ser;

pub struct GBKString {
    message: String,
}

impl GBKString {
    const LENGTH: usize = 17;
    pub fn string(&self) -> &String {
        &self.message
    }
    fn mut_string(&mut self) -> &mut String {
        &mut self.message
    }
}

impl ::serde::Serialize for GBKString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        let gbk = GB18030.encode(self.string().as_str(), EncoderTrap::Strict);
        if gbk.is_err() {
            return Err(::serde::ser::Error::custom("gbk encode"));
        }
        let mut gbk = gbk.unwrap();
        if gbk.len() > GBKString::LENGTH {
            return Err(::serde::ser::Error::custom("GB18030 too large"));
        }

        if gbk.is_empty() {
            return serializer.serialize_u8(0);
        }

        gbk.resize(GBKString::LENGTH, 0);
        serializer.serialize_bytes(gbk.as_slice())
    }
}

impl<'de> ::serde::Deserialize<'de> for GBKString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<GBKString, D::Error>
    where
        D: ::serde::de::Deserializer<'de> + Sized,
    {
        let mut buff = Vec::with_capacity(GBKString::LENGTH);
        buff.resize(GBKString::LENGTH, 0);
        let s = GBKString {
            message: String::from_utf8(buff).unwrap_or_default(),
        };
        Ok(s)
    }
}
//
// struct GBKBuff {
//     data: [u8],
// }
//
type BuffLen3 = [u8; 3];
type BuffLen4 = [u8; 4];

struct SString<T> {
    data: T,
}

type S3 = SString<BuffLen3>;
type S4 = SString<BuffLen4>;

impl<T> std::fmt::Display for SString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hello")
    }
}
#[test]
fn test_ss() {
    let s3: S3;
    // let s3: S3 = SString { data: [0, 1, 2] };
    let s4: S4 = SString { data: [0, 1, 2, 3] };
}

trait LengthTrait: Sized {
    const LENGTH: usize;
}

struct Length17 {}
impl LengthTrait for Length17 {
    const LENGTH: usize = 17;
}

struct StringBuffer<T> {
    message: String,
    _marker: std::marker::PhantomData<T>,
}
impl<T> StringBuffer<T>
where
    T: LengthTrait,
{
    const LENGTH: usize = T::LENGTH;
    pub fn new() -> Self {
        Self {
            message: String::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn length(self) -> usize {
        T::LENGTH
    }
}

impl<'de, L> ::serde::Deserialize<'de> for StringBuffer<L>
where
    L: LengthTrait,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<StringBuffer<L>, D::Error>
    where
        D: ::serde::de::Deserializer<'de> + Sized,
    {
        let s = StringBuffer::new();
        println!("{}", StringBuffer::<L>::LENGTH);
        Ok(s)
    }
}

type String17 = StringBuffer<Length17>;

struct TestStruct {
    s17: String17,
}

#[test]
fn test_length() {
    let s17: String17 = String17::new();
    println!("{}", s17.length())
}
