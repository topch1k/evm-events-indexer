#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error(transparent)]
    ConfigErrors(#[from] twelf::Error),

    #[error(transparent)]
    ProviderErrors(#[from] ethers::providers::ProviderError),

    #[error(transparent)]
    TokioSendError(#[from] tokio::sync::mpsc::error::SendError<ethers::types::Log>),

    #[error(transparent)]
    EthAbiErrors(#[from] ethers::abi::Error),

    #[error(transparent)]
    SerdeJsonErrors(#[from] serde_json::Error),

    #[error(transparent)]
    R2D2Errors(#[from] r2d2::Error),
}

pub type IndexerResult<T> = Result<T, Errors>;
