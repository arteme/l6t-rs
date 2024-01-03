use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Format not supported")]
    FormatNotSupported(),

    #[error("IO error: {0}")]
    IO(#[from] io::Error)

}