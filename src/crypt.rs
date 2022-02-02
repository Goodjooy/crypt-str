use std::{borrow::Cow, ops::Deref};

use crate::{str_wraper::StrWraper, Encoder};

#[derive(Debug, Clone)]
pub enum CryptString<E, C = Cow<'static, str>> {
    Raw(C, E),
    Crypt(Cow<'static, str>),
}

impl<E, C> CryptString<E, C>
where
    E: Encoder,
    C: StrWraper,
{
    pub fn new_raw<S>(raw: S) -> Result<Self, C::Error>
    where
        E: Default,
        S: Into<String>,
    {
        Ok(Self::Raw(C::from_owner(raw.into())?, E::default()))
    }

    pub fn new_crypt<S>(raw: S) -> Self
    where
        S: Into<String>,
    {
        Self::Crypt(Cow::Owned(raw.into()))
    }

    pub fn crypt(self) -> Result<Self, E::Error> {
        match self {
            CryptString::Raw(r, _) => E::encode(&r.into_ref()).and_then(|e| Ok(Self::Crypt(e))),
            c => Ok(c),
        }
    }

    pub fn verify(&self, rhs: &Self) -> std::result::Result<bool, E::Error> {
        match (self, rhs) {
            (Self::Raw(r, _), Self::Raw(r2, _)) => Ok(r.into_ref() == r2.into_ref()),
            (Self::Raw(r, _), Self::Crypt(c)) => E::verify(c, &r.into_ref()),
            (Self::Crypt(c), Self::Raw(r, _)) => E::verify(c, &r.into_ref()),
            (Self::Crypt(c1), Self::Crypt(c2)) => Ok(c1 == c2),
        }
    }
}

impl<E> AsRef<str> for CryptString<E> {
    fn as_ref(&self) -> &str {
        match self {
            CryptString::Raw(r, _) | CryptString::Crypt(r) => &r,
        }
    }
}

impl<E> Deref for CryptString<E> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

#[cfg(feature = "wrap")]
impl<E> CryptString<E> {
    pub fn into_crypt(self) -> crate::CryptWarp<crate::Crypt, E> {
        crate::CryptWarp(crate::Crypt, self)
    }
}
