use thiserror::Error as ThisError;

#[derive(Debug, Clone, ThisError)]
pub enum Error {
    #[error("TODO: Add message here")]
    LLMError,
}
