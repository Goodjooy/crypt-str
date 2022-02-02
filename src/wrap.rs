use std::ops::Deref;

use crate::{crypt::CryptString, Encoder};

pub(crate) mod private {
    pub trait CryptSrouce {}
}

/// 包装 `CryptoString` 的智能指针，用于提供原始数据来源类型（反序列化和Into时使用）
pub struct CryptWarp<Src, E>(pub(crate) Src, pub(crate) CryptString<E>);

impl<Src, E> Deref for CryptWarp<Src, E> {
    type Target = CryptString<E>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<Src, E> Into<CryptString<E>> for CryptWarp<Src, E>
where
    E: Encoder,
    Src: private::CryptSrouce,
{
    fn into(self) -> CryptString<E> {
        self.1
    }
}

pub struct Raw;

impl private::CryptSrouce for Raw {}

pub struct Crypt;
impl private::CryptSrouce for Crypt {}
