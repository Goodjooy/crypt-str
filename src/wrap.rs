use std::{borrow::Cow, ops::Deref};

use crate::{crypt::CryptString, str_wraper::StrWraper, Encoder};

pub(crate) mod private {
    pub trait CryptSrouce {}
}

/// 包装 `CryptoString` 的智能指针，用于提供原始数据来源类型（反序列化和Into时使用）
pub struct CryptWarp<Src, E, C = Cow<'static, str>>(pub(crate) Src, pub(crate) CryptString<E, C>);

impl<Src, E, C: StrWraper> Deref for CryptWarp<Src, E, C> {
    type Target = CryptString<E, C>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<Src, E, C> Into<CryptString<E, C>> for CryptWarp<Src, E, C>
where
    E: Encoder,
    Src: private::CryptSrouce,
    C: StrWraper,
{
    fn into(self) -> CryptString<E, C> {
        self.1
    }
}

pub struct Raw;

impl private::CryptSrouce for Raw {}

pub struct Crypt;
impl private::CryptSrouce for Crypt {}
