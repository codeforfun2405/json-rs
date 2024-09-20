use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("unsupported char: {0}")]
    UnsupportedChar(char),
    #[error("string is unterminated")]
    UnterminatedString,
    #[error("invalid json")]
    InvalidJson,
    #[error("unknown ident: {0}")]
    UnknowIdent(String),
    #[error("expect string")]
    ExpectedString,
    #[error("expect: {0}, but get: {1}")]
    ExpectToken(String, String),
}
