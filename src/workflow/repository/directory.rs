use std::path::PathBuf;
use crate::prelude::{WorkflowError, WorkflowResult};
use crate::workflow::file_format::Workflow;
use crate::workflow::repository::WorkflowRepository;

pub struct DirectoryRepository {
    root: PathBuf,
    workflows: Vec<Workflow>,
}

impl DirectoryRepository {
    pub fn new(root: PathBuf) -> Self {
        let mut workflows = Self::visit_dir(root.clone()).unwrap();
        DirectoryRepository {
            root,
            workflows,
        }
    }

    fn visit_dir(path_buf: PathBuf) -> WorkflowResult<Vec<Workflow>> {
        let mut workflows = vec![];
        let pathStr = path_buf.as_path().display().to_string();
        for entry in std::fs::read_dir(&path_buf).map_err(|e| WorkflowError::NotFound(pathStr.clone()))? {
            let entry = entry.map_err(|e| WorkflowError::NotFound(pathStr.clone()))?; // unwrap the result
            let path = entry.path();
            if path.is_file() {
                let file = std::fs::File::open(path).unwrap();
                let reader = std::io::BufReader::new(file);
                let workflow: Workflow = serde_yaml::from_reader(reader).unwrap();
                workflows.push(workflow);
            } else {
                let mut sub_workflows = Self::visit_dir(path)?;
                workflows.append(&mut sub_workflows);
            }
        }
        Ok(workflows)
    }
}

impl WorkflowRepository for DirectoryRepository {
    fn refresh(&mut self) -> WorkflowResult<()> {
        self.workflows = Self::visit_dir(self.root.clone())?;
        Ok(())
    }
    fn get_workflow(&self, name: impl Into<String>) -> WorkflowResult<Workflow> {
        // move name into owned String
        let name = name.into();
        let a = self.workflows.iter().find(|w| w.name == name);
        match a {
            Some(w) => Ok(w.clone()),
            None => Err(WorkflowError::NotFound(name))
        }
    }

    fn get_workflows(&self) -> WorkflowResult<Vec<Workflow>> {
        let a = self.workflows.iter().map(|w| w.clone()).collect();
        Ok(a)
    }

    fn save_workflow(& mut self, workflow: Workflow) -> WorkflowResult<()> {
        Ok(self.workflows.push(workflow.clone()))
    }

    fn delete_workflow(&mut self, name: &str) -> WorkflowResult<()> {
        Ok(self.workflows.retain(|w| w.name != name))
    }

    fn query_workflows(&self, query: &str) -> WorkflowResult<Vec<Workflow>> {
        let a:Vec<Workflow> = self.workflows.iter().filter(|w| w.name.contains(query)).map(|w| w.clone()).collect();
        if (a.len() > 0) {
            Ok(a)
        } else {
            Err(WorkflowError::NotFound(query.to_string()))
        }
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::workflow::repository::directory::DirectoryRepository;
    use crate::workflow::repository::WorkflowRepository;

    #[test]
    fn test_load_from_dir() {
        let repo = DirectoryRepository::new(PathBuf::from("tests/fixtures/workflows"));
        let workflows = repo.get_workflows();
        if let Ok(workflows) = workflows {
            assert_eq!(workflows.len(), 3);
        } else {
            panic!("Error loading workflows");
        }
    }

    #[test]
    fn test_adding_should_add_new_workflow() {
        let mut repo = DirectoryRepository::new(PathBuf::from("tests/fixtures/workflows"));

        if let Err(e) = repo.save_workflow(crate::workflow::file_format::Workflow::new("test", "echo test")) {
            panic!("Error saving workflow: {:?}", e);
        }

        let workflows = repo.get_workflows();
        if let Ok(workflows) = workflows {
            assert_eq!(workflows.len(), 4);
        } else {
            panic!("Error loading workflows");
        }
    }

    #[test]
    fn test_adding_and_delete_should_remove_workflow() {
        let mut repo = DirectoryRepository::new(PathBuf::from("tests/fixtures/workflows"));

        if let Err(e) = repo.save_workflow(crate::workflow::file_format::Workflow::new("test", "echo test")) {
            panic!("Error saving workflow: {:?}", e);
        }

        if let Err(e) = repo.delete_workflow("test") {
            panic!("Error saving workflow: {:?}", e);
        }

        let workflows = repo.get_workflows();

        if let Ok(workflows) = workflows {
            assert_eq!(workflows.len(), 3);
        } else {
            panic!("Error loading workflows");
        }

        repo.refresh();
    }
}