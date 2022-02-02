use std::{borrow::Cow, ops::Deref};

use serde_::de;

use crate::{
    crypt::CryptString,
    str_wraper::StrWraper,
    wrap::{Crypt, CryptWarp, Raw},
    Encoder,
};

impl<'de, E, C> serde_::Deserialize<'de> for CryptWarp<Raw, E, C>
where
    E: Encoder + Default,
    C: StrWraper,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(
            Raw,
            CryptString::new_raw(s).or_else(|e| Err(de::Error::custom(e)))?,
        ))
    }
}

impl<'de, E, C> serde_::Deserialize<'de> for CryptWarp<Crypt, E, C>
where
    E: Encoder + Default,
    C: StrWraper,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(Crypt, CryptString::new_crypt(s)))
    }
}

impl<E, C> serde_::Serialize for CryptWarp<Crypt, E, C>
where
    E: Encoder,
    C: StrWraper,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_::Serializer,
    {
        let crypt = match &self.1 {
            CryptString::Raw(r, _) => {
                E::encode(&r.into_ref()).or_else(|e| Err(serde_::ser::Error::custom(e)))?
            }
            CryptString::Crypt(r) => Cow::Borrowed(r.deref()),
        };

        crypt.serialize(serializer)
    }
}
