use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct HoundifyError {
    inner: Box<dyn Error>,
}

impl Display for HoundifyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An error occurred during the Houndify request: {:#?}", self.inner)
    }
}

impl Error for HoundifyError {}

impl HoundifyError {
    pub fn new(inner: Box<dyn Error>) -> Self {
        HoundifyError { inner }
    }
}

#[derive(Debug)]
pub struct InvalidRequestInfoError<'a> {
    msg: &'a str,
}

impl<'a> InvalidRequestInfoError<'a> {
    pub fn new(msg: &'a str) -> Self {
        InvalidRequestInfoError { msg }
    }
}

impl<'a> Display for InvalidRequestInfoError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str(self.msg)
    }
}
