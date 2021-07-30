#[derive(thiserror::Error, Debug)]
pub enum ByteStyleError {
    #[error("string decode as utf-8 failed")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("parse failed: {0}")]
    Parse(String),
}

pub type ByteStyleResult<T> = Result<T, ByteStyleError>;
