use crate::Encoder;

pub struct NoCrypto;

impl Encoder for NoCrypto {
    type Error = NoErr;

    fn encode<'s>(raw: &str) -> Result<std::borrow::Cow<'s, str>, Self::Error> {
        Ok(raw.to_owned().into())
    }

    fn verify<'s, S: AsRef<str>>(
        encrypted: &std::borrow::Cow<'s, str>,
        input: &S,
    ) -> Result<bool, Self::Error> {
        Ok(encrypted == input.as_ref())
    }
}

#[derive(Debug)]
pub struct NoErr;

impl std::error::Error for NoErr {}
impl std::fmt::Display for NoErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "No Error")
    }
}
