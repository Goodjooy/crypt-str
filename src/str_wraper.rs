use std::borrow::Cow;

#[derive(Debug)]
pub struct NoError;

impl std::fmt::Display for NoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NoError")
    }
}

impl std::error::Error for NoError {}

/// `CryptString` 内部的str容器
pub trait StrWraper: Sized {
    /// 当包装时，可能会产生错误，此处为错误关联类型
    type Error: std::error::Error;
    /// 从容器中获取引用
    fn into_ref(&self) -> &str;
    /// 从引用获取本体，错误时 返回`Err(Self::Error)`
    fn from_owner(src: String) -> Result<Self, Self::Error>;
}

impl StrWraper for String {
    fn into_ref(&self) -> &str {
        self.as_str()
    }

    fn from_owner(src: String) -> Result<Self, Self::Error> {
        Ok(src)
    }

    type Error = NoError;
}

impl<'s> StrWraper for Cow<'s, str> {
    fn into_ref(&self) -> &str {
        &*self
    }

    fn from_owner(src: String) -> Result<Self, Self::Error> {
        Ok(Cow::Owned(src))
    }

    type Error = NoError;
}

impl<'s> StrWraper for Box<String> {
    type Error = NoError;

    fn into_ref(&self) -> &str {
        &self
    }

    fn from_owner(src: String) -> Result<Self, Self::Error> {
        Ok(Box::new(src))
    }
}
