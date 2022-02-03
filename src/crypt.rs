use std::{borrow::Cow, ops::Deref};

use crate::{str_wrapper::StrWrapper, Encoder};

#[derive(Debug, Clone)]
pub enum CryptString<E, C = Cow<'static, str>> {
    Raw(C, E),
    Crypt(Cow<'static, str>),
}

impl<E, C> CryptString<E, C>
where
    E: Encoder,
    C: StrWrapper,
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

impl<E, C: StrWrapper> AsRef<str> for CryptString<E, C> {
    fn as_ref(&self) -> &str {
        match self {
            CryptString::Raw(r, _) => r.into_ref(),
            CryptString::Crypt(r) => &r,
        }
    }
}

impl<E, C: StrWrapper> Deref for CryptString<E, C> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

#[cfg(feature = "wrap")]
impl<E, C: StrWrapper> CryptString<E, C> {
    pub fn into_crypt(self) -> crate::CryptWarp<crate::Crypt, E, C> {
        crate::CryptWarp(crate::Crypt, self)
    }
}
