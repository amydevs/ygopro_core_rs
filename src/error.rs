use thiserror::Error;

#[derive(Error, Debug)]
pub enum DuelError {
    #[error("The Lua script `{0}` failed to load")]
    ScriptLoadFailure(String),
}
