mod error;
mod storage_error;

pub use error::{Error as AppError, Result};
pub use storage_error::StorageError;