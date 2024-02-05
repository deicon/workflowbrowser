pub enum WorkflowError {
    ParseError
}

pub type WorkflowResult<T> = Result<T, WorkflowError>;

pub struct W<A>(A);

