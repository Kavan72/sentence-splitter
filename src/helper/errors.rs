use thiserror::Error;

#[derive(Error, Debug)]
pub enum SentenceSplitterError {
    #[error("Endpoint not available error: {0}")]
    FileDownloadError(String),

    #[error("IO error: {0}")]
    IOError(String),
}

impl From<cached_path::Error> for SentenceSplitterError {
    fn from(error: cached_path::Error) -> Self {
        SentenceSplitterError::FileDownloadError(error.to_string())
    }
}

impl From<std::io::Error> for SentenceSplitterError {
    fn from(error: std::io::Error) -> Self {
        SentenceSplitterError::IOError(error.to_string())
    }
}
