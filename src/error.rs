use::thiserror::Error;


#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: subcanner <example.com> ")]
    CliUsage,
    #[error("Reqwest: {}")]
    Reqwest(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}