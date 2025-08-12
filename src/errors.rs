#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error(transparent)]
    ConfigErrors(#[from] twelf::Error),

    #[error(transparent)]
    ProviderErrors(#[from] ethers::providers::ProviderError),

    #[error(transparent)]
    EthAbiErrors(#[from] ethers::abi::Error),

    #[error(transparent)]
    SerdeJsonErrors(#[from] serde_json::Error),

    #[error(transparent)]
    R2D2Errors(#[from] r2d2::Error),

    #[error(transparent)]
    DieselErrors(#[from] diesel::result::Error),

    #[error(transparent)]
    DieselMigrationsErrors(#[from] diesel_migrations::MigrationError),

    #[error("Running migrations error")]
    RunningMigrationErrors,
}

pub type IndexerResult<T> = Result<T, Errors>;
