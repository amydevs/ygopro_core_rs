use thiserror::Error;

#[derive(Error, Debug)]
pub enum DuelError {
    #[error("{0}")]
    NulError(#[from] std::ffi::NulError),
    #[error("The Lua script `{0}` failed to load")]
    ScriptLoadFailure(String),
}
