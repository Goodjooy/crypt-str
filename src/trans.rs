use std::borrow::Cow;

use crate::{str_wraper::StrWraper, Crypt, CryptString, CryptWarp, Encoder, Raw};

impl<E, C> TryFrom<String> for CryptWarp<Raw, E, C>
where
    E: Encoder + Default,
    C: StrWraper,
{
    type Error = C::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(CryptWarp(Raw, CryptString::new_raw(s)?))
    }
}

impl<E, C> From<String> for CryptWarp<Crypt, E, C>
where
    E: Encoder + Default,
    C: StrWraper,
{
    fn from(s: String) -> Self {
        CryptWarp(Crypt, CryptString::new_crypt(s))
    }
}

impl<'s, E, C> TryInto<Cow<'s, str>> for &'s CryptWarp<Crypt, E, C>
where
    E: Encoder,
    C: StrWraper,
{
    type Error = E::Error;

    fn try_into(self) -> Result<Cow<'s, str>, Self::Error> {
        match &self.1 {
            CryptString::Raw(r, _) => E::encode(&r.into_ref()),
            CryptString::Crypt(c) => Ok(Cow::Borrowed(&c)),
        }
    }
}
