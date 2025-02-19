use tonic::Status;

/// Alias for a `Result` with the error type [`celestia_tonic::Error`].
///
/// [`celestia_tonic::Error`]: crate::Error
pub type Result<T> = std::result::Result<T, Error>;

/// Representation of all the errors that can occur when interacting with [`celestia_tonic`].
///
/// [`celestia_tonic`]: crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Tonic error
    #[error(transparent)]
    TonicError(#[from] Status),

    /// Tendermint Error
    #[error(transparent)]
    TendermintError(#[from] tendermint::Error),

    /// Celestia types error
    #[error(transparent)]
    CelestiaTypesError(#[from] celestia_types::Error),

    /// Tendermint Proto Error
    #[error(transparent)]
    TendermintProtoError(#[from] tendermint_proto::Error),

    /// Failed to parse gRPC response
    #[error("Failed to parse response")]
    FailedToParseResponse,

    /// Unexpected reponse type
    #[error("Unexpected response type")]
    UnexpectedResponseType(String),

    /// Empty blob submission list
    #[error("Attempted to submit blob transaction with empty blob list")]
    TxEmptyBlobList,
}
