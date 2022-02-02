use std::borrow::Cow;

use crate::{Crypt, CryptString, CryptWarp, Encoder, Raw, str_wraper::NoError};

impl<E> TryFrom<String> for CryptWarp<Raw, E>
where
    E: Encoder + Default,
{
    fn try_from(s: String) -> Result<Self,NoError> {
        Ok(CryptWarp(Raw, CryptString::new_raw(s)?))
    }

    type Error=NoError;
}

impl<E> From<String> for CryptWarp<Crypt, E>
where
    E: Encoder + Default,
{
    fn from(s: String) -> Self {
        CryptWarp(Crypt, CryptString::new_crypt(s))
    }
}

impl<'s, E> TryInto<Cow<'s, str>> for &'s CryptWarp<Crypt, E>
where
    E: Encoder,
{
    type Error = E::Error;

    fn try_into(self) -> Result<Cow<'s, str>, Self::Error> {
        match &self.1 {
            CryptString::Raw(r, _) => E::encode(&r),
            CryptString::Crypt(c) => Ok(Cow::Borrowed(&c)),
        }
    }
}

