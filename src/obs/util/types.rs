use std::error::Error;

pub type ResultType<T> = Result<T, Box<dyn Error>>;
