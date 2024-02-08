use di::ServiceProvider;
use crate::command::{HandleCommand, SearchCommand};
use crate::prelude::git::GitRepository;
use crate::workflow::repository::WorkflowRepository;

impl HandleCommand for SearchCommand {
    fn handle(&self, services: &ServiceProvider) {
        let repo = services.get::<GitRepository>().unwrap();

        println!("Search Command queries for {}", self.query.clone());
        let query_result = repo.query_workflows(self.query.as_str());
        if let Ok(workflows) = query_result {
            for a in workflows {
                println!("---\n{}\n{:?}", a.command, a.description.unwrap_or("".to_string()));
            }
        }
    }
}