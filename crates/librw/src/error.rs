use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("a librw engine is already active in this process")]
    EngineAlreadyActive,
    #[error("librw failed during {0}")]
    Engine(&'static str),
    #[error("this librw backend requires platform-specific EngineOpenParams")]
    PlatformParamsRequired,
    #[error("librw returned a null pointer while {0}")]
    Null(&'static str),
    #[error("the string contains an interior NUL byte")]
    // TODO @joey ditto lib.rs comment on no_std
    Nul(#[from] std::ffi::NulError),
    #[error("the path is not valid UTF-8")]
    NonUtf8Path,
    #[error("the value does not fit in librw's 32-bit length field")]
    LengthOverflow,
    #[error("librw did not find chunk 0x{0:08x}")]
    ChunkNotFound(u32),
    #[error("index {index} is outside 0..{len}")]
    IndexOutOfBounds { index: usize, len: usize },
}
