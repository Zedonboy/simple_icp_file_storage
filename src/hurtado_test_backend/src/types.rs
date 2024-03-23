use std::{cell::RefCell, cmp, io::Write, ops::{Deref, DerefMut}, rc::Rc};

use candid::{types::TypeInner, CandidType};
use serde::{de::Visitor, Deserialize, Serialize};
use serde_bytes::{ByteBuf};

struct RcbytesVisitor;

impl<'de> Visitor<'de> for RcbytesVisitor {
    type Value = Rcbytes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a byte array")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Rcbytes(Rc::new(RefCell::new(ByteBuf::from(v)))))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Rcbytes(Rc::new(RefCell::new(ByteBuf::from(v)))))
    }

    

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {

        let len = cmp::min(seq.size_hint().unwrap_or(0), 4096);
        let mut bytes = Vec::with_capacity(len);

        while let Some(b) = seq.next_element()? {
            bytes.push(b)
        };

        Ok(Rcbytes(Rc::new(RefCell::new(ByteBuf::from(bytes)))))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(Rcbytes(Rc::new(RefCell::new(ByteBuf::from(v)))))
    }

   
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v.encode_utf8(&mut [0u8; 4]))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_bytes(v)
    }

}
pub struct Rcbytes(pub Rc<RefCell<ByteBuf>>);

impl Rcbytes {
    pub fn new(arc : Rc<RefCell<ByteBuf>>) -> Self {
        Rcbytes(arc)
    }
}

impl CandidType for Rcbytes {
    fn _ty() -> candid::types::Type {
        TypeInner::Vec(TypeInner::Nat8.into()).into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer {
       serializer.serialize_blob(&self.0.borrow())
    }
}

impl<'de> Deserialize<'de> for Rcbytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_bytes(RcbytesVisitor)
    }
}

impl Serialize for Rcbytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_bytes(&self.0.borrow())
    }
}

impl Clone for Rcbytes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
