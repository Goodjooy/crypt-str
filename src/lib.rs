//! # Crypt String 方便扩展的密码加密字符串
//! 
//! ## features
//! - bcrypt 提供内置的 bcrypt Encoder
//! - none 提供内置不加密 Encoder
//! - serde 提供序列化与反序列化支持
//! - tarns 提供 `String` 与 `CryptString` 互相转换实现支持
//! - wrap 使得 `crate::CryptWrap` 可用
//!  
//! ---
//! 
//! ## 可更改的加密算法
//! CryptString 可通过 指定 `Encoder` 来指定加密算法
//! 
//! ---
//! 
//! ## 可更改的原文容器
//! 通过 指定 `StrWrapper` 为crypt String 更换默认原文容器
//! 默认为 `Cow<'static, str>`  
//! `StrWrapper` 自动为 `Cow<'s, str>`与 `String` 实现
//! 可以实现 其他容器用于存放原文 
//! 
//! ---
//! 
//! ## 指定情况的序列化与反序列化支持
//! 提供`crate::CryptWrap` 以 在序列化反序列化时特别指定转换模式

mod crypt;
mod encoders;
#[cfg(feature = "serde")]
mod serde;
mod str_wrapper;
#[cfg(feature = "trans")]
mod trans;
#[cfg(feature = "wrap")]
mod wrap;


pub use encoders::Encoder;
pub mod inner_encoders {
    #[cfg(feature = "bcrypt")]
    pub mod bcrypt {
        pub use crate::encoders::bcrypt::BcryptEncoder;
        pub type DefaultBcryptEncoder = BcryptEncoder<12>;
        pub type BcryptString = crate::CryptString<DefaultBcryptEncoder>;
    }

    #[cfg(feature = "none")]
    pub mod none {
        pub use crate::encoders::none::{NoCrypto, NoErr};
        pub type NoCryptString = crate::CryptString<NoCrypto>;
    }
}

pub use crate::str_wrapper::{NoError, StrWrapper};
pub use crypt::CryptString;
#[cfg(feature = "wrap")]
pub use wrap::{Crypt, CryptWarp, Raw};
