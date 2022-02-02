use std::borrow::Cow;

/// `CryptString` 内部的str容器
pub trait StrWraper: Sized {
    /// 从容器中获取引用
    fn into_ref(&self) -> &str;
    /// 从引用获取本体
    fn from_owner(src: String) -> Self;
}

impl StrWraper for String {
    fn into_ref(&self) -> &str {
        self.as_str()
    }

    fn from_owner(src: String) -> Self {
        src
    }
}

impl<'s> StrWraper for Cow<'s, str> {
    fn into_ref(&self) -> &str {
        &*self
    }

    fn from_owner(src: String) -> Self {
        Cow::Owned(src)
    }
}

