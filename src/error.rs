use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct HoundifyError {
    inner: Box<dyn Error>,  // TODO: display inner?
}

impl Display for HoundifyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An error occurred during the Houndify request")
    }
}

impl Error for HoundifyError {}

impl HoundifyError {
    pub fn new(inner: Box<dyn Error>) -> Self {
        HoundifyError { inner }
    }
}