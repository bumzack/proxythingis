use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    // #[error("error creating table: {0}")]
    // DBInitError(tokio_postgres::Error),
    // #[error("error reading file: {0}")]
    // ReadFileError(#[from] std::io::Error),
}

// TODO: hihihii
#[derive(Debug)]
pub struct DivideByZero;
