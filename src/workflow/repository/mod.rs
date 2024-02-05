use crate::prelude::WorkflowResult;
use crate::workflow::file_format::Workflow;

mod directory;
mod git;

pub trait WorkflowRepository {
    fn refresh(&mut self) -> WorkflowResult<()>;
    fn get_workflow(&self, name: impl Into<String>) -> WorkflowResult<Workflow>;
    fn get_workflows(&self) -> WorkflowResult<Vec<Workflow>>;
    fn save_workflow(&mut self, workflow: Workflow) -> WorkflowResult<()>;
    fn delete_workflow(& mut self, name: &str) -> WorkflowResult<()>;
    fn query_workflows(&self, query: &str) -> WorkflowResult<Vec<Workflow>>;
}