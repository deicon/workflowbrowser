#[derive(Debug)]
pub enum WorkflowError {
    ParseError,
    NotFound(String),
}

pub type WorkflowResult<T> = Result<T, WorkflowError>;

pub struct W<A>(A);

