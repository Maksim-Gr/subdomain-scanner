use::thiserror::Error;


#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: subcanner <example.com> ")]
    CliUsage,
}