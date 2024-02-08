use std::error::Error;
pub use crate::workflow::repository;
pub use crate::workflow::repository::directory;
pub use crate::workflow::repository::git;

#[derive(Debug)]
pub enum WorkflowError {
    NotFound(String),
    IoError(String),
}

impl From<std::io::Error> for WorkflowError {
    fn from(value: std::io::Error) -> Self {
        WorkflowError::IoError(format!("{:?}", value))
    }
}

pub type WorkflowResult<T> = Result<T, WorkflowError>;
#[allow(dead_code)]
pub struct W<A>(A);

