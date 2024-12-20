use lumina_node::NodeError;
use thiserror::Error;

/// Result type alias for LuminaNode operations that can fail with a LuminaError
pub type Result<T> = std::result::Result<T, LuminaError>;

/// Represents all possible errors that can occur in the LuminaNode.
#[derive(Error, Debug, uniffi::Error)]
pub enum LuminaError {
    /// Error returned when trying to perform operations on a node that isn't running
    #[error("Node is not running")]
    NodeNotRunning,

    /// Error returned when network operations fail
    #[error("Network error: {msg}")]
    NetworkError {
        /// Description of the network error
        msg: String,
    },

    /// Error returned when storage operations fail
    #[error("Storage error: {msg}")]
    StorageError {
        /// Description of the storage error
        msg: String,
    },

    /// Error returned when trying to start a node that's already running
    #[error("Node is already running")]
    AlreadyRunning,

    /// Error returned when a mutex lock operation fails
    #[error("Lock error")]
    LockError,

    /// Error returned when a hash string is invalid or malformed
    #[error("Invalid hash format: {msg}")]
    InvalidHash {
        /// Description of why the hash is invalid
        msg: String,
    },

    /// Error returned when a header is invalid or malformed
    #[error("Invalid header format: {msg}")]
    InvalidHeader {
        /// Description of why the header is invalid
        msg: String,
    },

    /// Error returned when storage initialization fails
    #[error("Storage initialization failed: {msg}")]
    StorageInit {
        /// Description of why storage initialization failed
        msg: String,
    },
}

impl From<NodeError> for LuminaError {
    fn from(error: NodeError) -> Self {
        LuminaError::NetworkError {
            msg: error.to_string(),
        }
    }
}

impl From<libp2p::multiaddr::Error> for LuminaError {
    fn from(e: libp2p::multiaddr::Error) -> Self {
        LuminaError::NetworkError {
            msg: format!("Invalid multiaddr: {}", e),
        }
    }
}
