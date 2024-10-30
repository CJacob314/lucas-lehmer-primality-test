use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrimeFinderError {
    #[error("num_bigint error interpreting {0} as an integer")]
    ParseError(u64),
}
